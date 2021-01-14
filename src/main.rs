use actix_web::{ App, HttpServer};
mod controller;

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(
        || App::new()
            .service(controller::index::index)

            .service(controller::v1::from_game::equipment::get_one)
            .service(controller::v1::from_game::equipment::get_list)

            .service(controller::v1::from_game::myset::get_one)
            .service(controller::v1::from_game::myset::get_list)
            .service(controller::v1::from_game::myset::register)
            .service(controller::v1::from_game::myset::update)
            .service(controller::v1::from_game::myset::delete)

            .service(controller::v1::from_game::parameter_chip::get_one)
            .service(controller::v1::from_game::parameter_chip::get_list)

            .service(controller::v1::from_game::robot::get_one)
            .service(controller::v1::from_game::robot::get_list)

            .service(controller::v1::from_game::user::get_myself)
            .service(controller::v1::from_game::user::register)

            .service(controller::v1::from_manager::equipment::get_one)
            .service(controller::v1::from_manager::equipment::get_list)
            .service(controller::v1::from_manager::equipment::register)
            .service(controller::v1::from_manager::equipment::update)
            .service(controller::v1::from_manager::equipment::delete)

            .service(controller::v1::from_manager::hoge_interface::get_one)
            .service(controller::v1::from_manager::hoge_interface::get_list)
            .service(controller::v1::from_manager::hoge_interface::register)
            .service(controller::v1::from_manager::hoge_interface::update)
            .service(controller::v1::from_manager::hoge_interface::delete)

            .service(controller::v1::from_manager::myset::get_one)
            .service(controller::v1::from_manager::myset::get_list)
            
            .service(controller::v1::from_manager::parameter_chip::get_one)
            .service(controller::v1::from_manager::parameter_chip::get_list)
            .service(controller::v1::from_manager::parameter_chip::register)
            .service(controller::v1::from_manager::parameter_chip::update)
            .service(controller::v1::from_manager::parameter_chip::delete)

            .service(controller::v1::from_manager::parameter::get_one)
            .service(controller::v1::from_manager::parameter::get_list)
            .service(controller::v1::from_manager::parameter::register)
            .service(controller::v1::from_manager::parameter::update)
            .service(controller::v1::from_manager::parameter::delete)

            .service(controller::v1::from_manager::robot::get_one)
            .service(controller::v1::from_manager::robot::get_list)
            .service(controller::v1::from_manager::robot::register)
            .service(controller::v1::from_manager::robot::update)
            .service(controller::v1::from_manager::robot::delete)

            .service(controller::v1::from_manager::user::get_one)
            .service(controller::v1::from_manager::user::get_list)
            .service(controller::v1::from_manager::user::update)
            .service(controller::v1::from_manager::user::delete)

            .service(controller::v1::from_mobile::equipment::get_one)
            .service(controller::v1::from_mobile::equipment::get_list)

            .service(controller::v1::from_mobile::myset::get_one)
            .service(controller::v1::from_mobile::myset::get_list)
            .service(controller::v1::from_mobile::myset::register)
            .service(controller::v1::from_mobile::myset::update)
            .service(controller::v1::from_mobile::myset::delete)

            .service(controller::v1::from_mobile::parameter_chip::get_one)
            .service(controller::v1::from_mobile::parameter_chip::get_list)

            .service(controller::v1::from_mobile::robot::get_one)
            .service(controller::v1::from_mobile::robot::get_list)

            .service(controller::v1::from_mobile::user::get_myself)
            .service(controller::v1::from_mobile::user::register)
    )
    .bind("0.0.0.0:5000")?
    .run()
    .await?;
    Ok(())
}
