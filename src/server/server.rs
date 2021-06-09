use actix_web::{ App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;
use crate::infrastructures::infrastructures as infra;
use crate::server::v1::handlers;

#[actix_rt::main]
pub async fn run() -> Result<(), actix_web::Error> {
    HttpServer::new(|| {
        // TODO ちゃんと設定する
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST","PUT","DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .data(infra::DbServer::new())
            .service(handlers::get_myself)
    })
    .bind(format!("0.0.0.0:{}", infra::CONFIG.server_port))?
    .run()
    .await?;
    Ok(())
}
