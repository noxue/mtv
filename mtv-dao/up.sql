/*
 用户表 users
 id              用户编号
 nickname        用户昵称，可选
 avatar          用户头像， 可选
 score           用户积分 充值获取
 vip             用户会员类型 0:普通用户 1:月会员 2:季会员 3:年会员 默认0
 vip_expire_time 会员过期时间 可空，默认 1970-01-01 00:00:00
 auth 认证信息 jsonb格式
 {
 "phone":"",   可选
 "password":"", 可选
 "wechat_unionid":"", 可选
 "wechat_openid":"", 可选
 }
 create_time     创建时间，时间戳+时区
 update_time     更新时间，时区
 */
CREATE TABLE users (
    id serial PRIMARY KEY,
    nickname varchar(20),
    avatar varchar(255),
    score int4 DEFAULT 0,
    vip int4 DEFAULT 0,
    vip_expire_time timestamp with time zone DEFAULT now(),
    auth jsonb DEFAULT '{}' :: jsonb,
    channel varchar(255),
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

CREATE UNIQUE INDEX users_phone_uindex ON users ((auth ->> 'phone' :: text) varchar_pattern_ops);

-- 创建索引 允许为空字符串


CREATE UNIQUE INDEX users_wechat_openid_uindex ON users (
    (auth ->> 'wechat_openid' :: text) varchar_pattern_ops
);

/*
 * 影片表 movies
 id              影片编号
 name            影片名称
 cover           影片封面
 total           影片总集数
 description     影片描述
 is_top          是否置顶 boolean
 is_hot          是否热门 boolean
 tags            影片标签 数组
 price_total     整部影片价格
 price_single    单集影片价格
 is_show         影片状态 boolean
 view            观看数
 likes           点赞数
 vlikes          虚拟点赞数
 is_finish       是否完结 boolean
 share_title     分享标题
 share_pic       分享图片
 create_time     创建时间
 update_time     更新时间
 */
CREATE TABLE movies (
    id serial PRIMARY KEY,
    name varchar(20) NOT NULL,
    cover varchar(255),
    total int4,
    description varchar(255),
    is_top bool,
    is_hot bool,
    tags varchar(255) [] NOT NULL DEFAULT '{}' :: varchar(255) [],
    price_total int4 NOT NULL DEFAULT 0,
    is_show bool NOT NULL DEFAULT false,
    view int4 NOT NULL DEFAULT 0,
    likes int4 DEFAULT 0,
    vlikes int4 DEFAULT 0,
    is_finish bool NOT NULL DEFAULT false,
    share_title varchar(100),
    share_pic varchar(255),
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

-- tags 创建索引
CREATE INDEX movies_tags_index ON movies USING gin (tags);

CREATE UNIQUE INDEX movies_name_uindex ON movies (name);

/*
 影片集表 videos
 id 
 movie_id        影片编号 创建外键
 name            集名称
 video           集视频地址
 price           集价格
 status          集状态 0:下架 1:上架
 likes           点赞数
 share           分享数
 view            观看数
 rank            排序
 create_time     创建时间
 update_time     更新时间
 */
CREATE TABLE videos (
    id serial PRIMARY KEY,
    movie_id int4 NOT NULL,
    name varchar(20) NOT NULL,
    video varchar(255) NOT NULL,
    price int4 NOT NULL DEFAULT 0,
    status int4 NOT NULL DEFAULT 0,
    likes int4 NOT NULL DEFAULT 0,
    share int4 NOT NULL DEFAULT 0,
    view int4 NOT NULL DEFAULT 0,
    rank int4 NOT NULL DEFAULT 0,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

-- 外键,movie_id,movie被删除，videos也会被删除, cascade 级联删除
ALTER TABLE
    videos
ADD
    CONSTRAINT videos_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id) ON DELETE CASCADE;


/*
 充值记录表 recharge_records
 id
 user_id         用户编号
 amount          充值金额
 score           获得积分  如果积分为0 就是开会员，在备注中说明
 mark            备注
 status          充值状态 0:失败 1:成功
 create_time     创建时间
 update_time     更新时间
 */
CREATE TABLE recharge_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    amount int4 NOT NULL DEFAULT 0,
    score int4 NOT NULL DEFAULT 0,
    mark varchar(255) NOT NULL,
    status int4 NOT NULL DEFAULT 0,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

-- 外键
ALTER TABLE
    recharge_records
ADD
    CONSTRAINT recharge_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

/*
 消费记录 consume_records
 id 
 user_id         用户编号
 movie_id        影片编号
 video_id   影片集编号
 score           消费积分
 mark            备注
 create_time     创建时间
 */
CREATE TABLE consume_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    movie_id int4 NOT NULL,
    video_id int4 NOT NULL,
    score int4 NOT NULL DEFAULT 0,
    mark varchar(255) NOT NULL,
    create_time timestamp with time zone DEFAULT now()
);

-- 外键
ALTER TABLE
    consume_records
ADD
    CONSTRAINT consume_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

-- 删除movie的时候不变
ALTER TABLE
    consume_records
ADD
    CONSTRAINT consume_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id) ON DELETE CASCADE;

ALTER TABLE
    consume_records
ADD
    CONSTRAINT consume_records_video_id_fkey FOREIGN KEY (video_id) REFERENCES videos(id)  ON DELETE CASCADE;

-- user_id 创建索引
CREATE INDEX consume_records_user_id_index ON consume_records (user_id);

-- movie_id 创建索引
CREATE INDEX consume_records_movie_id_index ON consume_records (movie_id);

-- 添加 video_id 和 user_id 联合索引
CREATE INDEX consume_records_video_id_user_id_index ON consume_records (video_id, user_id);

/*
 浏览记录 view_records
 id
 user_id         用户编号
 movie_id        影片编号
 video_id   影片集编号
 create_time     创建时间
 update_time     更新时间
 */
CREATE TABLE view_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    movie_id int4 NOT NULL,
    video_id int4 NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

-- 外键
ALTER TABLE
    view_records
ADD
    CONSTRAINT view_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE
    view_records
ADD
    CONSTRAINT view_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id)  ON DELETE CASCADE;

ALTER TABLE
    view_records
ADD
    CONSTRAINT view_records_video_id_fkey FOREIGN KEY (video_id) REFERENCES videos(id)  ON DELETE CASCADE;

/*
点赞记录
*/
CREATE TABLE likes_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    movie_id int4 NOT NULL,
    video_id int4 NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

-- 外键
ALTER TABLE
    likes_records
ADD
    CONSTRAINT likes_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE
    likes_records
ADD
    CONSTRAINT likes_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id)  ON DELETE CASCADE;

-- user_id 和 video_id 联合索引
CREATE INDEX likes_records_user_id_video_id_index ON likes_records (user_id, video_id);

/*
 追剧记录 follow_records
 id
 user_id         用户编号
 movie_id        影片编号
 create_time     创建时间
 */
CREATE TABLE follow_records (
    id serial PRIMARY KEY,
    user_id int4 NOT NULL,
    movie_id int4 NOT NULL,
    create_time timestamp with time zone DEFAULT now()
);

-- 外键
ALTER TABLE
    follow_records
ADD
    CONSTRAINT follow_records_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE
    follow_records
ADD
    CONSTRAINT follow_records_movie_id_fkey FOREIGN KEY (movie_id) REFERENCES movies(id)  ON DELETE CASCADE;

/*
 订单表 orders
 id
 goods_id        商品编号
 user_id         用户编号
 amount          订单金额
 order_no        订单编号
 description     订单描述
 status          订单状态 0:未支付 1:成功，-1失败
 create_time     创建时间
 update_time     更新时间
 */
CREATE TABLE orders (
    id serial PRIMARY KEY,
    goods_id int4 NOT NULL,
    user_id int4 NOT NULL,
    amount int4 NOT NULL,
    order_no varchar(50) NOT NULL,
    description varchar(255) NOT NULL,
    status int4 NOT NULL DEFAULT 0,
    create_time timestamp with time zone DEFAULT now(),
    update_time timestamp with time zone DEFAULT now()
);

-- 外键
ALTER TABLE
    orders
ADD
    CONSTRAINT orders_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id);

-- order_no 唯一索引
CREATE UNIQUE INDEX orders_order_no_uindex ON orders (order_no);



/*
商品列表
id              商品编号
name            商品名称
price           商品价格
description     商品描述
score           获得积分
is_hot          是否是热门商品
is_vip          是否是开会员商品,开vip和普通购买积分，分开处理
expire_type     会员过期类型 0:月 1:季 2:年
expire_count    会员过期周期数
create_time     创建时间
*/
CREATE TABLE goods (
    id serial PRIMARY KEY,
    name varchar(20) NOT NULL,
    price int4 NOT NULL DEFAULT 0,
    description varchar(255) NOT NULL,
    score int4 NOT NULL DEFAULT 0,
    is_hot bool NOT NULL DEFAULT false,
    is_vip bool NOT NULL DEFAULT false,
    expire_type int4 NOT NULL DEFAULT 0,
    expire_count int4 NOT NULL DEFAULT 0,
    create_time timestamp with time zone DEFAULT now()
);
 