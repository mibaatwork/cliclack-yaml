use std::collections::HashMap;
use anyhow::Result;
use crate::{PromptStepType, apply_text_style};

/// Handle Intro prompt step
pub fn handle_intro(step_type: &PromptStepType, _context: &mut HashMap<String, String>) -> Result<()> {
    if let PromptStepType::Intro { text, style } = step_type {
        let styled_text = apply_text_style(text, style);
        cliclack::intro(styled_text)?;
    }
    Ok(())
}
