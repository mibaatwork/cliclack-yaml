use std::env;
use anyhow::Result;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    render_prompt_interaction_with_vars,
    test_utils::setup_test
};

#[test]
fn test_print_log_levels() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let result = render_prompt_interaction_with_vars(
        "test_print",
        Vec::<(&str, String)>::new()
    )?;
    
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_print_with_variables() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let result = render_prompt_interaction_with_vars(
        "test_print",
        vec![
            ("name", String::from("Test User")),
            ("project", String::from("CLI Project"))
        ]
    )?;
    
    assert_eq!(result.get("name"), Some(&"Test User".to_string()));
    assert_eq!(result.get("project"), Some(&"CLI Project".to_string()));
    Ok(())
}

#[test]
fn test_print_with_input_variable() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let result = render_prompt_interaction_with_vars(
        "test_print",
        vec![("user_input", String::from("test value"))]
    )?;
    
    assert_eq!(result.get("user_input"), Some(&"test value".to_string()));
    Ok(())
}

#[test]
fn test_print_from_yaml_file() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let result = render_prompt_interaction_with_vars(
        "test_print",
        vec![
            ("name", String::from("John Doe")),
            ("project", String::from("Test Project")),
            ("version", String::from("1.0.0"))
        ]
    )?;
    
    assert_eq!(result.get("name"), Some(&"John Doe".to_string()));
    assert_eq!(result.get("project"), Some(&"Test Project".to_string()));
    assert_eq!(result.get("version"), Some(&"1.0.0".to_string()));
    
    Ok(())
}

#[test]
fn test_print_without_loglevel() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // Should succeed even though some print steps omit loglevel
    let result = render_prompt_interaction_with_vars(
        "test_print",
        Vec::<(&str, String)>::new()
    )?;
    
    assert!(result.is_empty());
    Ok(())
}

#[test]
fn test_print_without_loglevel_with_input() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    set_prompt_folder("tests/steps")?;
    
    // The plain print step (no loglevel) with input variable should work
    let result = render_prompt_interaction_with_vars(
        "test_print",
        vec![
            ("project", String::from("MyProject")),
        ]
    )?;
    
    assert_eq!(result.get("project"), Some(&"MyProject".to_string()));
    Ok(())
}
