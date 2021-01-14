use actix_web::{ App, HttpServer};
mod controller;

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(
        || App::new()
            .service(controller::index::index)

            .service(controller::v1::from_manager::robot::get_v1_robot_from_manager)
            .service(controller::v1::from_manager::robot::get_v1_robots_from_manager)
            .service(controller::v1::from_manager::robot::register_v1_robot_from_manager)
            .service(controller::v1::from_manager::robot::update_v1_robot_from_manager)
            .service(controller::v1::from_manager::robot::delete_v1_robot_from_manager)

            .service(controller::v1::from_manager::hoge_interface::get_v1_hoge_interface_from_manager)
            .service(controller::v1::from_manager::hoge_interface::get_v1_hoge_interfaces_from_manager)
            .service(controller::v1::from_manager::hoge_interface::register_v1_hoge_interface_from_manager)
            .service(controller::v1::from_manager::hoge_interface::update_v1_hoge_interface_from_manager)
            .service(controller::v1::from_manager::hoge_interface::delete_v1_hoge_interface_from_manager)

            .service(controller::v1::from_manager::equipment::get_v1_equipment_from_manager)
            .service(controller::v1::from_manager::equipment::get_v1_equipments_from_manager)
            .service(controller::v1::from_manager::equipment::register_v1_equipment_from_manager)
            .service(controller::v1::from_manager::equipment::update_v1_equipment_from_manager)
            .service(controller::v1::from_manager::equipment::delete_v1_equipment_from_manager)

            .service(controller::v1::from_manager::parameter::get_v1_parameter_from_manager)
            .service(controller::v1::from_manager::parameter::get_v1_parameters_from_manager)
            .service(controller::v1::from_manager::parameter::register_v1_parameter_from_manager)
            .service(controller::v1::from_manager::parameter::update_v1_parameter_from_manager)
            .service(controller::v1::from_manager::parameter::delete_v1_parameter_from_manager)

            .service(controller::v1::from_manager::parameter_chip::get_v1_parameter_chip_from_manager)
            .service(controller::v1::from_manager::parameter_chip::get_v1_parameter_chips_from_manager)
            .service(controller::v1::from_manager::parameter_chip::register_v1_parameter_chip_from_manager)
            .service(controller::v1::from_manager::parameter_chip::update_v1_parameter_chip_from_manager)
            .service(controller::v1::from_manager::parameter_chip::delete_v1_parameter_chip_from_manager)

            .service(controller::v1::from_manager::user::get_v1_user_from_manager)
            .service(controller::v1::from_manager::user::get_v1_users_from_manager)
            .service(controller::v1::from_manager::user::update_v1_user_from_manager)
            .service(controller::v1::from_manager::user::delete_v1_user_from_manager)

            .service(controller::v1::from_manager::myset::get_v1_myset_from_manager)
            .service(controller::v1::from_manager::myset::get_v1_mysets_from_manager)
            
            .service(controller::v1::from_mobile::robot::get_v1_robot_from_mobile)
            .service(controller::v1::from_mobile::robot::get_v1_robots_from_mobile)

            .service(controller::v1::from_mobile::equipment::get_v1_equipment_from_mobile)
            .service(controller::v1::from_mobile::equipment::get_v1_equipments_from_mobile)

            .service(controller::v1::from_mobile::parameter_chip::get_v1_parameter_chip_from_mobile)
            .service(controller::v1::from_mobile::parameter_chip::get_v1_parameter_chips_from_mobile)

            .service(controller::v1::from_mobile::user::get_v1_user_from_mobile)
            .service(controller::v1::from_mobile::user::register_v1_user_from_mobile)

            .service(controller::v1::from_mobile::myset::get_v1_myset_from_mobile)
            .service(controller::v1::from_mobile::myset::get_v1_mysets_from_mobile)
            .service(controller::v1::from_mobile::myset::update_v1_myset_from_mobile)
            .service(controller::v1::from_mobile::myset::delete_v1_myset_from_mobile)

            .service(controller::v1::from_game::robot::get_v1_robot_from_game)
            .service(controller::v1::from_game::robot::get_v1_robots_from_game)

            .service(controller::v1::from_game::equipment::get_v1_equipment_from_game)
            .service(controller::v1::from_game::equipment::get_v1_equipments_from_game)

            .service(controller::v1::from_game::parameter_chip::get_v1_parameter_chip_from_game)
            .service(controller::v1::from_game::parameter_chip::get_v1_parameter_chips_from_game)

            .service(controller::v1::from_game::user::get_v1_user_from_game)
            .service(controller::v1::from_game::user::register_v1_user_from_game)

            .service(controller::v1::from_game::myset::get_v1_myset_from_game)
            .service(controller::v1::from_game::myset::get_v1_mysets_from_game)
            .service(controller::v1::from_game::myset::update_v1_myset_from_game)
            .service(controller::v1::from_game::myset::delete_v1_myset_from_game)

    )
    .bind("0.0.0.0:5000")?
    .run()
    .await?;
    Ok(())
}
