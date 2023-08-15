-- 删除外键和索引
ALTER TABLE
    movie_parts DROP CONSTRAINT IF EXISTS movie_parts_movie_id_fkey;

DROP INDEX IF EXISTS movie_parts_movie_id_fkey;

DROP INDEX IF EXISTS movies_tags_index;

-- recharge_records_user_id_fkey
ALTER TABLE
    recharge_records DROP CONSTRAINT IF EXISTS recharge_records_user_id_fkey;

-- consume_records_user_id_fkey
ALTER TABLE
    consume_records DROP CONSTRAINT IF EXISTS consume_records_user_id_fkey;

-- consume_records_movie_id_fkey
ALTER TABLE
    consume_records DROP CONSTRAINT IF EXISTS consume_records_movie_id_fkey;

-- consume_records_movie_part_id_fkey
ALTER TABLE
    consume_records DROP CONSTRAINT IF EXISTS consume_records_movie_part_id_fkey;

DROP INDEX IF EXISTS movies_name_uindex;

DROP INDEX IF EXISTS users_wechat_openid_uindex;

DROP INDEX IF EXISTS users_wechat_unionid_uindex;

DROP INDEX IF EXISTS users_phone_uindex;

ALTER TABLE
    orders DROP CONSTRAINT IF EXISTS orders_user_id_fkey;

-- orders_order_no_uindex
DROP INDEX IF EXISTS orders_order_no_uindex;

-- view_records_user_id_fkey
ALTER TABLE
    view_records DROP CONSTRAINT IF EXISTS view_records_user_id_fkey;

-- view_records_movie_id_fkey
ALTER TABLE
    view_records DROP CONSTRAINT IF EXISTS view_records_movie_id_fkey;

-- view_records_movie_part_id_fkey
ALTER TABLE
    view_records DROP CONSTRAINT IF EXISTS view_records_movie_part_id_fkey;


-- follow_records_user_id_fkey
ALTER TABLE
    follow_records DROP CONSTRAINT IF EXISTS follow_records_user_id_fkey;

-- follow_records_movie_id_fkey
ALTER TABLE
    follow_records DROP CONSTRAINT IF EXISTS follow_records_movie_id_fkey;


-- 删除表
drop table if exists follow_records;

drop table if exists view_records;

drop table if exists consume_records;

drop table if exists recharge_records;

drop table if exists movie_parts;

drop table if exists movies;

drop table if exists users;

drop table if exists orders;