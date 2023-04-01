-- Your SQL goes here
CREATE TABLE users (
  id BINARY(16) NOT NULL,
  google_id VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  first_name VARCHAR(255) NOT NULL,
  last_name VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  role VARCHAR(255) NOT NULL,
  photo VARCHAR(255) NOT NULL,
  verified BOOLEAN NOT NULL DEFAULT 0,
  provider VARCHAR(255) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
  PRIMARY KEY (id),
  UNIQUE KEY (google_id)
);

-- Example SELECT statement to retrieve all users:
-- SELECT HEX(users.id) AS id_hex, BIN_TO_UUID(users.id) AS id_readable, users.google_id, users.email, users.first_name, users.last_name, users.password, users.role, users.photo, users.verified, users.provider, users.created_at, users.updated_at FROM users;
CREATE 
  ALGORITHM = UNDEFINED 
  DEFINER = `root`@`localhost` 
  SQL SECURITY DEFINER
VIEW `listly`.`user_w_readable_id` AS
  SELECT 
    HEX(`listly`.`users`.`id`) AS `id_hex`,
    BIN_TO_UUID(`listly`.`users`.`id`) AS `id_readable`,
    `listly`.`users`.`google_id` AS `google_id`,
    `listly`.`users`.`email` AS `email`,
    `listly`.`users`.`first_name` AS `first_name`,
    `listly`.`users`.`last_name` AS `last_name`,
    `listly`.`users`.`password` AS `password`,
    `listly`.`users`.`role` AS `role`,
    `listly`.`users`.`photo` AS `photo`,
    `listly`.`users`.`verified` AS `verified`,
    `listly`.`users`.`provider` AS `provider`,
    `listly`.`users`.`created_at` AS `created_at`,
    `listly`.`users`.`updated_at` AS `updated_at`
  FROM
    `listly`.`users`