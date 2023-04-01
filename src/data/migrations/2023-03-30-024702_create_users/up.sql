-- Your SQL goes here
CREATE TABLE users (
  id BINARY(16) NOT NULL,
  google_id VARCHAR(255) NOT NULL,
  provider VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  photo VARCHAR(255) NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT 0,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  UNIQUE KEY (google_id)
);

CREATE 
  ALGORITHM = UNDEFINED 
  DEFINER = `root`@`localhost` 
  SQL SECURITY DEFINER
VIEW `listly`.`user_w_readable_id` AS
  SELECT 
    HEX(`listly`.`users`.`id`) AS `id_hex`,
    BIN_TO_UUID(`listly`.`users`.`id`) AS `id_readable`,
    `listly`.`users`.`google_id` AS `google_id`,
    `listly`.`users`.`provider` AS `provider`,
    `listly`.`users`.`email` AS `email`,
    `listly`.`users`.`first_name` AS `first_name`,
    `listly`.`users`.`last_name` AS `last_name`,
    `listly`.`users`.`photo` AS `photo`,
    `listly`.`users`.`verified` AS `verified`,
    `listly`.`users`.`created_at` AS `created_at`,
    `listly`.`users`.`updated_at` AS `updated_at`
  FROM
    `listly`.`users`