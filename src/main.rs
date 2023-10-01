use xilem::view::{button, h_stack, v_stack, View};
use xilem::{App, AppLauncher};

fn main() {
    let app = App::new(AppData {}, app_logic);
    let launcher = AppLauncher::new(app);
    launcher.run()
}

struct AppData {}

fn app_logic(_data: &mut AppData) -> impl View<AppData> {
    v_stack((
        button("hello", |_| println!("Hi")),
        h_stack((
            button("right", |_| println!("right")),
            button("wrong", |_| println!("wrong")),
        )),
    ))
}
