-- Your SQL goes here
CREATE TABLE transactions (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    payee text NOT NULL,
    inflow INT NOT NULL,
    outflow INT NOT NULL,
    notes text NOT NULL,
    CONSTRAINT transactions_pkey PRIMARY KEY (id)
);