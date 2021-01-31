USE customize_game;

ALTER TABLE equipments MODIFY COLUMN add_socket_count int COMMENT '装備時に増えるソケット';
ALTER TABLE equipments MODIFY COLUMN display_order int COMMENT '表示順';
ALTER TABLE equipments MODIFY COLUMN is_deleted tinyint DEFAULT 0 COMMENT '削除フラグ';

ALTER TABLE mysets DROP COLUMN display_order;
