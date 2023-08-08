-- 删除外键和索引
ALTER TABLE
    movie_parts DROP CONSTRAINT IF EXISTS movie_parts_movie_id_fkey;

DROP INDEX IF EXISTS movie_parts_movie_id_fkey;

DROP INDEX IF EXISTS movies_name_uindex;

DROP INDEX IF EXISTS users_wechat_openid_uindex;

DROP INDEX IF EXISTS users_wechat_unionid_uindex;

DROP INDEX IF EXISTS users_phone_uindex;

-- 删除表
drop table if exists follow_records;

drop table if exists view_records;

drop table if exists consume_records;

drop table if exists recharge_records;

drop table if exists movie_parts;

drop table if exists movies;

drop table if exists users;