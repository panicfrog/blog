DROP USER IF EXISTS 'dev'@'localhost' ;
CREATE USER 'dev'@'localhost' IDENTIFIED BY '123456abc';

CREATE DATABASE IF NOT EXISTS blog;
GRANT ALL PRIVILEGES ON blog.* TO 'dev'@'localhost';