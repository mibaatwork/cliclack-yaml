pub mod test_utils;

use std::{
    thread, 
    time::Duration, 
    collections::HashMap, 
    path::{Path, PathBuf}, 
    fs,
    env,
    cell::RefCell,
};

use anyhow::{Result, Context};
use console::Style;
use serde::{Deserialize, Serialize};
use serde_saphyr;

mod steps;
use steps::*;

// Constants
thread_local! {
    static APP_ROOT: RefCell<Option<PathBuf>> = RefCell::new(None);
    #[cfg_attr(test, allow(unused))]
    pub static PROMPT_FOLDER: RefCell<Option<PathBuf>> = RefCell::new(None);
}

//-----------------------------------------------------------------------------
// Data Types
//-----------------------------------------------------------------------------

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum PromptStepType {
    #[serde(rename = "clearscreen")]
    ClearScreen,
    
    #[serde(rename = "intro")]
    Intro { 
        text: String, 
        style: Option<TextStyle>,
    },
    
    #[serde(rename = "confirm")]
    Confirm { 
        prompt: String,
        output: Option<String>,
        default: Option<bool>,
    },
    
    #[serde(rename = "input")]
    Input { 
        prompt: String, 
        placeholder: Option<String>,
        validate: Option<Vec<input::Validation>>,
        output: Option<String>,
    },
    
    #[serde(rename = "spinner")]
    Spinner { 
        start_text: String,
        stop_text: String,
        run_fn: String,
        output: Option<String>,
    },
    
    #[serde(rename = "print")]
    Print {
        text: String,
        #[serde(default)]
        loglevel: Option<String>,
        input: Option<String>,
    },
    
    #[serde(rename = "outro")]
    Outro { 
        text: String,
        input: Option<String>,
        output: Option<String>,
        style: Option<TextStyle>,
    },
    
    #[serde(rename = "multiselect")]
    MultiSelect {
        prompt: String,
        items: multi_select::MultiSelectItems,
        output: Option<String>,
        required: Option<bool>,
    },
    
    #[serde(rename = "select")]
    Select {
        prompt: String,
        items: select::SelectItems,
        output: Option<String>,
        initial: Option<String>,
    },
    
    #[serde(rename = "password")]
    Password {
        prompt: String,
        mask: Option<char>,
        validate: Option<Vec<password::Validation>>,
        output: Option<String>,
        confirm_password: Option<bool>,
        confirm_prompt: Option<String>,
    },
    
    #[serde(rename = "progress")]
    Progress {
        progress_type: progress::ProgressType,
        start_message: String,
        stop_message: String,
        items: Option<Vec<progress::ProgressItem>>,
        output: Option<String>,
    },
    
    #[serde(rename = "multi-progress")]
    MultiProgress {
        title: String,
        progress_bars: Vec<progress::ProgressBarConfig>,
        output: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Condition {
    pub parent: String,
    pub value: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptStep {
    #[serde(flatten)]
    pub step_type: PromptStepType,
    #[serde(skip)]
    pub output: Option<String>,
    #[serde(rename = "step_name")]
    pub step_name: Option<String>,
    #[serde(rename = "condition")]
    pub condition: Option<Condition>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OutputValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextStyle {
    pub color: Option<String>,
    pub background: Option<String>,
}

//-----------------------------------------------------------------------------
// Public API Functions
//-----------------------------------------------------------------------------

/// Creates and handles the interactive CLI prompts using the cliclack library.
pub fn render_prompt_interaction(prompt_name: &str) -> Result<HashMap<String, String>> {
    create_prompt_with_vars(prompt_name, None)
}

/// Simplified version that accepts a vector of key-value pairs to make it easier to use
pub fn render_prompt_interaction_with_vars(
    prompt_name: &str,
    vars: Vec<(&str, impl ToString)>
) -> Result<HashMap<String, String>> {
    let mut variables = HashMap::new();
    for (key, value) in vars {
        variables.insert(key.to_string(), value.to_string());
    }
    create_prompt_with_vars(prompt_name, Some(variables))
}

/// Set the folder of the app root
pub fn set_app_root<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref().to_path_buf();
    APP_ROOT.with(|r| {
        if r.borrow().is_none() {
            *r.borrow_mut() = Some(path);
            Ok(())
        } else {
            Err(anyhow::anyhow!("App root already set"))
        }
    })
}

/// Set the folder path where YAML prompt files are located
/// This path is relative to the app root if it doesn't start with "/" or contain ":"
pub fn set_prompt_folder<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_ref = path.as_ref();
    
    // Determine if this is an absolute path or relative to app root
    let full_path = if path_ref.is_absolute() || path_ref.to_string_lossy().contains(":") {
        // Use as is if it's absolute
        path_ref.to_path_buf()
    } else {
        // Otherwise, it's relative to app root or current directory
        if let Some(app_root) = get_app_root() {
            app_root.join(path_ref)
        } else {
            // Fallback to current working directory if app root not set
            match env::current_dir() {
                Ok(current_dir) => current_dir.join(path_ref),
                Err(_) => path_ref.to_path_buf()
            }
        }
    };
    
    PROMPT_FOLDER.with(|r| {
        if r.borrow().is_none() {
            *r.borrow_mut() = Some(full_path);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Prompt folder already set"))
        }
    })
}


/// Prepare external variables for use with prompts.
/// Returns the variables as a HashMap that can be passed to create_prompt_interaction_with_vars.
pub fn inject_external_variables(variables: Option<HashMap<String, String>>) -> HashMap<String, String> {
    variables.unwrap_or_else(HashMap::new)
}

/// Get the currently set app root path
pub fn get_app_root() -> Option<PathBuf> {
    APP_ROOT.with(|r| r.borrow().clone())
}

/// Get the currently set prompt folder path
pub fn get_prompt_folder() -> Option<PathBuf> {
    PROMPT_FOLDER.with(|r| r.borrow().clone())
}

//-----------------------------------------------------------------------------
// Core Implementation
//-----------------------------------------------------------------------------

/// Creates and handles the interactive CLI prompts with external variables.
pub fn create_prompt_with_vars(
    prompt_name: &str,
    external_vars: Option<HashMap<String, String>>
) -> Result<HashMap<String, String>> {
    // Get the YAML content based on the prompt name
    let yaml_str = get_yaml_content(prompt_name)
        .with_context(|| format!("Failed to get YAML configuration for prompt: {}", prompt_name))?;
    
    // Parse YAML into a vector of prompt steps
    let steps: Vec<PromptStep> = serde_saphyr::from_str(&yaml_str)
        .with_context(|| format!("Failed to parse YAML configuration for prompt: {}", prompt_name))?;
    
    create_prompt_with_vars_custom(steps, external_vars)
}

/// Creates and handles the interactive CLI prompts with external variables using provided steps.
pub fn create_prompt_with_vars_custom(
    steps: Vec<PromptStep>,
    external_vars: Option<HashMap<String, String>>
) -> Result<HashMap<String, String>> {
    // Store results from steps that produce output
    let mut context: HashMap<String, String> = HashMap::new();
    
    // Store step results by name for conditional execution
    let mut step_results: HashMap<String, bool> = HashMap::new();
    
    // Add external variables to the context if provided
    if let Some(vars) = external_vars {
        context.extend(vars);
    }
    
    let mut last_input: Option<String> = None;
    
    // Process each step in the YAML file
    for step in steps {
        if !should_execute_step(&step, &step_results) {
            continue;
        }
        
        // Execute the step and store the result
        let result = execute_prompt_step(&step, &mut context, &mut last_input)?;
        
        // Store the result if the step has a name
        if let Some(step_name) = step.step_name {
            step_results.insert(step_name, result);
        }
    }
    
    Ok(context)
}

/// Handle Output values
pub fn handle_output(values: &Vec<OutputValue>, output: &Option<String>, context: &mut HashMap<String, String>) -> Result<()> {
    let mut collected_values = Vec::new();
    
    for output_value in values {
        let value_name = &output_value.name;
        let value_content = match context.get(&output_value.value) {
            Some(v) => v.clone(),
            None => output_value.value.clone(),
        };
        
        context.insert(value_name.clone(), value_content.clone());
        collected_values.push(value_content);
    }
    
    // If there's an output field, store all collected values as a comma-separated string
    if let Some(output_key) = output {
        let combined = collected_values.join(",");
        context.insert(output_key.clone(), combined);
    }
    
    Ok(())
}

/// Apply text styling based on the provided style configuration
pub fn apply_text_style(text: &str, style_opt: &Option<TextStyle>) -> String {
    let mut styled = String::from(text);
    
    if let Some(text_style) = style_opt {
        // Create a new style object
        let mut style = Style::new();
        
        if let Some(bg) = &text_style.background {
            match bg.as_str() {
                "cyan" => { style = style.on_cyan(); },
                "green" => { style = style.on_green(); },
                "red" => { style = style.on_red(); },
                "yellow" => { style = style.on_yellow(); },
                _ => {}
            }
        }
        
        if let Some(color) = &text_style.color {
            match color.as_str() {
                "black" => { style = style.black(); },
                "white" => { style = style.white(); },
                "red" => { style = style.red(); },
                "green" => { style = style.green(); },
                "blue" => { style = style.blue(); },
                _ => {}
            }
        }
        
        styled = style.apply_to(text).to_string();
    }
    
    styled
}

//-----------------------------------------------------------------------------
// Helper Functions
//-----------------------------------------------------------------------------

/// Determines if a step should be executed based on its condition
fn should_execute_step(step: &PromptStep, step_results: &HashMap<String, bool>) -> bool {
    if let Some(condition) = &step.condition {
        if let Some(parent_result) = step_results.get(&condition.parent) {
            // Skip this step if the condition doesn't match
            return *parent_result == condition.value;
        } else {
            // Skip if parent step not found
            return false;
        }
    }
    
    // No condition means always execute
    true
}

/// Executes an individual prompt step
fn execute_prompt_step(
    step: &PromptStep,
    context: &mut HashMap<String, String>,
    last_input: &mut Option<String>
) -> Result<bool> {
    match &step.step_type {
        PromptStepType::ClearScreen => {
            clear::handle_clear_screen(&step.step_type, context)?;
            Ok(true)
        },
        PromptStepType::Intro { .. } => {
            intro::handle_intro(&step.step_type, context)?;
            Ok(true)
        },
        PromptStepType::Confirm { .. } => {
            confirm::handle_confirm(&step.step_type, context)
        },
        PromptStepType::Input { .. } => {
            input::handle_input(&step.step_type, context, last_input)?;
            Ok(true)
        },
        PromptStepType::Spinner { .. } => {
            spinner::handle_spinner(&step.step_type, context)
        },
        PromptStepType::Print { .. } => {
            print::handle_print(&step.step_type, context)?;
            Ok(true)
        },
        PromptStepType::Outro { .. } => {
            outro::handle_outro(&step.step_type, context, last_input)?;
            Ok(true)
        },
        PromptStepType::MultiSelect { .. } => {
            multi_select::handle_multi_select(&step.step_type, context, last_input)?;
            Ok(true)
        },
        PromptStepType::Select { .. } => {
            select::handle_select(&step.step_type, context, last_input)?;
            Ok(true)
        },
        PromptStepType::Password { .. } => {
            password::handle_password(&step.step_type, context, last_input)?;
            Ok(true)
        },
        PromptStepType::Progress { .. } => {
            progress::handle_progress(&step.step_type, context)?;
            Ok(true)
        },
        PromptStepType::MultiProgress { .. } => {
            progress::handle_multi_progress(&step.step_type, context)?;
            Ok(true)
        },
    }
}

/// Replace variables in text with values from context
pub fn replace_variables(
    text: &str, 
    context: &HashMap<String, String>,
    last_input: &Option<String>
) -> String {
    let mut result = text.to_string();
    
    // Replace {input} with the provided input value (from input field or last_input)
    if let Some(input) = last_input {
        result = result.replace("{input}", input);
    }
    
    // Replace variables from context
    for (key, value) in context {
        result = result.replace(&format!("{{{}}}", key), value);
    }
    
    result
}

/// Get YAML content from the prompt file
pub fn get_yaml_content(prompt_name: &str) -> Result<String> {
    // Try to load from the specified external folder if set
    if let Some(prompt_folder) = PROMPT_FOLDER.with(|r| r.borrow().clone()) {
        let yaml_path = prompt_folder.join(format!("{}.yaml", prompt_name));
        if yaml_path.exists() {
            return fs::read_to_string(&yaml_path)
                .with_context(|| format!("Failed to read YAML file: {}", yaml_path.display()));
        }
    }
    
    // If no external file was found
    if let Some(prompt_folder) = PROMPT_FOLDER.with(|r| r.borrow().clone()) {
        Err(anyhow::anyhow!(
            "Prompt file '{}' not found in configured prompt folder: {}", 
            prompt_name,
            prompt_folder.display()
        ))
    } else {
        Err(anyhow::anyhow!(
            "Prompt file '{}' not found and no external prompt folder is configured", 
            prompt_name
        ))
    }
}

/// Implementation of a 5-second wait function that can be called by name
fn wait_five_seconds() {
    thread::sleep(Duration::from_secs(5));
}

/// Create a function dispatch mechanism to call functions by string name
fn execute_function_by_name(name: &str) -> Result<String> {
    match name {
        "wait_five_seconds" => {
            wait_five_seconds();
            Ok("Task completed successfully".to_string())
        },
        _ => Err(anyhow::anyhow!("Function not found: {}", name))
    }
}

// Re-export public types for external use
pub use steps::multi_select::MultiSelectItem;
pub use steps::multi_select::MultiSelectItems;
pub use steps::select::SelectItem;
pub use steps::select::SelectItems;
pub use steps::input::Validation;
