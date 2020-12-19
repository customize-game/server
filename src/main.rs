use actix_web::{ web, get, post, put, delete, App, HttpResponse, HttpRequest, HttpServer , Responder};
use serde::{Serialize, Deserialize};

mod utils;

#[derive(Serialize, Deserialize, Debug)]
struct DataEntry {
    id: Option<u32>,
    text: String,
}

struct Template {
    entries: Vec<DataEntry>,
}

#[get("/")]
async fn index() ->Result<HttpResponse, actix_web::Error >{
    let response_body = "Customize Game!";
    utils::establish_connection();
    return Ok(HttpResponse::Ok().body(response_body));
}

// ロボット取得API
// from manager
#[get("/api/robots/{body_id}")]
async fn get_robot_from_manager(web::Path(body_id): web::Path<u32>) -> impl Responder{
    let body_id: Option<u32> = Some(body_id);
    let response_body = body_id;
    utils::establish_connection();
    HttpResponse::Ok().json(
        DataEntry {
            id: body_id,
            text: String::from("sample"),
        }
    )
}
// ロボット一覧取得API
// from manager
#[get("/api/robots")]
async fn get_robots_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Get robots";
    /*
    SELECT
        b.id ,
        b.name ,
        b.flavor ,
        b.display_order ,
        bep.equipable_place_id ,
        bep.name ,
        bep.display_order ,
        bfs.x ,
        bfs.y ,
        be.parameter_id ,
        be.name ,
        be.display_order ,
        be.num        
    FROM
        bodies b
    LEFT JOIN (
        SELECT
            bep.equipable_place_id ,
            ep.name ,
            ep.display_order ,
            bep.body_id
        FROM
            bodies_equipable_places bep
        INNER JOIN
            equipable_places ep
        ON
            bep.equipable_place_id = ep.id
        AND
            bep.body_id = b.id
    ) bep
    ON
        b.id = bep.body_id
    LEFT JOIN
        body_free_squares bfs
    ON
        b.id = bfs.body_id
    LEFT JOIN (
        SELECT
            p.parameter_id ,
            p.name ,
            p.display_order ,
            be.body_id ,
            be.num
        FROM
            body_effects be
        INNER JOIN
            parameters p
        ON
            be.parameter_id = p.id
        AND
            be.body_id = b.id
    ) be
    ON
        b.id = be.id
    ORDER BY
        b.id ,
        bep.equipable_place_id ,
        be.parameter_id ,
        bfs.x ,
        bfs.y
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// ロボット登録API
// from manager
#[post("/api/robots")]
async fn register_robot_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Register robot";
    /*
    INSERT INTO
        bodies (
            id ,
            name ,
            ruby ,
            flavor ,
            display_order
        )
        VALUES (

        )
    */
    /*
    INSERT INTO 
        bodies_equipable_places (
            body_id ,
            equipable_place_id
        )
        VALUES (

        )
    */
    /*
    INSERT INTO 
        body_free_squares (
            body_id ,
            x ,
            y
        )
        VALUES (

        )
    */
    /*
    INSERT INTO 
        body_effects (
            body_id ,
            parameter_id ,
            num
        )
        VALUES (

        )        
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// ロボット更新API
// from manager
#[put("/api/robots")]
async fn update_robot_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Update robot";
    /*
    DELETE FROM
        bodies_equipable_places
    WHERE
        body_id =
    */
    /*
    DELETE FROM
        body_free_squares
    WHERE
        body_id =
    */
    /*
    DELETE FROM
        body_effects
    WHERE
        body_id =
    */
    /*
    UPDATE
        bodies
    SET
        name = ,
        ruby = ,
        flavor = ,
        display_order = ,
    WHERE
        id =
    */
    /*
    INSERT INTO 
        bodies_equipable_places (
            body_id ,
            equipable_place_id
        )
        VALUES (

        )
    */
    /*
    INSERT INTO 
        body_free_squares (
            body_id ,
            x ,
            y
        )
        VALUES (

        )
    */
    /*
    INSERT INTO
        body_effects (
            body_id ,
            parameter_id ,
            num
        )
        VALUES (

        )     
    */
    // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
    /*
    更新後の情報をもとにマイセットを更新
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// ロボット削除API
// from manager
#[get("/api/robots")]
async fn delete_robot_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Delete robot";
    /*
    DELETE FROM
        bodies_equipable_places
    WHERE
        body_id =
    */
    /*
    DELETE FROM
        body_free_squares
    WHERE
        body_id =
    */
    /*
    DELETE FROM
        body_effects
    WHERE
        body_id =
    */
    /*
    DELETE FROM
        having_bodies
    WHERE
        body_id =
    */
    // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
    /*
    更新後の情報をもとにマイセット更新
    */
    /*
    DELETE FROM
        bodies
    WHERE
        id = 
    */
    return Ok(HttpResponse::Ok().body(response_body));
}

// 装備可能箇所一覧取得API
// from manager
#[get("/api/hoge-interfaces")]
async fn get_hoge_interfaces_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Get hoge interfaces";
    /*
    SELECT
        ep.id ,
        ep.name
    FROM
        equipable_places ep
    ORDER BY
        ep.id
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// 装備可能箇所登録API
// from manager
#[post("/api/hoge-interfaces")]
async fn register_hoge_interface_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Register hoge interface";
    /*
    INSERT INTO
        equipable_places (
            id ,
            name ,
            display_order
        )
        VALUES (

        )
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// 装備可能箇所更新API
// from manager
#[put("/api/hoge-interfaces")]
async fn update_hoge_interface_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Update hoge interface";
    /*
    UPDATE
        equipable_places
    SET
        name = ,
        display_order =
    WHERE
        id =
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// 装備可能箇所削除API
// from manager
#[delete("/api/hoge-interfaces")]
async fn delete_hoge_interface_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Delete hoge interface";
    /*
    DELETE FROM
        bodies_equipable_places
    WHERE
        equipable_place_id =
    */
    /*
    DELETE FROM
        equipped_when_increasing_equipable_places
    WHERE
        equipable_place_id =
    */
    /*
    DELETE FROM
        equipped_when_unequipping_equipable_places
    WHERE
        equipable_place_id =
    */
    /*
    DELETE FROM
        equipments_equipable_in_equipable_places
    WHERE
        equipable_place_id =
    */
    /*
    DELETE FROM
        designated_place_to_equipment_by_effects
    WHERE
        equipable_place_id =
    */
    // 既に更新前の状態でマイセットを作成している場合、新規仕様と構成がずれるので修正
    /*
    更新後の情報をもとにマイセット更新
    */
    /*
    DELETE FROM
        equipable_places
    WHERE
        id =
    */
    return Ok(HttpResponse::Ok().body(response_body));
}

// 装備一覧取得API
// from manager
#[get("/api/equipments")]
async fn get_equipments_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Get equipments interfaces";
    /*
    SELECT
        e.id ,
        e.name ,
        e.ruby ,
        e.flavor ,
        e.display_order ,
        e.add_square_count ,
        ee.parameter_id ,
        ee.name ,
        ee.num ,
        ee.display_order ,
        dptebe.equipable_place_id ,
        dptebe.parameter_id ,
        dptebe.num ,
        dptebe.equipable_place_name ,
        dptebe.equipable_place_display_order ,
        dptebe.parameter_name ,
        dptebe.parameter_display_order ,

    FROM
        equipments e
    LEFT JOIN (
        SELECT
            ee.equipment_id ,
            ee.parameter_id ,
            ee.num ,
            p.name ,
            p.display_order
        FROM
            equipment_effects ee
        INNER JOIN
            parameters p
        ON
            ee.parameter_id = p.id
        AND
            ee.equipment_id = ee.parameter_id
    ) ee
    ON
        e.id = ee.equipment_id
    LEFT JOIN (
        SELECT
            dptebe.equipment_id ,
            dptebe.equipable_place_id ,
            dptebe.parameter_id ,
            dptebe.num ,
            ep.name AS equipable_place_name ,
            ep.display_order AS equipable_place_display_order ,
            p.name AS parameter_name ,
            p.display_order AS parameter_display_order
        FROM
            designated_place_to_equipment_by_effects dptebe
        INNER JOIN
            equipable_places ep
        ON
            dptebe.equipable_place_id = ep.id
        AND
            dptebe.eqipment_id = e.id
        INNER JOIN
            parameters p
        ON
            dptebe.parameter_id = p.id        
    ) dptebe
    ON
        e.id = dptebe.equipment_id


        ~~~
    
    */
    return Ok(HttpResponse::Ok().body(response_body));
}
// 装備登録API
// from manager
#[post("/api/equipments")]
async fn register_equipment_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Register equipments interface";
    return Ok(HttpResponse::Ok().body(response_body));
}
// 装備更新API
// from manager
#[put("/api/equipments")]
async fn update_equipment_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Update equipment";
    return Ok(HttpResponse::Ok().body(response_body));
}
// 装備削除API
// from manager
#[delete("/api/equipments")]
async fn delete_equipment_from_manager() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Delete equipment";
    return Ok(HttpResponse::Ok().body(response_body));
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || 
    App::new()
        .service(index)

        .service(get_robot_from_manager)
        .service(get_robots_from_manager)
        .service(register_robot_from_manager)
        .service(update_robot_from_manager)
        .service(delete_robot_from_manager)

        .service(get_hoge_interfaces_from_manager)
        .service(register_hoge_interface_from_manager)
        .service(update_hoge_interface_from_manager)
        .service(delete_hoge_interface_from_manager)

        .service(get_equipments_from_manager)
        .service(register_equipment_from_manager)
        .service(update_equipment_from_manager)
        .service(delete_equipment_from_manager)
    )
    .bind("0.0.0.0:5000")?
    .run()
    .await?;
    Ok(())
}
