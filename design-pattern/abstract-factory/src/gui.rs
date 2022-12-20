pub trait GUIFactory {
    fn create_button(&self) -> Box<dyn Button>;
    fn create_checkbox(&self) -> Box<dyn Checkbox>;
}

pub trait Button {
    fn paint(&self);
}

pub trait Checkbox {
    fn paint(&self);
}
