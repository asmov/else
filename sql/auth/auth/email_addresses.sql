create table email_addresses {
    email_address varchar(255) primary key,
    interface_id bigint not null references interfaces(interface_id) on delete cascade,
    is_primary boolean not null,
    unique (email_address, interface_id),
    index (is_primary)
}