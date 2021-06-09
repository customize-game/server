mod server;
mod domains;
mod infrastructures;
mod usecases;



// メイン処理
fn main() -> Result<(), actix_web::Error> {
    server::server::run()
}
