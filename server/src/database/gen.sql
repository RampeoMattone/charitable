drop database if exists charitable;

create database charitable character set 'utf8';

use charitable;

create table if not exists users (
    id serial,
    code CHAR(32) NOT NULL unique
);

create table if not exists codes (
    id serial,
    code CHAR(32) NOT NULL unique,
    used_by bigint unsigned zerofill not null,
    timestamp timestamp,

    constraint code_used_by foreign key (used_by) references users (id)
);
