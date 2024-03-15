create table pending_email_blockchain (
    id serial primary key,
    email_address varchar(255) not null,
    blockchain blockchain_t not null,
    pubkey bytea not null,
    challenge int not null,
    expires timestamp not null,
    unique (email, blockchain, pubkey),
    index (expires)
);

