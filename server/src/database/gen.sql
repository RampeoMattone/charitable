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

drop procedure if exists verify;
delimiter //
create procedure verify (param1 char(32), param2 char(32))
begin
    select if ((select (select code from codes WHERE code=param1 AND used_by IS NULL) IS NOT NULL) and ((select id from users where code=param2) is not null), 1, 0);
end
//
DELIMITER ;