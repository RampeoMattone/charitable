drop database if exists anoni;
create database if not exists anoni;
use anoni;
create table if not exists server
(
    id             bigint unsigned zerofill not null,
    mod_channel    bigint unsigned zerofill not null,
    pub_channel    bigint unsigned zerofill not null,
    available      bool default true        not null,

    constraint server_id_pk       primary key (id),
    constraint server_mod_channel unique      (mod_channel),
    constraint server_pub_channel unique      (pub_channel)
);

create table if not exists user
(
    id        bigint unsigned zerofill not null,
    username  varchar(32)              not null,
    timestamp timestamp default  current_timestamp() not null,

    constraint user_id_pk primary key (id)
);

create table if not exists inbox
(
    id         bigint unsigned zerofill              not null,
    author     bigint unsigned zerofill              not null,
    message    varchar(2000)                         not null,
    timestamp  timestamp default current_timestamp() not null,

    constraint inbox_id_pk      primary key (id),
    constraint inbox_author_fk  foreign key (author) references user (id)
);

create table if not exists outbox
(
    id        serial,
    inbox_id  bigint unsigned zerofill 	     not null,
    server_id bigint unsigned zerofill       not null,
    mod_id    bigint unsigned zerofill       not null,
    pub_id    bigint unsigned zerofill       null,
    rejected  boolean default 0           not null,

    constraint outbox_id_pk        primary key (id),
    constraint outbox_mod_id       unique      (mod_id),
    constraint outbox_inbox_id_fk  foreign key (inbox_id)  references inbox  (id),
    constraint outbox_server_id_fk foreign key (server_id) references server (id)
);

create table if not exists user_to_server
(
    id    serial,
    user_id   bigint unsigned zerofill not null,
    server_id bigint unsigned          not null,
    enabled   bool default true        not null,

    constraint user_to_server_id_pk  primary key (id),
    constraint user_to_server_id_combo unique (user_id, server_id),
    constraint user_to_server_server_id_fk foreign key (server_id) references server (id),
    constraint user_to_server_user_id_fk   foreign key (user_id)   references user   (id)
);
