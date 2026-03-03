use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::PromptStepType;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Validation {
    #[serde(rename = "not-empty")]
    NotEmpty { message: String },
    #[serde(rename = "regex")]
    Regex { pattern: String, message: String },
    #[serde(rename = "must-start-with-letter")]
    MustStartWithLetter { message: String },
}

/// Handle Input prompt step
pub fn handle_input(
    step_type: &PromptStepType, 
    context: &mut HashMap<String, String>,
    last_input: &mut Option<String>
) -> Result<()> {
    if let PromptStepType::Input { prompt, placeholder, validate, output } = step_type {
        let mut input_builder = cliclack::input(prompt);
        
        if let Some(ph) = placeholder {
            input_builder = input_builder.placeholder(ph);
        }
        
        if let Some(validations) = validate {
            // Clone the validations to ensure ownership in the closure
            let validations_owned = validations.clone();
            input_builder = input_builder.validate(move |input: &String| {
                for validation in &validations_owned {
                    match validation {
                        Validation::NotEmpty { message } => {
                            if input.is_empty() {
                                return Err(message.clone());
                            }
                        },
                        Validation::Regex { pattern, message } => {
                            let regex = Regex::new(&pattern).map_err(|_| "Invalid regex pattern")?;
                            if !regex.is_match(input) {
                                return Err(message.clone());
                            }
                        },
                        Validation::MustStartWithLetter { message } => {
                            if !input.chars().next().map_or(false, |c| c.is_alphabetic()) {
                                return Err(message.clone());
                            }
                        }
                    }
                }
                Ok(())
            });
        }
        
        let input_value: String = input_builder.interact()?;
        *last_input = Some(input_value.clone());
        
        // Store output in context if specified
        if let Some(output_key) = output {
            context.insert(output_key.clone(), input_value);
        }
    }
    Ok(())
}
