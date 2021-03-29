mod app;
mod data;
mod io;
mod latex;

use crate::app::App;
use crate::io::AppView;

fn main() {
    let mut app = App::<AppView>::new();
    app.run();
}
