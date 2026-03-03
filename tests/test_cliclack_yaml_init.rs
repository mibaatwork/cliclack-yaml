use std::env;
use anyhow::Result;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    get_app_root,
    get_prompt_folder,
    test_utils::setup_test
};

/// This test specifically tests the initialization functionality of cliclack_yaml
/// by setting the app root and prompt folder paths.
#[test]
fn test_cliclack_yaml_initialization() -> Result<()> {
    setup_test();
    
    // Set up app root using current directory
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    // Set prompt folder to tests/steps
    let prompt_folder = "tests/steps";
    set_prompt_folder(prompt_folder)?;
    
    // Now verify the paths were set correctly
    let app_root = get_app_root()
        .ok_or_else(|| anyhow::anyhow!("APP_ROOT was not set"))?;
    assert_eq!(app_root, current_dir, "APP_ROOT was not set to current directory");
    
    let prompt_path = get_prompt_folder()
        .ok_or_else(|| anyhow::anyhow!("PROMPT_FOLDER was not set"))?;
    assert_eq!(prompt_path, current_dir.join(prompt_folder), "PROMPT_FOLDER was not set correctly");
    
    Ok(())
}
