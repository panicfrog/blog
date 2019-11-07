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
                                        `name` VARCHAR(20) UNIQUE NOT NULL,                                               /* 类别名称 */
                                        `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                        `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                        `delete_time` TIMESTAMP NULL DEFAULT NULL
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# posts表
CREATE TABLE IF NOT EXISTS posts(
                                     `post_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                     `title` VARCHAR(100) NOT NULL,                                             /* 文章标题 */
                                     `content` Text NOT NULL,                                           /* 文章内容uri */
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

# post - 标签 关系表
CREATE TABLE IF NOT EXISTS posts_tags(
                                          `post_tag_id` INT UNSIGNED PRIMARY KEY  NOT NULL AUTO_INCREMENT,
                                          `post_id` INT UNSIGNED NOT NULL,                                          /* 文章id */
                                          `tag_id` INT UNSIGNED NOT NULL,                                            /* 标签id */
                                          `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                          `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                          `delete_time` TIMESTAMP NULL DEFAULT NULL,
                                          CONSTRAINT fk_post_id FOREIGN KEY (post_id) REFERENCES posts(post_id),
                                          CONSTRAINT fk_tag_id FOREIGN KEY (tag_id) REFERENCES tags(tag_id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# 评论表
CREATE TABLE IF NOT EXISTS comments(
                                       `comment_id` INT UNSIGNED PRIMARY KEY NOT NULL AUTO_INCREMENT,
                                       `content` VARCHAR(300) NOT NULL,
                                       `post_id` INT UNSIGNED NOT NULL,
                                       `sid` INT UNSIGNED,
                                       `create_time` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                                       `update_time` TIMESTAMP NULL DEFAULT NULL ON UPDATE CURRENT_TIMESTAMP,
                                       `delete_time` TIMESTAMP NULL DEFAULT NULL,
                                       FOREIGN KEY (post_id) REFERENCES posts(post_id),
                                       FOREIGN KEY (sid) REFERENCES comments(comment_id)
)ENGINE=InnoDB DEFAULT CHARSET=utf8 AUTO_INCREMENT=100000;

# 添加一个用户
INSERT INTO users (user_name, passwd) VALUES ('yeyongping', '123456abc');
INSERT INTO categorys (name) VALUES ('技术');
INSERT INTO tags (name) VALUES ('rust');
INSERT INTO posts (title, content, user_id, category_id) VALUES ('第一篇标题', '夜，结束了一天的喧嚣后安静下来，伴随着远处路灯那微弱的光。风，毫无预兆地席卷整片旷野，撩动人的思绪万千。星，遥遥地挂在天空之中，闪烁着它那微微星光，不如阳光般灿烂却如花儿般如痴如醉', 100000, 100000);
INSERT INTO posts_tags (post_id, tag_id) VALUES (100000, 100000);
INSERT INTO comments (content, post_id) VALUES ('这个文章很棒', 100000);