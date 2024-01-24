use crate::identity::Identity;

pub type MessageID = u16;
pub type ErrorCode = u8;
pub type TimeFrame = u64;

pub enum ErrorCodes {
    IllegalWebsocketFrame = 0x01 
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ClientToZoneMessage {
    Acknowledged(AcknowledgedMsg), // 0
    Error(ErrorMsg), // 1
    /// Request to connect to the world through this server.
    /// Expects responses: ZoneToClientMessage::[Connected, ConnectRejected]
    Connect,
    /// Request to be transferred from another Zone (by the original Zone's instructions) to this one.
    Transfer,
    /// Notify the original Zone that the transfer it ordered is in progress.
    Transferring,
    /// Notify the original Zone that the transfer it ordered has failed.
    TransferDenied,
    /// Notify the original Zone that the transfer it ordered is complete and it is safe to Disconnect.
    Transfered,
    /// Client is about to disconnect.
    /// Expects response: ZoneToClientMessage::Disconnect
    Disconnect,
    /// Request to move through a Route to a different Area.  
    /// Expected responses:
    /// - ZoneToClientMessage::GoApproved
    /// - ZoneToClientMessage::GoRejected
    Go,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ZoneToClientMessage {
    Acknowledged(AuthorityAcknowledgedMsg), // 0
    Error(ErrorMsg), // 1
    TimeFrame(NewTimeFrameMsg), // 2
    /// Response: Request to connect to the world through this server has been accepted.
    Connected,
    /// Response: Request to connect to the world through this server has been rejected.
    ConnectRejected,
    Transfered,
    TransferRejected,
    Disconnect,
    /// Response: Request to move through a Route has been approved. It will occur in the specified timeframe.
    GoApproved,
    /// Response: Request to move through a Route has been rejected.
    GoRejected,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ZoneToWorldMessage {
    Acknowledged(AcknowledgedMsg), // 0
    Error(ErrorMsg), // 1
    Connect,
    Disconnect,
    ClientApproval,
    ClientConnected,
    ClientTransferring,
    ClientTransfered,
    ClientDisconnected
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum WorldToZoneMessage {
    Acknowledged(AuthorityAcknowledgedMsg), // 0
    Error(ErrorMsg), // 1
    TimeFrame(NewTimeFrameMsg), // 2
    Connected,
    ConnectRejected,
    Disconnect,
    ClientApproved,
    ClientRejected,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AuthorityAcknowledgedMsg {
    pub message_id: MessageID,
    pub timeframe: TimeFrame
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AcknowledgedMsg {
    pub message_id: MessageID
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ErrorMsg {
    pub message_id: MessageID,
    pub error_code: ErrorCode
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct NewTimeFrameMsg {
    pub timeframe: TimeFrame
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GoRequest {
    pub route_id: Identity,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GoApproved {
    pub message_id: MessageID,
    pub timeframe: TimeFrame,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct GoRejected {
    pub message_id: MessageID,
    pub error_code: ErrorCode
}