mod app;
mod data;
mod io;

use crate::app::App;
use crate::io::AppFileOps;
use crate::io::AppView;

fn main() {
    let mut app = App::<AppView, AppFileOps>::new();
    app.run();
}
