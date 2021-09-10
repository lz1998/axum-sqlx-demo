create database demo charset utf8mb4;

use demo;

CREATE TABLE `user`
(
    `id`       bigint(20) unsigned NOT NULL PRIMARY KEY AUTO_INCREMENT,
    `username` varchar(255) NOT NULL COMMENT 'username',
    `password` varchar(255) NOT NULL COMMENT 'password'
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4;

insert into user(id, username, password) VALUES
(1,"aaa","bbb"),(2,"ccc","ddd"),(3,"eee","fff");