-- 素体：hogeインタフェース
CREATE TABLE bodies_hoge_interfaces (
  body_id INT NOT NULL COMMENT '素体ID'
  , hoge_interface_id INT NOT NULL COMMENT 'hogeインタフェースID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT bodies_hoge_interfaces_PKC PRIMARY KEY (body_id,hoge_interface_id)
) COMMENT '素体：hogeインタフェース:素体が持つhogeインタフェース一覧' ;

-- 素体の空きソケット
CREATE TABLE body_free_sockets (
  body_id INT NOT NULL COMMENT '素体ID'
  , x INT NOT NULL COMMENT 'X座標'
  , y INT NOT NULL COMMENT 'Y座標'
  , operator ENUM('plus','minus','multi','div') COMMENT '演算子:plus:足し算 minus:引き算 multi:掛け算 div:割り算'
  , num INT COMMENT '増減値'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT body_free_sockets_PKC PRIMARY KEY (body_id,x,y)
) COMMENT '素体の空きソケット:素体が持つパラメータチップを埋められる空きソケット' ;

-- 素体ステータス
CREATE TABLE body_statuses (
  body_id INT NOT NULL COMMENT '素体ID'
  , parameter_id INT NOT NULL COMMENT 'パラメータID'
  , num INT DEFAULT 0 NOT NULL COMMENT '増減値'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT body_statuses_PKC PRIMARY KEY (body_id,parameter_id)
) COMMENT '素体ステータス:素体自体が持つステータス' ;

-- 指定箇所への装備による効果
CREATE TABLE designated_place_to_equipment_by_effects (
  equipment_id INT NOT NULL COMMENT '装備ID'
  , hoge_intarface_id INT NOT NULL COMMENT 'hogeインタフェースID'
  , parameter_id INT NOT NULL COMMENT 'パラメータID'
  , num INT COMMENT '増減値'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT designated_place_to_equipment_by_effects_PKC PRIMARY KEY (equipment_id,hoge_intarface_id,parameter_id)
) COMMENT '指定箇所への装備による効果' ;

-- 装備マイセット
CREATE TABLE equipment_mysets (
  myset_id INT NOT NULL COMMENT 'マイセットID'
  , hoge_interface_id INT NOT NULL COMMENT 'hogeインタフェースID'
  , equipment_id INT NOT NULL COMMENT '装備ID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT equipment_mysets_PKC PRIMARY KEY (myset_id,hoge_interface_id,equipment_id)
) COMMENT '装備マイセット' ;

-- 装備ステータス
CREATE TABLE equipment_status (
  equipment_id INT NOT NULL COMMENT '装備ID'
  , parameter_id INT NOT NULL COMMENT 'パラメータID'
  , num INT COMMENT '増減値'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT equipment_status_PKC PRIMARY KEY (equipment_id,parameter_id)
) COMMENT '装備ステータス' ;

-- 装備すると増えるhogeインタフェース
CREATE TABLE equipped_when_increasing_hoge_interfaces (
  equipment_id INT NOT NULL COMMENT '装備ID'
  , hoge_interface_id INT NOT NULL COMMENT 'hogeインタフェースID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT equipped_when_increasing_hoge_interfaces_PKC PRIMARY KEY (equipment_id,hoge_interface_id)
) COMMENT '装備すると増えるhogeインタフェース' ;

-- 装備すると装備できなくなるhogeインタフェース
CREATE TABLE equipped_when_unequipping_hoge_interfaces (
  equipment_id INT NOT NULL COMMENT '装備ID'
  , equipped_hoge_intarface_id INT NOT NULL COMMENT '装備したhogeインタフェースID'
  , unequipping_hoge_intarface_id INT NOT NULL COMMENT '装備できなくなるhogeインタフェースID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT equipped_when_unequipping_hoge_interfaces_PKC PRIMARY KEY (equipment_id,equipped_hoge_intarface_id,unequipping_hoge_intarface_id)
) COMMENT '装備すると装備できなくなるhogeインタフェース' ;

-- 所持している素体
CREATE TABLE having_bodies (
  user_id INT NOT NULL COMMENT 'ユーザID'
  , body_id INT NOT NULL COMMENT '素体ID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT having_bodies_PKC PRIMARY KEY (user_id,body_id)
) COMMENT '所持している素体' ;

-- 所持している装備
CREATE TABLE having_equipments (
  user_id INT NOT NULL COMMENT 'ユーザID'
  , equipment_id INT NOT NULL COMMENT '装備ID'
  , count INT DEFAULT 0 NOT NULL COMMENT '所持数'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT having_equipments_PKC PRIMARY KEY (user_id,equipment_id)
) COMMENT '所持している装備' ;

-- 所持しているパラメータチップ
CREATE TABLE having_parameter_chips (
  user_id INT NOT NULL COMMENT 'ユーザID'
  , parameter_chip_id INT NOT NULL COMMENT 'パラメータチップID'
  , count INT DEFAULT 0 NOT NULL COMMENT '所持数'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT having_parameter_chips_PKC PRIMARY KEY (user_id,parameter_chip_id)
) COMMENT '所持しているパラメータチップ' ;

-- hogeソケット
CREATE TABLE hoge_sockets (
  parameter_chip_id INT NOT NULL COMMENT 'パラメータチップID'
  , x INT NOT NULL COMMENT 'X座標'
  , y INT NOT NULL COMMENT 'Y座標'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT hoge_sockets_PKC PRIMARY KEY (parameter_chip_id,x,y)
) COMMENT 'hogeソケット:hogeソケット' ;

-- パラメータチップの効果
CREATE TABLE parameter_chip_effects (
  parameter_chip_id INT NOT NULL COMMENT 'パラメータチップID'
  , parameter_id INT NOT NULL COMMENT 'パラメータID'
  , num INT COMMENT '増減値'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT parameter_chip_effects_PKC PRIMARY KEY (parameter_chip_id,parameter_id)
) COMMENT 'パラメータチップの効果:パラメータチップの効果' ;

-- パラメータチップマイセット
CREATE TABLE parameter_chip_mysets (
  myset_id INT NOT NULL COMMENT 'マイセットID'
  , parameter_chip_id INT NOT NULL COMMENT 'パラメータチップID'
  , x INT NOT NULL COMMENT 'X座標:パラメータチップのソケット(0,0)が設定されるX座標'
  , y INT NOT NULL COMMENT 'Y座標:パラメータチップのソケット(0,0)が設定されるY座標'
  , angle INT NOT NULL COMMENT '角度:デフォルトの向きを0として右回転の順に90,180,270'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT parameter_chip_mysets_PKC PRIMARY KEY (myset_id,parameter_chip_id)
) COMMENT 'パラメータチップマイセット' ;

-- パラメータチップ
CREATE TABLE parameter_chips (
  id INT NOT NULL COMMENT 'パラメータチップID'
  , name VARCHAR(50) NOT NULL COMMENT 'パラメータチップ名'
  , display_order INT NOT NULL COMMENT '表示順'
  , is_deleted TINYINT DEFAULT 0 NOT NULL COMMENT '削除フラグ'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT parameter_chips_PKC PRIMARY KEY (id)
) COMMENT 'パラメータチップ:パラメータチップ' ;

-- パラメータ
CREATE TABLE parameters (
  id INT NOT NULL COMMENT 'パラメータID'
  , name VARCHAR(50) NOT NULL COMMENT 'パラメータ名'
  , display_order INT NOT NULL COMMENT '表示順'
  , is_deleted TINYINT DEFAULT 0 NOT NULL COMMENT '削除フラグ'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT parameters_PKC PRIMARY KEY (id)
) COMMENT 'パラメータ:パラメータ' ;

-- 参加者
CREATE TABLE participants (
  result_id INT NOT NULL COMMENT '戦績ID'
  , user_id INT NOT NULL COMMENT 'ユーザID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT participants_PKC PRIMARY KEY (result_id,user_id)
) COMMENT '参加者' ;

-- 戦績
CREATE TABLE results (
  id INT NOT NULL COMMENT '戦績ID'
  , start_date DATETIME NOT NULL COMMENT '開始日時'
  , end_date DATETIME NOT NULL COMMENT '終了日時'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT results_PKC PRIMARY KEY (id)
) COMMENT '戦績' ;

-- hogeインタフェースに装備できる装備
CREATE TABLE equipments_equipable_in_hoge_interfaces (
  equipment_id INT NOT NULL COMMENT '装備ID'
  , hoge_interface_id INT NOT NULL COMMENT 'hogeインタフェースID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT equipments_equipable_in_hoge_interfaces_PKC PRIMARY KEY (equipment_id,hoge_interface_id)
) COMMENT 'hogeインタフェースに装備できる装備' ;

-- hogeインタフェース
CREATE TABLE hoge_interfaces (
  id INT NOT NULL COMMENT 'hogeインタフェースID'
  , name VARCHAR(50) NOT NULL COMMENT 'hogeインタフェース名'
  , display_order INT NOT NULL COMMENT '表示順'
  , is_deleted TINYINT DEFAULT 0 NOT NULL COMMENT '削除フラグ'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT hoge_interfaces_PKC PRIMARY KEY (id)
) COMMENT 'hogeインタフェース:hogeインターフェース' ;

-- マイセット
CREATE TABLE mysets (
  id INT NOT NULL COMMENT 'マイセットID'
  , name VARCHAR(50) NOT NULL COMMENT 'マイセット名'
  , user_id INT NOT NULL COMMENT 'ユーザID'
  , body_id INT NOT NULL COMMENT '素体ID'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT mysets_PKC PRIMARY KEY (id)
) COMMENT 'マイセット' ;

-- ユーザ
CREATE TABLE users (
  id INT NOT NULL COMMENT 'ユーザID'
  , exp INT DEFAULT 0 NOT NULL COMMENT '経験値'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT users_PKC PRIMARY KEY (id)
) COMMENT 'ユーザ' ;

-- 素体
CREATE TABLE bodies (
  id INT NOT NULL COMMENT '素体ID'
  , name VARCHAR(50) NOT NULL COMMENT '素体名'
  , ruby VARCHAR(50) COMMENT '素体名ルビ'
  , flavor TEXT COMMENT 'フレーバーテキスト'
  , display_order INT NOT NULL COMMENT '表示順'
  , is_deleted TINYINT DEFAULT 0 NOT NULL COMMENT '削除フラグ'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT bodies_PKC PRIMARY KEY (id)
) COMMENT '素体:素体' ;

-- 装備
CREATE TABLE equipments (
  id INT NOT NULL COMMENT '装備ID'
  , name VARCHAR(50) NOT NULL COMMENT '装備名'
  , ruby VARCHAR(50) COMMENT '装備名ルビ'
  , flavor TEXT COMMENT 'フレーバーテキスト'
  , add_socket_count INT COMMENT '装備時に増えるソケット'
  , display_order INT COMMENT '表示順'
  , is_deleted TINYINT DEFAULT 0 COMMENT '削除フラグ'
  , created_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '作成日時'
  , updated_datetime DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL COMMENT '更新日時'
  , version INT DEFAULT 0 NOT NULL COMMENT 'バージョン'
  , CONSTRAINT equipments_PKC PRIMARY KEY (id)
) COMMENT '装備' ;
