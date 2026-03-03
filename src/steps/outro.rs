use std::collections::HashMap;
use anyhow::Result;
use crate::{PromptStepType, replace_variables, apply_text_style};

/// Handle Outro prompt step
pub fn handle_outro(
    step_type: &PromptStepType, 
    context: &mut HashMap<String, String>,
    last_input: &Option<String>
) -> Result<()> {
    if let PromptStepType::Outro { text, input, output, style } = step_type {
        // Get the referenced input value if specified
        // But don't use it for replacing {input} directly
        // We just add it to the last_input for context
        let mut local_last_input = last_input.clone();
        
        if let Some(input_key) = input {
            if let Some(value) = context.get(input_key) {
                local_last_input = Some(value.clone());
            }
        }
        
        // Process text with variable replacement
        let processed_text = replace_variables(text, context, &local_last_input);
        let styled_text = apply_text_style(&processed_text, style);
        
        cliclack::outro(styled_text.clone())?;
        println!("");
        
        // Store output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), processed_text);
        }
    }
    Ok(())
}
