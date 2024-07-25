drop table if exists data;

create table data (
    id serial primary key,
    name text not null,
    email text not null unique,
    status text not null check (status in ('ok','error', 'unknown')) default 'unknown',
    value1 integer,
    value2 decimal,
    created_at timestamptz not null default now(),
    updated_at timestamptz
);

create index on data(created_at);
create index on data(updated_at);
