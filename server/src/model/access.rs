use crate::model::identity::*;

#[derive(Debug)]
pub struct Permission(u32);

#[derive(Debug)]
pub struct Access {
    last_access: LastAccess,
    group_rights: Vec<GroupRights> 
}

#[derive(Debug)]
pub struct LastAccess {
    created_time: u64,
    modified_time: u64,
    created_player_id: ID,
    modified_player_id: ID,
}

#[derive(Debug)]
pub struct GroupRights {
    group_id: ID,
    permission: Permission 
}

#[derive(Debug)]
pub struct AccessGroup {
    id: ID,
    player_ids: Vec<u64>,
    // the Access.player_group_ids must not circular-reference this group ID
    access: Access
}