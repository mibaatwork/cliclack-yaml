use std::collections::HashMap;
use anyhow::Result;
use crate::PromptStepType;

/// Handle Confirm prompt step
pub fn handle_confirm(step_type: &PromptStepType, context: &mut HashMap<String, String>) -> Result<bool> {
    if let PromptStepType::Confirm { prompt, output, default } = step_type {
        let mut confirm_builder = cliclack::confirm(prompt);
        
        // Set default value if specified
        if let Some(default_value) = default {
            confirm_builder = confirm_builder.initial_value(*default_value);
        }
        
        let response = confirm_builder.interact()?;
        
        // Store output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), response.to_string());
        }
        
        return Ok(response);
    }
    Ok(false)
}
