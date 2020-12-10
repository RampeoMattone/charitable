if(select (select code from codes WHERE code = _code AND used_by IS NULL) IS NOT NULL)
            and
            ((
select id
from users
where code = _ user) is not null)
    then
update codes
set used_by=(select id from users where users.code = _ user)
where code = _code;
set
success=true;
else
        set success=false;
end if;