use std::path::PathBuf;
use std::cell::RefCell;
use std::thread_local;

thread_local! {
    static TEST_APP_ROOT: RefCell<Option<PathBuf>> = RefCell::new(None);
    static TEST_PROMPT_FOLDER: RefCell<Option<PathBuf>> = RefCell::new(None);
}

pub fn setup_test() {
    TEST_APP_ROOT.with(|r| *r.borrow_mut() = None);
    TEST_PROMPT_FOLDER.with(|r| *r.borrow_mut() = None);
}

pub fn get_test_app_root() -> Option<PathBuf> {
    TEST_APP_ROOT.with(|r| r.borrow().clone())
}

pub fn get_test_prompt_folder() -> Option<PathBuf> {
    TEST_PROMPT_FOLDER.with(|r| r.borrow().clone())
}

pub fn set_test_app_root(path: PathBuf) -> Option<()> {
    TEST_APP_ROOT.with(|r| {
        if r.borrow().is_none() {
            *r.borrow_mut() = Some(path);
            Some(())
        } else {
            None
        }
    })
}

pub fn set_test_prompt_folder(path: PathBuf) -> Option<()> {
    TEST_PROMPT_FOLDER.with(|r| {
        if r.borrow().is_none() {
            *r.borrow_mut() = Some(path);
            Some(())
        } else {
            None
        }
    })
}
