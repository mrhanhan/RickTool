-- 自增编号
create table if not exists t_sequence
(
    code text primary key,
    seq  integer
);
-- 应用分组
-- drop table if exists v_app_group;
create table if not exists v_app_group
(
    id   integer primary key,
    name text,
    icon text
);
-- 运行环境
-- drop table if exists v_app_runtime_item;
create table if not exists v_app_runtime_item
(
    id             integer primary key,
    app_runtime_id integer not null,
    code           text    not null,
    value          text    not null
);
-- drop table if exists v_app_runtime;
create table if not exists v_app_runtime
(
    id             integer primary key,
    name           text    not null,
    include_system integer not null default 0,
    description    text    not null
);

-- 应用程序
-- app 应用程序信息
--  app_ext 应用程序扩展信息
--  app_start 应用程序启动方式
--  app_start_args 启动参数
create table if not exists v_app
(
    id          int primary key,           -- id
    group_id    int  not null default 0,   -- 分组ID 默认为0 全局
    name        text not null,             -- 应用名称
    type        int  not null default 100, -- 类型 100 可执行程序 200 JAVA程序 201 Python 程序 202 NodeJs 300 网页
    target      text not null default '',  -- 目标: 100\200\201\202 可执行程序路径, 300 网页地址
    logo_path   text not null default '',  -- LOGO 图片路径
    remark      text not null default '',  -- 备注
    create_time int  not null              -- 创建事件
);

-- v_app_start
create table if not exists v_app_start
(
    id     int primary key,         -- id
    app_id int  not null,           -- app id
    name   text not null,           -- 启动方式名称
    remark text not null default '' -- 备注
);

-- v_app_start_args 启动参数
create table if not exists v_app_start_args
(
    id            int primary key,         -- id
    app_id        int  not null,           -- app id
    statr_id      int  not null,           -- 启动ID
    group_id      int  not null,           -- 参数是否分组 0 不分组, 1分组
    type          int  not null default 1, -- 参数类型 1 固定参数 2 文件参数 3 枚举
    name          text not null,           -- 参数名称
    default_value text not null,           -- 参数默认值
    config        text not null,           -- 参数配置
    multiple      text not null,           -- 参数是否支持添加多个
    optional      int  not null default 0, -- 是否可选 0 必选 1  可选
    remark        text not null default '' -- 参数配置说明
);

-- v_app_ext 扩展信息
create table if not exists v_app_ext
(
    id       int primary key,         -- id
    app_id   int  not null,           -- app id
    start_id int  not null default 0, -- app start 启动方式ID 如果关联的部署启动方式则为0
    type     text not null,           -- 扩展信息类型
    code     text not null,           -- 扩展信息Code
    value    text not null            -- 扩展信息值
);


