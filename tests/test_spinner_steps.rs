use std::env;
use anyhow::Result;
use std::fs;
use cliclack_yaml::{
    set_app_root,
    set_prompt_folder,
    render_prompt_interaction,
    test_utils::setup_test
};

#[test]
#[ignore = "Spinner tests require terminal interaction"]
fn test_spinner_and_function() -> Result<()> {
    setup_test();
    
    let current_dir = env::current_dir()?;
    set_app_root(&current_dir)?;
    
    let steps_dir = current_dir.join("tests").join("steps");
    fs::create_dir_all(&steps_dir)?;
    set_prompt_folder("tests/steps")?;
    
    let test_yaml = r#"
- type: spinner
  start_text: "Processing..."
  stop_text: "Done!"
  run_fn: "wait_five_seconds"
  output: "result"
"#;
    
    let test_file = steps_dir.join("test_spinner.yaml");
    fs::write(&test_file, test_yaml)?;
    
    let result = render_prompt_interaction("test_spinner")?;
    
    fs::remove_file(test_file)?;
    assert_eq!(result.get("result"), Some(&"Task completed successfully".to_string()));
    
    Ok(())
}
