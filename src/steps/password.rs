use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::PromptStepType;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Validation {
    #[serde(rename = "not-empty")]
    NotEmpty { message: String },
    #[serde(rename = "min-length")]
    MinLength { length: usize, message: String },
}

/// Handle Password prompt step
pub fn handle_password(
    step_type: &PromptStepType,
    context: &mut HashMap<String, String>,
    last_input: &mut Option<String>
) -> Result<()> {
    if let PromptStepType::Password { prompt, mask, validate, output, confirm_password, confirm_prompt } = step_type {
        // First password prompt
        let mut password_builder = cliclack::password(prompt);
        
        if let Some(mask_char) = mask {
            password_builder = password_builder.mask(*mask_char);
        }
        
        if let Some(validations) = validate {
            // Clone the validations to ensure ownership in the closure
            let validations_owned = validations.clone();
            password_builder = password_builder.validate(move |input: &String| {
                for validation in &validations_owned {
                    match validation {
                        Validation::NotEmpty { message } => {
                            if input.is_empty() {
                                return Err(message.clone());
                            }
                        },
                        Validation::MinLength { length, message } => {
                            if input.len() < *length {
                                return Err(message.clone());
                            }
                        }
                    }
                }
                Ok(())
            });
        }
        
        let password_value: String = password_builder.interact()?;
        
        // Handle confirm password if enabled
        if confirm_password.unwrap_or(false) {
            // Use custom prompt or default
            let confirm_text = confirm_prompt.as_deref().unwrap_or("Confirm your password");
            let mut confirm_builder = cliclack::password(confirm_text);
            
            if let Some(mask_char) = mask {
                confirm_builder = confirm_builder.mask(*mask_char);
            }
            
            // Add validation to ensure passwords match
            let first_password = password_value.clone();
            confirm_builder = confirm_builder.validate(move |input: &String| {
                if input != &first_password {
                    Err("Passwords do not match".to_string())
                } else {
                    Ok(())
                }
            });
            
            let _confirm_value: String = confirm_builder.interact()?;
        }
        
        *last_input = Some(password_value.clone());
        
        // Store output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), password_value);
        }
    }
    Ok(())
}
