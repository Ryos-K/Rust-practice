use crate::gui::*;

pub struct WinCheckbox;
impl Checkbox for WinCheckbox {
    fn paint(&self) {
        println!("windows os checkbox");
    }
}
pub struct WinButton;
impl Button for WinButton {
    fn paint(&self) {
        println!("windows os button");
    }
}
pub struct WinFactory;
impl GUIFactory for WinFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WinButton {})
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WinCheckbox {})
    }
}
