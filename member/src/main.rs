// fn main() {
//     println!("Welcome from Building Membership Registration Service");
// }

use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use member::{handler, utils};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    utils::initiate_logging();
    info!("*** Welcome from member's Service");

    // let address = std::env::var("ADDRESS").expect("Missed 'ADDRESS' environment variable");
    let address = format!("0.0.0.0:8090");

    info!("Server address: {address}");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/api").configure(handler::routes_config))
    })
    .bind(address)?
    .run()
    .await
}
