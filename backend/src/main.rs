use actix_web::{web, App, HttpServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::load_public()?;
    let addr = ([0; 4], config.domains().backend().port()).into();

    HttpServer::new(|| {
        App::new()
            .service(account::query_current_user)
            .service(account::query_user)
            .service(ci::list_recent)
            .service(ci::list_projects)
            .service(ci::view_project)
            .service(ci::view_build)
            .service(plugins::list)
            .service(plugins::details)
            .service(plugins::release)
            .service(plugins::approve)
            .service(plugins::reject)
            .service(plugins::review)
    })
    .bind(addr)?
    .run()
    .await
}
