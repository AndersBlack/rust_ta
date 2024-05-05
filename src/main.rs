use actix_web::{App, HttpServer};
mod calls;
mod car;

#[actix_web::main]
async fn main () -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(calls::get_all_cars)
            .service(calls::get_specific_car)
            .service(calls::delete_specific_car)
            .service(calls::add_car)
            .service(calls::update_car)
    }).bind(("localhost", 9090))?
    .run()
    .await

}