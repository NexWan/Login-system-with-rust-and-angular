create database test with owner = 'postgres';

\c test

create table test_table (
    id numeric(5) primary key,
    name varchar(255) not null
);
