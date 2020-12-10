create table if not exists users
(
    id   serial primary key,
    code CHAR(32) NOT NULL unique
);

create table if not exists codes
(
    id        serial primary key,
    code      CHAR(32) not null,
    used_by   bigint,
    timestamp timestamp,

    constraint code_used_by foreign key (used_by) references users (id),
    unique (code)
);

drop function if exists register;
create function register(_code char(32), _user char(32))
    returns record as
$$
declare
    ret record;
begin
    if
            (select (select code from codes WHERE code = _code AND used_by IS NULL) IS NOT NULL)
            and
            ((select id from users where code = _user) is not null)
    then
        update codes set used_by=(select id from users where users.code = _user) where code = _code;
        select true, _code, _user into ret;
    else
        select false, _code, _user into ret;
    end if;
    return ret;
end;
$$ language plpgsql