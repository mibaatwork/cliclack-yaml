use std::env;
use anyhow::Result;
use std::fs;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    render_prompt_interaction,
    render_prompt_interaction_with_vars,
    test_utils::setup_test
};

#[test]
fn test_clear_intro_outro_steps() -> Result<()> {
    setup_test();
    set_app_root(env::current_dir()?)?;
    set_prompt_folder("tests/steps")?;
    let _output_values = render_prompt_interaction("clear_intro_outro")?;
    Ok(())
}

#[test]
fn test_intro_outro_steps() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    let steps_dir = current_dir.join("tests").join("steps");
    fs::create_dir_all(&steps_dir)?;
    
    let test_yaml = r#"
- type: intro
  text: "Welcome to the test"
  style:
    color: "blue"
- type: outro
  text: "Test completed"
  style:
    color: "green"
"#;
    
    let test_file = steps_dir.join("test_intro_outro.yaml");
    fs::write(&test_file, test_yaml)?;
    
    set_prompt_folder("tests/steps")?;
    let result = render_prompt_interaction("test_intro_outro")?;
    
    fs::remove_file(test_file)?;
    assert!(result.is_empty(), "Expected empty result map");
    
    Ok(())
}

#[test]
fn test_intro_outro_with_variables() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    let steps_dir = current_dir.join("tests").join("steps");
    fs::create_dir_all(&steps_dir)?;
    
    let test_yaml = r#"
- type: intro
  text: "Welcome {name}"
  style:
    color: "blue"
- type: outro
  text: "Goodbye {name}"
  style:
    color: "green"
"#;
    
    let test_file = steps_dir.join("test_vars.yaml");
    fs::write(&test_file, test_yaml)?;
    
    set_prompt_folder("tests/steps")?;
    let result = render_prompt_interaction_with_vars(
        "test_vars",
        vec![("name", "Test User")]
    )?;
    
    fs::remove_file(test_file)?;
    assert_eq!(result.get("name"), Some(&"Test User".to_string()));
    
    Ok(())
}
