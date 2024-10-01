#[cfg(not(feature="ssr",feature="csr"),feature="syc")]
fn main() {
    mod sycamore;
    use sycamore::app::App;
    console_error_panic_hook::set_once();
    sycamore::render(App);
}
#[cfg(not(feature="csr",feature="syc"),feature="ssr")]
use actix_web::{get, web::{self, Data}, App, Error, HttpResponse, HttpServer, Responder};
#[cfg(not(feature="csr",feature="syc"),feature="ssr")]
#[actix_web::main]
async fn main()->std::io::Result<()>{
    mod server;
    
    Ok()
}