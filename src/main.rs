use actix_web::{get, post, delete, App, HttpResponse, HttpServer};


struct DataEntry {
    id: u32,
    text: String,
}

struct template {
    entries: Vec<DataEntry>,
}

#[post("/")]
async fn index() ->Result<HttpResponse, actix_web::Error >{
    let response_body = "Customize Game!";
    Ok(HttpResponse::Ok().body(response_body))
}

#[post("/add")]
async fn add() ->Result<HttpResponse, actix_web::Error >{
    let response_body = "Add Post";
    Ok(HttpResponse::Ok().body(response_body))
}

#[delete("/delete")]
async fn delete() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Delete method";
    return Ok(HttpResponse::Ok().body(response_body));
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || 
    App::new()
        .service(index)
        .service(add)
        .service(delete)
    )
    .bind("0.0.0.0:5000")?
    .run()
    .await?;
    Ok(())
}
