use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::{PromptStepType, replace_variables, render_prompt_interaction_with_vars};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SelectItem {
    pub value: String,
    pub label: String,
    pub hint: Option<String>,
    pub execute_yaml: Option<String>, // YAML file to execute if this item is selected
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum SelectItems {
    Static(Vec<SelectItem>),
    Variable(String),
}

/// Handle Select prompt step (single choice)
pub fn handle_select(
    step_type: &PromptStepType, 
    context: &mut HashMap<String, String>,
    last_input: &mut Option<String>
) -> Result<()> {
    if let PromptStepType::Select { prompt, items, output, initial } = step_type {
        let mut select_builder = cliclack::select(prompt);
        
        // Determine which items to use
        let resolved_items = match items {
            SelectItems::Static(static_items) => static_items.clone(),
            SelectItems::Variable(variable_ref) => {
                // Replace variables in the variable reference string
                let resolved_var = replace_variables(variable_ref, context, last_input);
                
                // The resolved_var should now contain the actual JSON data
                // First try to parse as an array of SelectItem objects
                if let Ok(items) = serde_json::from_str::<Vec<SelectItem>>(&resolved_var) {
                    items
                } else if let Ok(config_map) = serde_json::from_str::<HashMap<String, String>>(&resolved_var) {
                    // Fallback to HashMap for backward compatibility
                    config_map.into_iter().map(|(key, value)| {
                        let hint = if value.len() > 50 { 
                            format!("{}...", &value[..47]) 
                        } else { 
                            value 
                        };
                        SelectItem {
                            value: key.clone(),
                            label: key,
                            hint: Some(hint),
                            execute_yaml: None,
                        }
                    }).collect()
                } else {
                    Vec::new()
                }
            }
        };
        
        // Add items to select builder
        if resolved_items.is_empty() {
            return Err(anyhow::anyhow!("No items available for select"));
        }
        
        for item in &resolved_items {
            let hint = item.hint.as_deref().unwrap_or("");
            select_builder = select_builder.item(&item.value, &item.label, hint);
        }
        
        // Set initial selection if specified
        let resolved_initial_opt = initial.as_ref().map(|iv| replace_variables(iv, context, last_input));
        if let Some(ref resolved_initial) = resolved_initial_opt {
            select_builder = select_builder.initial_value(resolved_initial);
        }
        
        let selected_value: String = select_builder.interact()?.to_string();
        
        // Check if the selected item has conditional YAML execution
        let mut final_output_value = selected_value.clone();
        if let Some(selected_item) = resolved_items.iter().find(|item| item.value == selected_value) {
            if let Some(ref yaml_file) = selected_item.execute_yaml {
                // Convert current context to variables for the YAML execution
                let variables: Vec<(&str, String)> = context.iter()
                    .map(|(k, v)| (k.as_str(), v.clone()))
                    .collect();
                
                // Execute the conditional YAML file
                let result = render_prompt_interaction_with_vars(yaml_file, variables)?;
                
                // Merge the results back into the context
                for (key, value) in &result {
                    context.insert(key.clone(), value.clone());
                }
                
                // If the executed YAML has an "external_step_output" or similar output,
                // use that as the final output value instead of the selected value
                if let Some(external_output) = result.get("external_step_output") {
                    final_output_value = external_output.clone();
                } else {
                    // If no specific external output, serialize the entire result as JSON
                    // This allows access to all outputs from the executed YAML
                    final_output_value = serde_json::to_string(&result).unwrap_or(selected_value);
                }
            }
        }
        
        // Store final output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), final_output_value.clone());
            *last_input = Some(final_output_value);
        }
    }
    Ok(())
}
