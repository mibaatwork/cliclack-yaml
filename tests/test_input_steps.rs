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
#[ignore = "Input prompts require user interaction"]
fn test_input_validation() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    let steps_dir = current_dir.join("tests").join("steps");
    fs::create_dir_all(&steps_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let test_yaml = r#"
- type: input
  prompt: "Enter your name"
  output: "name"
  validate:
    - type: not-empty
      message: "Name cannot be empty"
    - type: must-start-with-letter
      message: "Name must start with a letter"
    - type: regex
      pattern: "^[A-Za-z][A-Za-z0-9_]*$"
      message: "Name must contain only letters, numbers, and underscores"
"#;
    
    let test_file = steps_dir.join("test_input.yaml");
    fs::write(&test_file, test_yaml)?;
    
    let result = render_prompt_interaction_with_vars(
        "test_input",
        vec![("name", "TestUser123")]
    )?;
    
    fs::remove_file(test_file)?;
    assert_eq!(result.get("name"), Some(&"TestUser123".to_string()));
    
    Ok(())
}
