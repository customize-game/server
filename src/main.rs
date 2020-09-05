use actix_web::{get, App, HttpResponse, HttpServer};


#[get("/")]
async fn index() ->Result<HttpResponse, actix_web::Error >{
    let response_body = "Customize Game!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:5000")?
        .run()
        .await?;
    Ok(())
}
