use thousandfold::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);

    info!("HI THERE");
    app.run();
}
