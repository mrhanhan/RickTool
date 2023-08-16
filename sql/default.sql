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
    id   integer primary key,
    app_runtime_id integer not null ,
    code text not null ,
    value text not null
);
-- drop table if exists v_app_runtime;
create table if not exists v_app_runtime
(
    id   integer primary key,
    name text not null ,
    include_system integer not null default 0,
    description text not null
);





