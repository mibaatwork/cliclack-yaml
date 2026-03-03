use std::collections::HashMap;
use anyhow::Result;
use crate::{PromptStepType, execute_function_by_name};

/// Handle Spinner prompt step
pub fn handle_spinner(step_type: &PromptStepType, context: &mut HashMap<String, String>) -> Result<bool> {
    if let PromptStepType::Spinner { start_text, stop_text, run_fn, output } = step_type {
        let spinner = cliclack::spinner();
        spinner.start(start_text);
        
        // Execute the function by name and get output
        let fn_output = execute_function_by_name(run_fn)?;
        
        spinner.stop(stop_text);
        
        // Store function output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), fn_output);
        }
    }
    Ok(true)
}
