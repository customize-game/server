ALTER TABLE parameters ADD parameter_type enum( 'calculation', 'having' ) NOT NULL COMMENT '種別' AFTER name;