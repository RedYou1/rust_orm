drop database if exists rust_query;
create database rust_query;
use rust_query;

#Test 1

create table Test(
    id integer,
    db integer not null,
    nom varchar(255) not null,
    primary key(db, id)
);

create table prof(
    id integer,
    db integer not null,
    nom varchar(255),
    primary key(db, id)
);

create table test_prof(
    db integer not null,
    test_id integer,
    prof_id integer,
    score integer not null,
    primary key(db, test_id, prof_id),
    foreign key (test_id) references Test(id),
    foreign key (prof_id) references prof(id)
);

insert into Test(db, id, nom) values (1,1,'A'),(1,2,'B'), (2,1,'A'),(2,2,'B');
insert into prof(db, id, nom) values (1,1,null),(1,2,'B'), (2,1,null),(2,2,'B');
insert into test_prof(db, test_id, prof_id, score) values (1,1,1,1),(1,1,2,2),(1,2,1,3),(1,2,2,4), (2,1,1,1),(2,1,2,2),(2,2,1,3),(2,2,2,4);

#Test 2

create table user_info (
    id integer primary key auto_increment,
    user_id integer,
    uuid varchar(255),
    foreign key (user_id) references user(id)
);

create table category (
    id integer primary key auto_increment,
    nom varchar(255)
);

create table user (
    id integer primary key auto_increment,
    nom varchar(255)
);

create table pet (
    id integer primary key auto_increment,
    nom varchar(255)
);

create table category_user (
    user_id integer not null,
    category_id integer not null,
    foreign key (user_id) references user(id),
    foreign key (category_id) references category(id)
);

create table category_pet (
    pet_id integer not null,
    category_id integer not null,
    foreign key (pet_id) references pet(id),
    foreign key (category_id) references category(id)
);

insert into category(nom) values ('A'),('B');
insert into user(nom) values ('A'),('B'),('C');
insert into user_info(user_id, uuid) values (1, null),(2, 'B');
insert into category_user(user_id,category_id) values (1,1),(1,2),(2,1);
insert into pet(nom) values ('A'),('B'),('C');
insert into category_pet(pet_id,category_id) values (1,2),(1,1),(2,2);