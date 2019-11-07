-- Your SQL goes here
# 用户表
CREATE TABLE IF NOT EXISTS users(
                                    `user_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                    `user_name` VARCHAR(50) UNIQUE NOT NULL,                                         /* 用户名 */
                                    `passwd` VARCHAR(50) NOT NULL,                                            /* 密码 */
                                    `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                    `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                    `delete_time` TIMESTAMP NULL DEFAULT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# 分类表
CREATE TABLE IF NOT EXISTS categorys(
                                        `category_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                        `name` VARCHAR(20) NOT NULL,                                               /* 类别名称 */
                                        `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                        `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                        `delete_time` TIMESTAMP NULL DEFAULT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# topics表
CREATE TABLE IF NOT EXISTS topics(
                                     `topic_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                     `title` VARCHAR(100) NOT NULL,                                             /* 文章标题 */
                                     `content` VARCHAR(200) NOT NULL,                                           /* 文章内容uri */
                                     `user_id` INT UNSIGNED NOT NULL,                                           /* 文章作者 */
                                     `category_id` INT UNSIGNED NOT NULL,                                       /* 文章类别 */
                                     `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                     `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                     `delete_time` TIMESTAMP NULL DEFAULT NULL,
                                     CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(user_id),
                                     CONSTRAINT fk_category_id FOREIGN KEY (category_id) REFERENCES categorys(category_id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# 标签表
CREATE TABLE IF NOT EXISTS tags (
                                    `tag_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                    `name` VARCHAR(20) UNIQUE NOT NULL UNIQUE,                                        /* 标签名称 */
                                    `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                    `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                    `delete_time` TIMESTAMP NULL DEFAULT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# topic - 标签 关系表
CREATE TABLE IF NOT EXISTS topics_tags(
                                          `topic_tag_id` INT UNSIGNED PRIMARY KEY  NOT NULL AUTO_INCREMENT,
                                          `topic_id` INT UNSIGNED NOT NULL,                                          /* 文章id */
                                          `tag_id` INT UNSIGNED NOT NULL,                                            /* 标签id */
                                          `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                          `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                          `delete_time` TIMESTAMP NULL DEFAULT NULL,
                                          CONSTRAINT fk_topic_id FOREIGN KEY (topic_id) REFERENCES topics(topic_id),
                                          CONSTRAINT fk_tag_id FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# 评论表
CREATE TABLE IF NOT EXISTS comments(
                                       `comment_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                       `content` VARCHAR(300) NOT NULL,
                                       `topic_id` INT UNSIGNED NOT NULL,
                                       `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                       `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                       `delete_time` TIMESTAMP NULL DEFAULT NULL,
                                       FOREIGN KEY (topic_id) REFERENCES topics(topic_id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# 添加一个用户
INSERT INTO users (user_name, passwd) VALUES ('yeyongping', '123456abc');