use crate::gui::GUIFactory;
use crate::macos::MacFactory;
use crate::windows::WinFactory;

pub struct Application;
impl Application {
    pub fn new_gui_factory(os: &str) -> Box<dyn GUIFactory> {
        match os {
            "mac" => Box::new(MacFactory {}),
            "win" => Box::new(WinFactory {}),
            _ => panic!("error"),
        }
    }
}
