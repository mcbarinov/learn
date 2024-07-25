drop table if exists data;

create table data(
    id serial primary key,
    name text not null unique
);

insert into data (name) values ('n1'), ('n2'), ('n3');
select name from data where name not in ('n1', 'n3');
-- n2