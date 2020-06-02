use actix_web::{web, App, HttpServer};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let config = config::load_public()?;
    let addr = ([0; 4], config.domains().addr().port()).into();

    HttpServer::new(|| {
        App::new()
            .service(download::download)
            .service(info::downloads)
            .service(info::size)
            .service(info::md5)
            .service(info::sha1)
    })
    .bind(addr)?
    .run()
    .await
}
