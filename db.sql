create database test with owner = 'postgres';

\c test

create table test_table (
    username varchar(255) primary key,
    password varchar(255) not null
);

insert into test_table (username, password) values ('admin', 'admin');