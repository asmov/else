create table blockchain_pubkeys {
    blockchain blockchain_t not null,
    pubkey bytea not null,
    interface_id bigint not null references interfaces(interface_id) on delete cascade,
    primary key (blockchain, pubkey),
    unique (blockchain, pubkey, interface_id)
}