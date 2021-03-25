mod comment;
mod assignment;
mod app;

use crate::app::App;


fn main() {
    let mut app = App::new();
    app.run();
}
