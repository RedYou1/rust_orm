drop database if exists rust_query;
create database rust_query;
use rust_query;

create table Test(
    id integer primary key auto_increment,
    nom varchar(255) not null
);

create table prof(
    id integer primary key auto_increment,
    nom varchar(255)
);

create table test_prof(
    test_id integer,
    prof_id integer,
    score integer not null,
    primary key(test_id, prof_id),
    foreign key (test_id) references Test(id),
    foreign key (prof_id) references prof(id)
);

insert into Test(nom) values ('A'),('B');
insert into prof(nom) values (null),('B');
insert into test_prof(test_id, prof_id, score) values (1,1,1),(1,2,2),(2,1,3),(2,2,4);