-- Your SQL goes here
CREATE TABLE accounts (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    type_of text NOT NULL,
    starting_balance INT NOT NULL,
    current_balance INT NOT NULL,
    CONSTRAINT accounts_pkey PRIMARY KEY (id)
);