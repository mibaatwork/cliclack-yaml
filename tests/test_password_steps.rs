use std::env;
use anyhow::Result;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    render_prompt_interaction_with_vars,
    test_utils::setup_test
};

#[test]
#[ignore = "Password prompts require user interaction"]
fn test_password_basic() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Use the existing test_password.yaml file
    let result = render_prompt_interaction_with_vars(
        "test_password",
        vec![("user_password", "secret123")]
    )?;
    
    assert_eq!(result.get("user_password"), Some(&"secret123".to_string()));
    
    Ok(())
}

#[test]
#[ignore = "Password prompts require user interaction"]
fn test_password_simple() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Test simple password without confirmation
    let result = render_prompt_interaction_with_vars(
        "test_password_simple",
        vec![("api_key", "test-api-key-123")]
    )?;
    
    assert_eq!(result.get("api_key"), Some(&"test-api-key-123".to_string()));
    
    Ok(())
}

#[test]
#[ignore = "Password prompts require user interaction"]
fn test_password_confirm_default() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Test password with default confirm prompt
    let result = render_prompt_interaction_with_vars(
        "test_password_confirm_default",
        vec![("new_password", "newpassword123")]
    )?;
    
    assert_eq!(result.get("new_password"), Some(&"newpassword123".to_string()));
    
    Ok(())
}
