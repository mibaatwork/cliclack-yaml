use std::env;
use anyhow::Result;
use std::fs;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    render_prompt_interaction_with_vars,
    test_utils::setup_test
};

#[test]
#[ignore = "Confirm prompts require user interaction"]
fn test_confirm_with_output() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    let steps_dir = current_dir.join("tests").join("steps");
    fs::create_dir_all(&steps_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let test_yaml = r#"
- type: confirm
  prompt: "Do you want to continue?"
  output: "should_continue"
"#;
    
    let test_file = steps_dir.join("test_confirm.yaml");
    fs::write(&test_file, test_yaml)?;
    
    let result = render_prompt_interaction_with_vars(
        "test_confirm",
        vec![("should_continue", "true")]
    )?;
    
    fs::remove_file(test_file)?;
    assert_eq!(result.get("should_continue"), Some(&"true".to_string()));
    
    Ok(())
}
