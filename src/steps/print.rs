use std::collections::HashMap;
use anyhow::Result;
use console::style;
use crate::PromptStepType;
use cliclack::log;

/// Handle Print prompt step
pub fn handle_print(step_type: &PromptStepType, context: &mut HashMap<String, String>) -> Result<()> {
    if let PromptStepType::Print { text, loglevel, input } = step_type {
        // Get the text value with variables replaced
        let mut display_text = text.clone();
        
        // If input is specified, use it from context
        if let Some(input_key) = input {
            if let Some(input_value) = context.get(input_key) {
                display_text = display_text.replace(&format!("{{{}}}", input_key), input_value);
            }
        }
        
        // Replace any other variables from context
        for (key, value) in context {
            display_text = display_text.replace(&format!("{{{}}}", key), value);
        }
        
        // Use appropriate log level:
        //   None (no loglevel set)    → bar continuation │  (blends into frame)
        //   Some("none")              → remark ├  (connect-left, no marker icon)
        //   Some("info/warning/error")→ marker icon (●, ▲, ■)
        match loglevel.as_deref() {
            Some("info") => log::info(&display_text)?,
            Some("warning") | Some("warn") => log::warning(&display_text)?,
            Some("error") => log::error(&display_text)?,
            Some("none") => log::remark(&display_text)?,
            None => {
                let bar = style("│").bright().black();
                for line in display_text.lines() {
                    println!("{}  {}", bar, line);
                }
                if display_text.ends_with('\n') {
                    println!("{}", bar);
                }
            }
            _ => log::remark(&display_text)?,
        }
    }
    Ok(())
}
