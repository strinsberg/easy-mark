mod comment;
mod assignment;
mod app;
mod display;
mod latex;

use crate::app::App;


fn main() {
    let mut app = App::new();
    app.run();
}
