-- Add up migration script here
CREATE TABLE users.users
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