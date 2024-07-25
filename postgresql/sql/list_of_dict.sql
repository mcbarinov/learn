drop table if exists data;

create table data (
    id serial primary key,
    name text not null,
    logs jsonb[] not null default '{}'
);

insert into data (name) values ('n1');

update data set logs=logs||'{"op":"move", "time": "zzz"}'::jsonb where name='n1';
update data set logs=logs||'{"op":"move2", "time": "zzz2"}'::jsonb where name='n1';


select logs from data;