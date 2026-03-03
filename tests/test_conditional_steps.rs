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
#[ignore = "Conditional tests require user interaction"]
fn test_conditional_step_execution() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    let steps_dir = current_dir.join("tests").join("steps");
    fs::create_dir_all(&steps_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let test_yaml = r#"
- type: confirm
  prompt: "Do you want to proceed?"
  output: "should_proceed"
  step_name: "proceed_step"

- type: input
  prompt: "Enter name"
  output: "name"
  condition:
    parent: "proceed_step"
    value: true

- type: print
  text: "Hello {name}"
  loglevel: "info"
  condition:
    parent: "proceed_step"
    value: true
"#;
    
    let test_file = steps_dir.join("test_conditional.yaml");
    fs::write(&test_file, test_yaml)?;
    
    let result = render_prompt_interaction_with_vars(
        "test_conditional",
        vec![
            ("should_proceed", "true"),
            ("name", "Test User")
        ]
    )?;
    
    fs::remove_file(test_file)?;
    assert_eq!(result.get("name"), Some(&"Test User".to_string()));
    
    Ok(())
}
