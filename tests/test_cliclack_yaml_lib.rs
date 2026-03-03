use std::{env, path::{Path, PathBuf}};
use anyhow::Result;
use cliclack_yaml::{get_app_root, get_prompt_folder, set_app_root, set_prompt_folder, PROMPT_FOLDER};

#[cfg(test)]
pub fn set_prompt_folder_for_test<P: AsRef<Path>>(path: P) -> Result<()> {
    let path_ref = path.as_ref();
    
    // Determine if this is an absolute path or relative to app root
    let full_path = if path_ref.is_absolute() || path_ref.to_string_lossy().contains(":") {
        // Use as is if it's absolute
        path_ref.to_path_buf()
    } else {
        // Otherwise, it's relative to app root or current directory
        if let Some(app_root) = get_app_root() {
            app_root.join(path_ref)
        } else {
            // Fallback to current working directory if app root not set
            match env::current_dir() {
                Ok(current_dir) => current_dir.join(path_ref),
                Err(_) => path_ref.to_path_buf()
            }
        }
    };
    
    PROMPT_FOLDER.with(|r| {
        // In tests, reset the value if it's already set
        *r.borrow_mut() = Some(full_path);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_paths() {
        assert_eq!(get_app_root(), None, "App root should be None initially");
        assert_eq!(
            get_prompt_folder(),
            None,
            "Prompt folder should be None initially"
        );
    }

    #[test]
    fn test_set_prompt_folder() -> Result<()> {
        // First ensure we have an app root set
        let current_dir = env::current_dir()?;
        set_app_root(&current_dir)?;

        let test_path = PathBuf::from("test_prompts");
        assert!(
            set_prompt_folder(&test_path).is_ok(),
            "Should be able to set prompt folder"
        );
        assert!(
            set_prompt_folder(&test_path).is_err(),
            "Should not be able to set prompt folder twice"
        );
        Ok(())
    }
}
