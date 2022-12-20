use abstract_factory::app::Application;

fn main() {
    let mac_app = Application::new_gui_factory("mac");
    let btn = mac_app.create_button();
    btn.paint(); // output: mac os button
    let cb = mac_app.create_checkbox();
    cb.paint(); // output: mac os checkbox

    let win_app = Application::new_gui_factory("win");
    let btn = win_app.create_button();
    btn.paint(); // output: windows os button
    let cb = win_app.create_checkbox();
    cb.paint(); // output: windows os checkbox
}
