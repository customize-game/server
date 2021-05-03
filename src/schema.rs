table! {
    bodies (id) {
        id -> Integer,
        name -> Varchar,
        ruby -> Nullable<Varchar>,
        flavor -> Nullable<Text>,
        display_order -> Integer,
        is_deleted -> Tinyint,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    bodies_hoge_interfaces (body_id, hoge_interface_id) {
        body_id -> Integer,
        hoge_interface_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    body_free_sockets (body_id, x, y) {
        body_id -> Integer,
        x -> Integer,
        y -> Integer,
        operator -> Nullable<Varchar>,
        num -> Nullable<Integer>,
        version -> Integer,
    }
}

table! {
    body_statuses (body_id, parameter_id) {
        body_id -> Integer,
        parameter_id -> Integer,
        num -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    designated_place_to_equipment_by_effects (equipment_id, hoge_intarface_id, parameter_id) {
        equipment_id -> Integer,
        hoge_intarface_id -> Integer,
        parameter_id -> Integer,
        num -> Nullable<Integer>,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    equipment_mysets (myset_id, hoge_interface_id, equipment_id) {
        myset_id -> Integer,
        hoge_interface_id -> Integer,
        equipment_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    equipment_status (equipment_id, parameter_id) {
        equipment_id -> Integer,
        parameter_id -> Integer,
        num -> Nullable<Integer>,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    equipments (id) {
        id -> Integer,
        name -> Varchar,
        ruby -> Nullable<Varchar>,
        flavor -> Nullable<Text>,
        add_socket_count -> Nullable<Integer>,
        display_order -> Nullable<Integer>,
        is_deleted -> Nullable<Tinyint>,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    equipments_equipable_in_hoge_interfaces (equipment_id, hoge_interface_id) {
        equipment_id -> Integer,
        hoge_interface_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    equipped_when_increasing_hoge_interfaces (equipment_id, hoge_interface_id) {
        equipment_id -> Integer,
        hoge_interface_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    equipped_when_unequipping_hoge_interfaces (equipment_id, equipped_hoge_intarface_id, unequipping_hoge_intarface_id) {
        equipment_id -> Integer,
        equipped_hoge_intarface_id -> Integer,
        unequipping_hoge_intarface_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    having_bodies (user_id, body_id) {
        user_id -> Integer,
        body_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    having_equipments (user_id, equipment_id) {
        user_id -> Integer,
        equipment_id -> Integer,
        count -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    having_parameter_chips (user_id, parameter_chip_id) {
        user_id -> Integer,
        parameter_chip_id -> Integer,
        count -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    hoge_interfaces (id) {
        id -> Integer,
        name -> Varchar,
        display_order -> Integer,
        version -> Integer,
    }
}

table! {
    hoge_sockets (parameter_chip_id, x, y) {
        parameter_chip_id -> Integer,
        x -> Integer,
        y -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    mysets (id) {
        id -> Integer,
        name -> Varchar,
        user_id -> Integer,
        body_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    parameter_chip_effects (parameter_chip_id, parameter_id) {
        parameter_chip_id -> Integer,
        parameter_id -> Integer,
        num -> Nullable<Integer>,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    parameter_chip_mysets (myset_id, parameter_chip_id) {
        myset_id -> Integer,
        parameter_chip_id -> Integer,
        x -> Integer,
        y -> Integer,
        angle -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    parameter_chips (id) {
        id -> Integer,
        name -> Varchar,
        display_order -> Integer,
        is_deleted -> Tinyint,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    parameters (id) {
        id -> Integer,
        name -> Varchar,
        display_order -> Integer,
        version -> Integer,
    }
}

table! {
    participants (result_id, user_id) {
        result_id -> Integer,
        user_id -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    results (id) {
        id -> Integer,
        start_date -> Datetime,
        end_date -> Datetime,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        exp -> Integer,
        created_datetime -> Datetime,
        updated_datetime -> Datetime,
        version -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(
    bodies,
    bodies_hoge_interfaces,
    body_free_sockets,
    body_statuses,
    designated_place_to_equipment_by_effects,
    equipment_mysets,
    equipment_status,
    equipments,
    equipments_equipable_in_hoge_interfaces,
    equipped_when_increasing_hoge_interfaces,
    equipped_when_unequipping_hoge_interfaces,
    having_bodies,
    having_equipments,
    having_parameter_chips,
    hoge_interfaces,
    hoge_sockets,
    mysets,
    parameter_chip_effects,
    parameter_chip_mysets,
    parameter_chips,
    parameters,
    participants,
    results,
    users,
);
