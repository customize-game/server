USE customize_game;

ALTER TABLE equipments MODIFY COLUMN add_socket_count int NOT NULL COMMENT '装備時に増えるソケット';
ALTER TABLE equipments MODIFY COLUMN display_order int NOT NULL COMMENT '表示順';
ALTER TABLE equipments MODIFY COLUMN is_deleted tinyint NOT NULL COMMENT '削除フラグ' DEFAULT 0;

ALTER TABLE mysets ADD display_order int NOT NULL COMMENT '表示順' AFTER body_id;
