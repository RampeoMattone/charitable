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

drop procedure if exists register;
delimiter //
create procedure register (_code char(32), _user char(32))
begin
    if
            (select (select code from codes WHERE code=_code AND used_by IS NULL) IS NOT NULL)
            and
            ((select id from users where code=_user) is not null)
    then
        update codes set used_by=(select id from users where users.code=_user) where code=_code;
        select true;
    else
        select false;
    end if;
end
//
DELIMITER ;