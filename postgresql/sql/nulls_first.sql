drop table if exists data;

create table data (
    id serial primary key,
    name text,
    value integer
);

insert into data (name, value) values ('n1', 21);
insert into data (name) values ('n2');
insert into data (name, value) values ('n3', 3);

select * from data order by value nulls first;


--  id | name | value 
-- ----+------+-------
--   2 | n2   |      
--   3 | n3   |     3
--   1 | n1   |    21
-- (3 rows)
