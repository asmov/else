#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MachineCortex {
    interface_id: InterfaceID,
    routine_id: RoutineID,
    routine_awareness: Awareness,
}

