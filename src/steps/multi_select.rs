use std::collections::HashMap;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::{PromptStepType, replace_variables};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MultiSelectItem {
    pub value: String,
    pub label: String,
    pub hint: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum MultiSelectItems {
    Static(Vec<MultiSelectItem>),
    Variable(String),
}

/// Handle MultiSelect prompt step
pub fn handle_multi_select(
    step_type: &PromptStepType, 
    context: &mut HashMap<String, String>,
    last_input: &mut Option<String>
) -> Result<()> {
    if let PromptStepType::MultiSelect { prompt, items, output, required } = step_type {
        let mut multiselect_builder = cliclack::multiselect(prompt);
        
        // Determine which items to use
        let resolved_items = match items {
            MultiSelectItems::Static(static_items) => static_items.clone(),
            MultiSelectItems::Variable(variable_ref) => {
                // Replace variables in the variable reference string
                let resolved_var = replace_variables(variable_ref, context, last_input);
                
                // The resolved_var should now contain the actual JSON data
                // First try to parse as an array of MultiSelectItem objects
                if let Ok(items) = serde_json::from_str::<Vec<MultiSelectItem>>(&resolved_var) {
                    items
                } else if let Ok(config_map) = serde_json::from_str::<HashMap<String, String>>(&resolved_var) {
                    // Fallback to HashMap for backward compatibility
                    config_map.into_iter().map(|(key, value)| {
                        let hint = if value.len() > 50 { 
                            format!("{}...", &value[..47]) 
                        } else { 
                            value 
                        };
                        MultiSelectItem {
                            value: key.clone(),
                            label: key,
                            hint: Some(hint),
                        }
                    }).collect()
                } else {
                    Vec::new()
                }
            }
        };
        
        // Add items to multiselect builder
        if resolved_items.is_empty() {
            return Err(anyhow::anyhow!("No items available for multiselect"));
        }
        
        for item in &resolved_items {
            let hint = item.hint.as_deref().unwrap_or("");
            multiselect_builder = multiselect_builder.item(&item.value, &item.label, hint);
        }
        
        // Apply required validation if specified
        if let Some(is_required) = required {
            if *is_required {
                multiselect_builder = multiselect_builder.required(true);
            }
        }
        
        let selected_values: Vec<String> = multiselect_builder.interact()?.iter().map(|s| s.to_string()).collect();
        
        // Store output in context if specified
        if let Some(output_key) = output {
            // Join selected values with commas for storage
            let combined_values = selected_values.join(",");
            context.insert(output_key.clone(), combined_values);
            
            // Also store individual selections with indexed keys for potential use
            for (index, value) in selected_values.iter().enumerate() {
                context.insert(format!("{}_{}", output_key, index), value.clone());
            }
            
            // Store count of selected items
            context.insert(format!("{}_count", output_key), selected_values.len().to_string());
        }
    }
    Ok(())
}
