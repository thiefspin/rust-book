-- Your SQL goes here
CREATE SCHEMA user_auth;

CREATE TABLE user_auth.users
(
    id         bigserial PRIMARY KEY,
    email      varchar(40) NOT NULL,
    name       varchar(40) NOT NULL,
    surname    varchar(40) NOT NULL,
    phone      varchar(15) NOT NULL,
    job_title  varchar(40) NOT NULL,
    password   varchar     NOT NULL,
    address_id bigint      NOT NULL,
    created    timestamp   NOT NULL,
    last_login timestamp
);

INSERT INTO user_auth.users(email, name, surname, phone, job_title, password, address_id, created)
VALUES ('admin@mail.com', 'Admin', 'User', '08148262234', 'Chief Executive officer', 'password', 1, current_timestamp);
INSERT INTO user_auth.users(email, name, surname, phone, job_title, password, address_id, created)
VALUES ('peter@mail.com', 'Peter', 'Van Zyl', '08148262234', 'Head of Marketing', 'password', 1, current_timestamp);
INSERT INTO user_auth.users(email, name, surname, phone, job_title, password, address_id, created)
VALUES ('fred@mail.com', 'Fred', 'Couples', '08148262234', 'Engineer', 'password', 1, current_timestamp);

CREATE TABLE user_auth.roles
(
    id   bigserial NOT NULL PRIMARY KEY,
    name varchar(255)
);

INSERT INTO user_auth.roles(name)
VALUES ('ADMIN'),
       ('USER');

CREATE TABLE user_auth.users_roles
(
    id      bigserial NOT NULL PRIMARY KEY,
    role_id bigint    NOT NULL REFERENCES user_auth.roles (id),
    user_id bigint    NOT NULL REFERENCES user_auth.users (id)
);

INSERT INTO user_auth.users_roles(role_id, user_id)
VALUES (1, 1),
       (2, 1),
       (2, 2),
       (2, 3);
