use std::env;
use anyhow::Result;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    render_prompt_interaction,
    test_utils::setup_test
};

#[test]
#[ignore = "Progress bars require terminal interaction"]
fn test_progress_bar() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Test basic progress bar
    let result = render_prompt_interaction("test_progress_bar")?;
    
    assert_eq!(result.get("install_status"), Some(&"completed".to_string()));
    
    Ok(())
}

#[test]
#[ignore = "Progress bars require terminal interaction"]
fn test_progress_bar_download() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Test progress bar with download template
    let _result = render_prompt_interaction("test_progress_bar_download")?;
    
    Ok(())
}

#[test]
#[ignore = "Progress bars require terminal interaction"]
fn test_spinner() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Test spinner
    let _result = render_prompt_interaction("test_spinner")?;
    
    Ok(())
}

#[test]
#[ignore = "Progress bars require terminal interaction"]
fn test_multi_progress() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Test multi-progress
    let result = render_prompt_interaction("test_multi_progress")?;
    
    assert_eq!(result.get("build_status"), Some(&"completed".to_string()));
    
    Ok(())
}
