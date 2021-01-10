use actix_web::{ web, get, post, put, delete, App, HttpResponse, HttpRequest, HttpServer , Responder};
use serde::{Serialize, Deserialize};

mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

// struct Template {
//     entries: Vec<DataEntry>,
// }

// // ロボット取得API
// // from manager
// #[get("/api/v1/robots/{body_id}")]
// async fn get_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder{
//     let body_id: Option<u32> = Some(body_id);
//     utils::establish_connection();
//     /*
//     SELECT
//         b.id ,
//         b.name ,
//         b.flavor ,
//         b.display_order ,
//         bep.equipable_place_id ,
//         bep.name AS equipable_place_name ,
//         bep.display_order AS equipable_place_display_order ,
//         bfs.x ,
//         bfs.y ,
//         be.parameter_id ,
//         be.name AS parameter_name ,
//         be.display_order AS parameter_display_order ,
//         be.num        
//     FROM (
//         SELECT
//             b.id ,
//             b.name ,
//             b.flavor ,
//             b.display_order
//         FROM
//             bodies b
//         WHERE
//             b.id = body_id
//     ) b
//     LEFT JOIN (
//         SELECT
//             bep.equipable_place_id ,
//             ep.name ,
//             ep.display_order ,
//             bep.body_id
//         FROM
//             bodies_equipable_places bep
//         INNER JOIN
//             equipable_places ep
//         ON
//             bep.equipable_place_id = ep.id
//         AND
//             bep.body_id = b.id
//     ) bep
//     ON
//         b.id = bep.body_id
//     LEFT JOIN
//         body_free_squares bfs
//     ON
//         b.id = bfs.body_id
//     LEFT JOIN (
//         SELECT
//             p.parameter_id ,
//             p.name ,
//             p.display_order ,
//             be.body_id ,
//             be.num
//         FROM
//             body_effects be
//         INNER JOIN
//             parameters p
//         ON
//             be.parameter_id = p.id
//         AND
//             be.body_id = b.id
//     ) be
//     ON
//         b.id = be.id
//     ORDER BY
//         b.id ,
//         bep.equipable_place_id ,
//         be.parameter_id ,
//         bfs.x ,
//         bfs.y
//     */
//     return HttpResponse::Ok().json(
//         DataEntry {
//             id: body_id,
//             text: String::from("get robot"),
//         }
//     );
// }
// // ロボット一覧取得API
// // from manager
// #[get("/api/v1/robots")]
// async fn get_robots_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Get robots";
//     /*
//     SELECT
//         b.id ,
//         b.name ,
//         b.flavor ,
//         b.display_order ,
//         bep.equipable_place_id ,
//         bep.name AS equipable_place_name ,
//         bep.display_order AS equipable_place_display_order ,
//         bfs.x ,
//         bfs.y ,
//         be.parameter_id ,
//         be.name AS parameter_name ,
//         be.display_order AS parameter_display_order ,
//         be.num        
//     FROM
//         bodies b
//     LEFT JOIN (
//         SELECT
//             bep.equipable_place_id ,
//             ep.name ,
//             ep.display_order ,
//             bep.body_id
//         FROM
//             bodies_equipable_places bep
//         INNER JOIN
//             equipable_places ep
//         ON
//             bep.equipable_place_id = ep.id
//         AND
//             bep.body_id = b.id
//     ) bep
//     ON
//         b.id = bep.body_id
//     LEFT JOIN
//         body_free_squares bfs
//     ON
//         b.id = bfs.body_id
//     LEFT JOIN (
//         SELECT
//             p.parameter_id ,
//             p.name ,
//             p.display_order ,
//             be.body_id ,
//             be.num
//         FROM
//             body_effects be
//         INNER JOIN
//             parameters p
//         ON
//             be.parameter_id = p.id
//         AND
//             be.body_id = b.id
//     ) be
//     ON
//         b.id = be.id
//     ORDER BY
//         b.id ,
//         bep.equipable_place_id ,
//         be.parameter_id ,
//         bfs.x ,
//         bfs.y
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // ロボット登録API
// // from manager
// #[post("/api/v1/robots/{body_id}")]
// async fn register_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder{
//     let body_id: Option<u32> = Some(body_id);
//     /*
//     INSERT INTO
//         bodies (
//             id ,
//             name ,
//             ruby ,
//             flavor ,
//             display_order
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO 
//         bodies_equipable_places (
//             body_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO 
//         body_free_squares (
//             body_id ,
//             x ,
//             y
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO 
//         body_effects (
//             body_id ,
//             parameter_id ,
//             num
//         )
//         VALUES (

//         )        
//     */
//     return HttpResponse::Ok().json(
//         DataEntry {
//             id: body_id,
//             text: String::from("register robot"),
//         }
//     );
// }
// // ロボット更新API
// // from manager
// #[put("/api/v1/robots/{body_id}")]
// async fn update_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder{
//     let body_id: Option<u32> = Some(body_id);
//     /*
//     DELETE FROM
//         bodies_equipable_places
//     WHERE
//         body_id =
//     */
//     /*
//     DELETE FROM
//         body_free_squares
//     WHERE
//         body_id =
//     */
//     /*
//     DELETE FROM
//         body_effects
//     WHERE
//         body_id =
//     */
//     /*
//     UPDATE
//         bodies
//     SET
//         name = ,
//         ruby = ,
//         flavor = ,
//         display_order = ,
//     WHERE
//         id =
//     */
//     /*
//     INSERT INTO 
//         bodies_equipable_places (
//             body_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO 
//         body_free_squares (
//             body_id ,
//             x ,
//             y
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         body_effects (
//             body_id ,
//             parameter_id ,
//             num
//         )
//         VALUES (

//         )     
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセットを更新
//     */
//     return HttpResponse::Ok().json(
//         DataEntry {
//             id: body_id,
//             text: String::from("update robot"),
//         }
//     );
// }
// // ロボット削除API
// // from manager
// #[delete("/api/v1/robots/{body_id}")]
// async fn delete_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder{
//     let body_id: Option<u32> = Some(body_id);
//     /*
//     DELETE FROM
//         bodies_equipable_places
//     WHERE
//         body_id =
//     */
//     /*
//     DELETE FROM
//         body_free_squares
//     WHERE
//         body_id =
//     */
//     /*
//     DELETE FROM
//         body_effects
//     WHERE
//         body_id =
//     */
//     /*
//     DELETE FROM
//         having_bodies
//     WHERE
//         body_id =
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセット更新
//     */
//     /*
//     DELETE FROM
//         bodies
//     WHERE
//         id = 
//     */
//     return HttpResponse::Ok().json(
//         DataEntry {
//             id: body_id,
//             text: String::from("delete robot"),
//         }
//     );
// }

// // 装備可能箇所一覧取得API
// // from manager
// #[get("/api/v1/hoge-interfaces")]
// async fn get_hoge_interfaces_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Get hoge interfaces";
//     /*
//     SELECT
//         ep.id ,
//         ep.name
//     FROM
//         equipable_places ep
//     ORDER BY
//         ep.id
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // 装備可能箇所登録API
// // from manager
// #[post("/api/v1/hoge-interfaces")]
// async fn register_hoge_interface_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Register hoge interface";
//     /*
//     INSERT INTO
//         equipable_places (
//             id ,
//             name ,
//             display_order
//         )
//         VALUES (

//         )
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // 装備可能箇所更新API
// // from manager
// #[put("/api/v1/hoge-interfaces")]
// async fn update_hoge_interface_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Update hoge interface";
//     /*
//     UPDATE
//         equipable_places
//     SET
//         name = ,
//         display_order =
//     WHERE
//         id =
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // 装備可能箇所削除API
// // from manager
// #[delete("/api/v1/hoge-interfaces")]
// async fn delete_hoge_interface_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Delete hoge interface";
//     /*
//     DELETE FROM
//         bodies_equipable_places
//     WHERE
//         equipable_place_id =
//     */
//     /*
//     DELETE FROM
//         equipped_when_increasing_equipable_places
//     WHERE
//         equipable_place_id =
//     */
//     /*
//     DELETE FROM
//         equipped_when_unequipping_equipable_places
//     WHERE
//         equipable_place_id =
//     */
//     /*
//     DELETE FROM
//         equipments_equipable_in_equipable_places
//     WHERE
//         equipable_place_id =
//     */
//     /*
//     DELETE FROM
//         designated_place_to_equipment_by_effects
//     WHERE
//         equipable_place_id =
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセット更新
//     */
//     /*
//     DELETE FROM
//         equipable_places
//     WHERE
//         id =
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }

// // 装備一覧取得API
// // from manager
// #[get("/api/v1/equipments")]
// async fn get_equipments_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Get equipments interfaces";
//     /*
//     SELECT
//         e.id ,
//         e.name ,
//         e.ruby ,
//         e.flavor ,
//         e.display_order ,
//         e.add_square_count ,
//         ee.parameter_id ,
//         ee.name AS parameter_name ,
//         ee.num AS parameter_num ,
//         ee.display_order AS parameter_display_order ,
//         dptebe.equipable_place_id AS designated_place_to_equipment_by_effect_equipable_place_id ,
//         dptebe.equipable_place_name AS designated_place_to_equipment_by_effect_equipable_place_name ,
//         dptebe.equipable_place_display_order AS designated_place_to_equipment_by_effect_equipable_place_display_order ,
//         dptebe.parameter_id AS designated_place_to_equipment_by_effect_parameter_id ,
//         dptebe.parameter_name AS designated_place_to_equipment_by_effect_parameter_name ,
//         dptebe.parameter_display_order AS designated_place_to_equipment_by_effect_equipable_place_display_order ,
//         dptebe.num AS designated_place_to_equipment_by_effect_num ,
//         ewiep.equipable_place_id AS equipped_when_increasing_equipable_place_id ,
//         ewiep.name AS equipped_when_increasing_equipable_place_name ,
//         ewiep.display_order AS equipped_when_increasing_equipable_place_display_order ,
//         ewuep.equipable_place_id AS equipped_when_unequipping_equipable_place_id ,
//         ewuep.name AS equipped_when_unequipping_equipable_place_name ,
//         ewuep.display_order AS equipped_when_unequipping_equipable_place_display_order ,
//         eeiep.equipable_place_id AS equipments_equipable_in_equipable_place_id ,
//         eeiep.name AS equipments_equipable_in_equipable_place_name ,
//         eeiep.display_order AS equipments_equipable_in_equipable_place_display_order
//     FROM
//         equipments e
//     LEFT JOIN (
//         SELECT
//             ee.equipment_id ,
//             ee.parameter_id ,
//             ee.num ,
//             p.name ,
//             p.display_order
//         FROM
//             equipment_effects ee
//         INNER JOIN
//             parameters p
//         ON
//             ee.parameter_id = p.id
//         AND
//             ee.equipment_id = ee.parameter_id
//     ) ee
//     ON
//         e.id = ee.equipment_id
//     LEFT JOIN (
//         SELECT
//             dptebe.equipment_id ,
//             dptebe.equipable_place_id ,
//             dptebe.parameter_id ,
//             dptebe.num ,
//             ep.name AS equipable_place_name ,
//             ep.display_order AS equipable_place_display_order ,
//             p.name AS parameter_name ,
//             p.display_order AS parameter_display_order
//         FROM
//             designated_place_to_equipment_by_effects dptebe
//         INNER JOIN
//             equipable_places ep
//         ON
//             dptebe.equipable_place_id = ep.id
//         AND
//             dptebe.eqipment_id = e.id
//         INNER JOIN
//             parameters p
//         ON
//             dptebe.parameter_id = p.id        
//     ) dptebe
//     ON
//         e.id = dptebe.equipment_id
//     LEFT JOIN (
//         SELECT
//             ewiep.equipment_id ,
//             eqisp.equipable_place_id ,
//             ep.name ,
//             ep.display_order
//         FROM
//             equipped_when_increasing_equipable_places ewiep
//         INNER JOIN
//             equipable_places ep
//         ON
//             ewiep.equipable_place_id = ep.id
//         AND
//             ewiep.equipment_id = e.id
//     ) eqiep
//     ON
//         e.id = eqiep.equipment_id
//     LEFT JOIN (
//         SELECT
//             ewuep.equipment_id ,
//             ewuep.equipable_place_id ,
//             ep.name ,
//             ep.display_order
//         FROM
//             equipped_when_unequipping_equipable_places ewuep
//         INNER JOIN
//             equipable_places ep
//         ON
//             ewuep.equipable_place_id = ep.id
//         AND
//             ewuep.equipment_id = e.id
//     ) ewuep
//     ON
//         e.id = ewuep.equipment_id
//     LEFT JOIIN (
//         SELECT
//             eeiep.equipment_id ,
//             eeiep.equipable_place_id ,
//             ep.name ,
//             ep.display_order
//         FROM
//             equipments_equipable_in_equipable_places eeiep
//         INNER JOIN
//             equipable_places ep
//         ON
//             eeiep.equipable_place_id = ep.id
//         AND
//             eeiep.equipment_id = e.id
//     ) eeiep
//     ON
//         e.id = eeiep.equipment_id
//     ORDER BY
//         e.id ,
//         ee.parameter_id ,
//         dptebe.equipable_place_id ,
//         dptebe.parameter_id ,
//         ewiep.equipable_place_id ,
//         ewuep.equipable_place_id ,
//         eeiep.equipable_place_id
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // 装備登録API
// // from manager
// #[post("/api/v1/equipments")]
// async fn register_equipment_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Register equipments interface";
//     /*
//     INSERT INTO
//         equipments (
//             id ,
//             name ,
//             ruby ,
//             flavor ,
//             display_order ,
//             add_square_count
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipped_when_increasing_equipable_places (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipped_when_unequipping_equipable_places (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipments_equipable_in_equipable_places (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         designated_place_to_equipment_by_effects (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipment_effects (
//             equipment_id ,
//             parameter_id ,
//             num
//         )
//         VALUES (

//         )
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // 装備更新API
// // from manager
// #[put("/api/v1/equipments")]
// async fn update_equipment_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Update equipment";
//     /*
//     DELETE FROM
//         equipped_when_increasing_equipable_places
//     WHERE
//         equipment_id = 
//     */
//     /*
//     DELETE FROM
//         equipped_when_unequipping_equipable_places
//     WHERE
//         equipment_id = 
//     */
//     /*
//     DELETE FROM
//         equipments_equipable_in_equipable_places
//     WHERE
//         equipment_id =
//     */
//     /*
//     DELETE FROM
//         designated_place_to_equipment_by_effects
//     WHERE
//         equipment_id =
//     */
//     /*
//     DELETE FROM
//         equipment_effects
//     WHERE
//         equipment_id =
//     */
//     /*
//     UPDATE
//         equipments
//     SET
//         name = ,
//         ruby = ,
//         flavor = ,
//         display_order = ,
//         add_square_count = 
//     WHERE
//         id =
//     */
//     /*
//     INSERT INTO
//         equipped_when_increasing_equipable_places (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipped_when_unequipping_equipable_places (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipments_equipable_in_equipable_places (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         designated_place_to_equipment_by_effects (
//             equipment_id ,
//             equipable_place_id
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         equipment_effects (
//             equipment_id ,
//             parameter_id ,
//             num
//         )
//         VALUES (

//         )
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセット更新
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // 装備削除API
// // from manager
// #[delete("/api/v1/equipments")]
// async fn delete_equipment_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Delete equipment";
//     /*
//     DELETE FROM
//         equipped_when_increasing_equipable_places
//     WHERE
//         equipment_id = 
//     */
//     /*
//     DELETE FROM
//         equipped_when_unequipping_equipable_places
//     WHERE
//         equipment_id = 
//     */
//     /*
//     DELETE FROM
//         equipments_equipable_in_equipable_places
//     WHERE
//         equipment_id =
//     */
//     /*
//     DELETE FROM
//         designated_place_to_equipment_by_effects
//     WHERE
//         equipment_id =
//     */
//     /*
//     DELETE FROM
//         equipment_effects
//     WHERE
//         equipment_id =
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセット更新
//     */
//     /*
//     DELETE FROM
//         equipments
//     WHERE
//         id =
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }

// // パラメータ一覧取得API
// // from manager
// #[get("/api/v1/parameters")]
// async fn get_parameters_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Get parameters";
//     /*
//     SELECT
//         p.id ,
//         p.name ,
//         p.display_order
//     FROM
//         parameters p
//     ORDER BY
//         p.id
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // パラメータ登録API
// // from manager
// #[post("/api/v1/parameters")]
// async fn register_parameter_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Register parameter";
//     /*
//     INSERT INTO
//         parameters (
//             id ,
//             name ,
//             display_order
//         )
//         VALUES (

//         )
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // パラメータ更新API
// // from manager
// #[put("/api/v1/parameters")]
// async fn update_parameter_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Update parameter";
//     /*
//     UPDATE
//         parameters
//     SET
//         name = ,
//         display_order =
//     WHERE
//         id =
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // パラメータ削除API
// // from manager
// #[get("/api/v1/parameters")]
// async fn delete_parameter_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Delete parameter";
//     /*
//     DELETE FROM
//         body_free_squares
//     WHERE
//         parameter_id =
//     */
//     /*
//     DELETE FROM
//         body_effects
//     WHERE
//         parameter_id =
//     */
//     /*
//     DELETE FROM
//         designated_place_to_equipment_by_effects
//     WHERE
//         parameter_id = 
//     */
//     /*
//     DELETE FROM
//         equipment_effects
//     WHERE
//         parameter_id =
//     */
//     /*
//     DELETE FROM
//         parameter_chip_effects
//     WHERE
//         parameter_id =
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }

// // パラメータチップ一覧取得API
// // from manager
// #[get("/api/v1/parameter-chips")]
// async fn get_parameter_chips_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Get parameter chips";
//     /*
//     SELECT
//         pc.id ,
//         pc.name ,
//         pc.display_order ,
//         pcs.x ,
//         pcs.y ,
//         pce.parameter_id ,
//         pce.num ,
//         pce.name AS parameter_name ,
//         pce.display_order AS parameter_display_order
//     FROM
//         parameter_chips pc
//     LEFT JOIN
//         parameter_chip_squares pcs
//     ON
//         pc.id = pcs.parameter_chip_id
//     LEFT JOIN (
//         SELECT
//             pce.parameter_chip_id ,
//             pce.parameter_id ,
//             pce.num ,
//             p.name ,
//             p.display_order
//         FROM
//             parameter_chip_effects pce
//         INNER JOIN
//             parameters p
//         ON
//             p.id = pce.parameter_id
//         AND
//             pce.parameter_chip_id = pc.id
//     ) pce
//     ON
//         pc.id = pce.parameter_chip_id
//     ORDER BY
//         pc.id ,
//         pce.parameter_id ,
//         pcs.x ,
//         pcs.y
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // パラメータ登録API
// // from manager
// #[post("/api/v1/parameter-chips")]
// async fn register_parameter_chip_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Register parameter chip";
//     /*
//     INSERT INTO
//         parameter_chips (
//             id ,
//             name ,
//             display_order
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         parameter_chip_squares (
//             parameter_chip_id ,
//             x ,
//             y
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         parameter_chip_effects (
//             parameter_chip_id ,
//             parameter_id ,
//             num
//         )
//         VALUES (

//         )
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // パラメータ更新API
// // from manager
// #[put("/api/v1/parameter-chips")]
// async fn update_parameter_chip_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Update parameter chip";
//     /*
//     DELETE FROM
//         parameter_chip_squares
//     WHERE
//         parameter_chip_id =
//     */
//     /*
//     DELETE FROM
//         parameter_chip_effects
//     WHERE
//         parameter_chip_id =
//     */
//     /*
//     UPDATE
//         parameter_chips
//     SET
//         name = ,
//         display_order =
//     WHERE
//         id =
//     */
//     /*
//     INSERT INTO
//         parameter_chip_squares (
//             parameter_chip_id ,
//             x ,
//             y
//         )
//         VALUES (

//         )
//     */
//     /*
//     INSERT INTO
//         parameter_chip_effects (
//             parameter_chip_id ,
//             parameter_id ,
//             num
//         )
//         VALUES (

//         )
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセットを更新
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }
// // パラメータ削除API
// // from manager
// #[get("/api/v1/parameter-chips")]
// async fn delete_parameter_chip_from_manager() -> Result<HttpResponse, actix_web::Error> {
//     let response_body = "Delete parameter chip";
//     /*
//     DELETE FROM
//         parameter_chip_squares
//     WHERE
//         parameter_chip_id =
//     */
//     /*
//     DELETE FROM
//         parameter_chip_effects
//     WHERE
//         parameter_chip_id =
//     */
//     /*
//     DELETE FROM
//         parameter_chips
//     WHERE
//         id = 
//     */
//     // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
//     /*
//     更新後の情報をもとにマイセットを更新
//     */
//     return Ok(HttpResponse::Ok().body(response_body));
// }

// index
#[get("/")]
async fn index() -> impl Responder {
    let response_body = "Customize Game!";
    utils::establish_connection();
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ロボット取得API
#[get("/api/v1/manager/robots/{body_id}")]
async fn get_v1_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "get_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// ロボット一覧取得API
#[get("/api/v1/manager/robots")]
async fn get_v1_robots_from_manager() -> impl Responder {
    let response_body = "get_v1_robots_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ロボット登録API
#[post("/api/v1/manager/robots")]
async fn register_v1_robot_from_manager() -> impl Responder {
    let response_body = "register_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ロボット更新API
#[put("/api/v1/manager/robots/{body_id}")]
async fn update_v1_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "update_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// ロボット削除API
#[delete("/api/v1/manager/robots/{body_id}")]
async fn delete_v1_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "delete_v1_robot_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース取得API
#[get("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
async fn get_v1_hoge_interface_from_manager(web::Path(hoge_interface_id): web::Path<u32>) -> impl Responder {
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let response_body = "get_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: hoge_interface_id,
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース一覧取得API
#[get("/api/v1/manager/hoge-interfaces")]
async fn get_v1_hoge_interfaces_from_manager() -> impl Responder {
    let response_body = "get_v1_hoge_interfaces_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース登録API
#[post("/api/v1/manager/hoge-interfaces")]
async fn register_v1_hoge_interface_from_manager() -> impl Responder {
    let response_body = "register_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース更新API
#[put("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
async fn update_v1_hoge_interface_from_manager(web::Path(hoge_interface_id): web::Path<u32>) -> impl Responder {
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let response_body = "update_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: hoge_interface_id,
            text: String::from(response_body),
        }
    );
}

// hogeインタフェース削除API
#[delete("/api/v1/manager/hoge-interfaces/{hoge_interface_id}")]
async fn delete_v1_hoge_interface_from_manager(web::Path(hoge_interface_id): web::Path<u32>) -> impl Responder {
    let hoge_interface_id: Option<u32> = Some(hoge_interface_id);
    let response_body = "delete_v1_hoge_interface_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: hoge_interface_id,
            text: String::from(response_body),
        }
    );
}

// 装備取得API
#[get("/api/v1/manager/equipments/{equipment_id}")]
async fn get_v1_equipment_from_manager(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let response_body = "get_v1_equipment_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: equipment_id,
            text: String::from(response_body),
        }
    );
}

// 装備一覧取得API
#[get("/api/v1/manager/equipments")]
async fn get_v1_equipments_from_manager() -> impl Responder {
    let response_body = "get_v1_equipments_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// 装備登録API
#[post("/api/v1/manager/equipments")]
async fn register_v1_equipment_from_manager() -> impl Responder {
    let response_body = "register_v1_equipment_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// 装備更新API
#[put("/api/v1/manager/equipments/{equipment_id}")]
async fn update_v1_equipment_from_manager(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let response_body = "update_v1_equipment_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: equipment_id,
            text: String::from(response_body),
        }
    );
}

// 装備削除API
#[delete("/api/v1/manager/equipments/{equipment_id}")]
async fn delete_v1_equipment_from_manager(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let response_body = "delete_v1_equipment_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: equipment_id,
            text: String::from(response_body),
        }
    );
}

// パラメータ取得API
#[get("/api/v1/manager/parameters/{parameter_id}")]
async fn get_v1_parameter_from_manager(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    let parameter_id: Option<u32> = Some(parameter_id);
    let response_body = "get_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_id,
            text: String::from(response_body),
        }
    );
}

// パラメータ一覧取得API
#[get("/api/v1/manager/parameters")]
async fn get_v1_parameters_from_manager() -> impl Responder {
    let response_body = "get_v1_parameters_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータ登録API
#[post("/api/v1/manager/parameters")]
async fn register_v1_parameter_from_manager() -> impl Responder {
    let response_body = "register_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータ更新API
#[put("/api/v1/manager/parameters/{parameter_id}")]
async fn update_v1_parameter_from_manager(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    let parameter_id: Option<u32> = Some(parameter_id);
    let response_body = "update_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_id,
            text: String::from(response_body),
        }
    );
}

// パラメータ削除API
#[delete("/api/v1/manager/parameters/{parameter_id}")]
async fn delete_v1_parameter_from_manager(web::Path(parameter_id): web::Path<u32>) -> impl Responder {
    let parameter_id: Option<u32> = Some(parameter_id);
    let response_body = "delete_v1_parameter_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_id,
            text: String::from(response_body),
        }
    );
}

// パラメータチップ取得API
#[get("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
async fn get_v1_parameter_chip_from_manager(web::Path(parameter_chip_id): web::Path<u32>) -> impl Responder {
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let response_body = "get_v1_parameter_chip_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_chip_id,
            text: String::from(response_body),
        }
    );
}

// パラメータチップ一覧取得API
#[get("/api/v1/manager/parameter-chips")]
async fn get_v1_parameter_chips_from_manager() -> impl Responder {
    let response_body = "get_v1_parameter_chips_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータチップ登録API
#[post("/api/v1/manager/parameter-chips")]
async fn register_v1_parameter_chip_from_manager() -> impl Responder {
    let response_body = "register_v1_parameter_chip_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータチップ更新API
#[put("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
async fn update_v1_parameter_chip_from_manager(web::Path(parameter_chip_id): web::Path<u32>) -> impl Responder {
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let response_body = "update_v1_parameter_chip_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_chip_id,
            text: String::from(response_body),
        }
    );
}

// パラメータチップ削除API
#[delete("/api/v1/manager/parameter-chips/{parameter_chip_id}")]
async fn delete_v1_parameter_chip_from_manager(web::Path(parameter_chip_id): web::Path<u32>) -> impl Responder {
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let response_body = "delete_v1_parameter_chip_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_chip_id,
            text: String::from(response_body),
        }
    );
}

// ユーザ取得API
#[get("/api/v1/manager/users/{user_id}")]
async fn get_v1_user_from_manager(web::Path(user_id): web::Path<u32>) -> impl Responder {
    let user_id: Option<u32> = Some(user_id);
    let response_body = "get_v1_user_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: user_id,
            text: String::from(response_body),
        }
    );
}

// ユーザ一覧取得API
#[get("/api/v1/manager/users")]
async fn get_v1_users_from_manager() -> impl Responder {
    let response_body = "get_v1_users_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ更新API
#[put("/api/v1/manager/users/{user_id}")]
async fn update_v1_user_from_manager(web::Path(user_id): web::Path<u32>) -> impl Responder {
    let user_id: Option<u32> = Some(user_id);
    let response_body = "update_v1_user_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: user_id,
            text: String::from(response_body),
        }
    );
}

// ユーザ削除API
#[delete("/api/v1/manager/users/{user_id}")]
async fn delete_v1_user_from_manager(web::Path(user_id): web::Path<u32>) -> impl Responder {
    let user_id: Option<u32> = Some(user_id);
    let response_body = "delete_v1_user_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: user_id,
            text: String::from(response_body),
        }
    );
}

// マイセット取得API
#[get("/api/v1/manager/mysets/{mysets_id}")]
async fn get_v1_myset_from_manager(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "get_v1_myset_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット一覧取得API
#[get("/api/v1/manager/mysets")]
async fn get_v1_mysets_from_manager() -> impl Responder {
    let response_body = "get_v1_mysets_from_manager";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ロボット取得API
#[get("/api/v1/mobile/robots/{body_id}")]
async fn get_v1_robot_from_mobile(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "get_v1_robot_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// ロボット一覧取得API
#[get("/api/v1/mobile/robots")]
async fn get_v1_robots_from_mobile() -> impl Responder {
    let response_body = "get_v1_robots_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// 装備取得API
#[get("/api/v1/mobile/equipments/{equipment_id}")]
async fn get_v1_equipment_from_mobile(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let response_body = "get_v1_equipment_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: equipment_id,
            text: String::from(response_body),
        }
    );
}

// 装備一覧取得API
#[get("/api/v1/mobile/equipments")]
async fn get_v1_equipments_from_mobile() -> impl Responder {
    let response_body = "get_v1_equipments_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータチップ取得API
#[get("/api/v1/mobile/parameter-chips/{parameter_chip_id}")]
async fn get_v1_parameter_chip_from_mobile(web::Path(parameter_chip_id): web::Path<u32>) -> impl Responder {
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let response_body = "get_v1_parameter_chip_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_chip_id,
            text: String::from(response_body),
        }
    );
}

// パラメータチップ一覧取得API
#[get("/api/v1/mobile/parameter-chips")]
async fn get_v1_parameter_chips_from_mobile() -> impl Responder {
    let response_body = "get_v1_parameter_chips_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ取得API
#[get("/api/v1/mobile/users/myself")]
async fn get_v1_user_from_mobile() -> impl Responder {
    let response_body = "get_v1_user_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ登録API
#[post("/api/v1/mobile/users")]
async fn register_v1_user_from_mobile() -> impl Responder {
    let response_body = "register_v1_user_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット取得API
#[get("/api/v1/mobile/mysets/{mysets_id}")]
async fn get_v1_myset_from_mobile(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "get_v1_myset_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット一覧取得API
#[get("/api/v1/mobile/mysets/myself")]
async fn get_v1_mysets_from_mobile() -> impl Responder {
    let response_body = "get_v1_mysets_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット登録API
#[post("/api/v1/mobile/mysets")]
async fn register_v1_myset_from_mobile() -> impl Responder {
    let response_body = "register_v1_myset_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット更新API
#[put("/api/v1/mobile/mysets/{mysets_id}")]
async fn update_v1_myset_from_mobile(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "update_v1_myset_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット削除API
#[delete("/api/v1/mobile/mysets/{mysets_id}")]
async fn delete_v1_myset_from_mobile(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "delete_v1_myset_from_mobile";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// ロボット取得API
#[get("/api/v1/game/robots/{body_id}")]
async fn get_v1_robot_from_game(web::Path(body_id): web::Path<u32>) -> impl Responder {
    let body_id: Option<u32> = Some(body_id);
    let response_body = "get_v1_robot_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from(response_body),
        }
    );
}

// ロボット一覧取得API
#[get("/api/v1/game/robots/myself")]
async fn get_v1_robots_from_game() -> impl Responder {
    let response_body = "get_v1_robots_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// 装備取得API
#[get("/api/v1/game/equipments/{equipment_id}")]
async fn get_v1_equipment_from_game(web::Path(equipment_id): web::Path<u32>) -> impl Responder {
    let equipment_id: Option<u32> = Some(equipment_id);
    let response_body = "get_v1_equipment_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: equipment_id,
            text: String::from(response_body),
        }
    );
}

// 装備一覧取得API
#[get("/api/v1/game/equipments")]
async fn get_v1_equipments_from_game() -> impl Responder {
    let response_body = "get_v1_equipments_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// パラメータチップ取得API
#[get("/api/v1/game/parameter-chips/{parameter_chip_id}")]
async fn get_v1_parameter_chip_from_game(web::Path(parameter_chip_id): web::Path<u32>) -> impl Responder {
    let parameter_chip_id: Option<u32> = Some(parameter_chip_id);
    let response_body = "get_v1_parameter_chip_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: parameter_chip_id,
            text: String::from(response_body),
        }
    );
}

// パラメータチップ一覧取得API
#[get("/api/v1/game/parameter-chips")]
async fn get_v1_parameter_chips_from_game() -> impl Responder {
    let response_body = "get_v1_parameter_chips_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ取得API
#[get("/api/v1/game/users/myself")]
async fn get_v1_user_from_game() -> impl Responder {
    let response_body = "get_v1_user_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// ユーザ登録API
#[post("/api/v1/game/users")]
async fn register_v1_user_from_game() -> impl Responder {
    let response_body = "register_v1_user_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット取得API
#[get("/api/v1/game/mysets/{mysets_id}")]
async fn get_v1_myset_from_game(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "get_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット一覧取得API
#[get("/api/v1/game/mysets")]
async fn get_v1_mysets_from_game() -> impl Responder {
    let response_body = "get_v1_mysets_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット登録API
#[post("/api/v1/game/mysets")]
async fn register_v1_myset_from_game() -> impl Responder {
    let response_body = "register_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: Some(999),
            text: String::from(response_body),
        }
    );
}

// マイセット更新API
#[put("/api/v1/game/mysets/{mysets_id}")]
async fn update_v1_myset_from_game(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "update_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

// マイセット削除API
#[delete("/api/v1/game/mysets/{mysets_id}")]
async fn delete_v1_myset_from_game(web::Path(mysets_id): web::Path<u32>) -> impl Responder {
    let mysets_id: Option<u32> = Some(mysets_id);
    let response_body = "delete_v1_myset_from_game";
    return HttpResponse::Ok().json(
        DataEntry {
            id: mysets_id,
            text: String::from(response_body),
        }
    );
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(
        || App::new()
            .service(index)

            .service(get_v1_robot_from_manager)
            .service(get_v1_robots_from_manager)
            .service(register_v1_robot_from_manager)
            .service(update_v1_robot_from_manager)
            .service(delete_v1_robot_from_manager)

            .service(get_v1_hoge_interface_from_manager)
            .service(get_v1_hoge_interfaces_from_manager)
            .service(register_v1_hoge_interface_from_manager)
            .service(update_v1_hoge_interface_from_manager)
            .service(delete_v1_hoge_interface_from_manager)

            .service(get_v1_equipment_from_manager)
            .service(get_v1_equipments_from_manager)
            .service(register_v1_equipment_from_manager)
            .service(update_v1_equipment_from_manager)
            .service(delete_v1_equipment_from_manager)

            .service(get_v1_parameter_from_manager)
            .service(get_v1_parameters_from_manager)
            .service(register_v1_parameter_from_manager)
            .service(update_v1_parameter_from_manager)
            .service(delete_v1_parameter_from_manager)

            .service(get_v1_parameter_chip_from_manager)
            .service(get_v1_parameter_chips_from_manager)
            .service(register_v1_parameter_chip_from_manager)
            .service(update_v1_parameter_chip_from_manager)
            .service(delete_v1_parameter_chip_from_manager)

            .service(get_v1_user_from_manager)
            .service(get_v1_users_from_manager)
            .service(update_v1_user_from_manager)
            .service(delete_v1_user_from_manager)

            .service(get_v1_myset_from_manager)
            .service(get_v1_mysets_from_manager)
            
            .service(get_v1_robot_from_mobile)
            .service(get_v1_robots_from_mobile)

            .service(get_v1_equipment_from_mobile)
            .service(get_v1_equipments_from_mobile)

            .service(get_v1_parameter_chip_from_mobile)
            .service(get_v1_parameter_chips_from_mobile)

            .service(get_v1_user_from_mobile)
            .service(register_v1_user_from_mobile)

            .service(get_v1_myset_from_mobile)
            .service(get_v1_mysets_from_mobile)
            .service(update_v1_myset_from_mobile)
            .service(delete_v1_myset_from_mobile)

            .service(get_v1_robot_from_game)
            .service(get_v1_robots_from_game)

            .service(get_v1_equipment_from_game)
            .service(get_v1_equipments_from_game)

            .service(get_v1_parameter_chip_from_game)
            .service(get_v1_parameter_chips_from_game)

            .service(get_v1_user_from_game)
            .service(register_v1_user_from_game)

            .service(get_v1_myset_from_game)
            .service(get_v1_mysets_from_game)
            .service(update_v1_myset_from_game)
            .service(delete_v1_myset_from_game)

    )
    .bind("0.0.0.0:5000")?
    .run()
    .await?;
    Ok(())
}
