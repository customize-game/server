DROP DATABASE IF EXISTS player_info;
CREATE DATABASE player_info;
USE player_info;

CREATE TABLE parts_data(
  id INT PRIMARY KEY AUTO_INCREMENT,
  name TEXT NOT NULL
);
