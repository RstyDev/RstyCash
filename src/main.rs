mod app;
mod mods;
use app::App;

fn main() {
    println!("inicio");
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
