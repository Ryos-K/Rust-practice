use crate::gui::*;

pub struct MacFactory;
impl GUIFactory for MacFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(MacButton {})
    }
    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(MacCheckbox {})
    }
}

pub struct MacButton;
impl Button for MacButton {
    fn paint(&self) {
        println!("mac os button");
    }
}

pub struct MacCheckbox;
impl Checkbox for MacCheckbox {
    fn paint(&self) {
        println!("mac os checkbox");
    }
}
