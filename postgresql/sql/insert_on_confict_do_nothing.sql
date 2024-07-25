drop table if exists data;

create table data (
    id serial primary key,
    name text not null unique
);

insert into data (name) values ('n1') on conflict do nothing returning id;
insert into data (name) values ('n1') on conflict do nothing returning id;