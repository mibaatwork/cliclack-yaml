use std::collections::HashMap;
use anyhow::Result;
use crate::PromptStepType;

/// Handle ClearScreen prompt step
pub fn handle_clear_screen(step_type: &PromptStepType, _context: &mut HashMap<String, String>) -> Result<()> {
    if let PromptStepType::ClearScreen = step_type {
        cliclack::clear_screen()?;
    }
    Ok(())
}
