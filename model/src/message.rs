use std::fmt::Display;
use strum;

use crate::{timeframe::*, action::*, sync::Sync};

pub type MessageID = u16;
pub type ErrorCode = u8;

pub enum ErrorCodes {
    IllegalWebsocketFrame = 0x01 
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Copy, Debug, strum::Display)]
pub enum Protocol {
    Unsupported,
    ClientToZone,
    ZoneToWorld,
    WorldToZone,
    ZoneToClient,
}

pub trait Messaging: Sized + serde::Serialize + serde::de::DeserializeOwned {
    fn message_type_name() -> &'static str;
    fn message_name(&self) -> &'static str;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProtocolHeader {
    pub protocol: Protocol,
    pub version: u8
}

pub const PROTOCOL_VERSION: u8 = 1;

impl Messaging for ProtocolHeader {
    fn message_name(&self) -> &'static str {
        Self::MESSAGE_NAME
    }

    fn message_type_name() -> &'static str {
        Self::MESSAGE_NAME
    }
}

impl ProtocolHeader {
    pub const MESSAGE_NAME: &'static str = "ProtocolHeader";

    pub fn current(protocol: Protocol) -> Self {
        Self {
            protocol,
            version: PROTOCOL_VERSION
        }
    }

    /// Checks this library's version and the expected protocol
    pub fn compatible(&self, expected_protocol: Protocol) -> bool {
        self.version == PROTOCOL_VERSION
            && self.protocol == expected_protocol
    }
}

impl Display for ProtocolHeader {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} v{}", self.protocol, self.version)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, strum::AsRefStr)]
pub enum ClientToZoneMessage {
    /// Request to connect to the world through this server.
    /// Expects responses: ZoneToClientMessage::[Connected, ConnectRejected]
    Connect,
    /// Client is about to disconnect.
    /// Expects response: ZoneToClientMessage::Disconnect
    Disconnect,
    Downlink(UID),
    Action(Action)
}

impl Messaging for ClientToZoneMessage {
    fn message_type_name() -> &'static str {
        "ClientToZoneMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            ClientToZoneMessage::Connect => "ClientToZoneMessage::Connect",
            ClientToZoneMessage::Disconnect => "ClientToZoneMessage::Disconnect",
            ClientToZoneMessage::Downlink(_) => "ClientToZoneMessage::Downlink",
            ClientToZoneMessage::Action(_) => "ClientToZoneMessage::Action",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, strum::AsRefStr)]
pub enum ZoneToClientMessage {
    TimeFrame(NewTimeFrameMsg), // 2
    /// Response: Request to connect to the world through this server has been accepted.
    Connected,
    /// Response: Request to connect to the world through this server has been rejected.
    ConnectRejected,
    InitInterfaceView(TimeFrame, Vec<u8>),
    Sync(Sync),
    Disconnect,
    DownlinkApproved(Frame),
    DownlinkRejected(Frame),
    ActionApproved(Frame),
    ActionApprovedAt(Frame, Frame),
    ActionRejected(Frame),
}

impl Messaging for ZoneToClientMessage {
    fn message_name(&self) -> &'static str {
        match self {
            Self::TimeFrame(_) => "ZoneToClientMessage::TimeFrame",
            Self::Connected => "ZoneToClientMessage::Connected",
            Self::ConnectRejected => "ZoneToClientMessage::ConnectRejected",
            Self::InitInterfaceView(_, _)=> "ZoneToClientMessage::InitInterfaceView",
            Self::Sync(_) => "ZoneToClientMessage::Sync",
            Self::Disconnect => "ZoneToClientMessage::Disconnect",
            Self::DownlinkApproved(_) => "ZoneToClientMessage::DownlinkApproved",
            Self::DownlinkRejected(_) => "ZoneToClientMessage::DownlinkRejected",
            Self::ActionApproved(_) => "ZoneToClientMessage::ActionApproved",
            Self::ActionApprovedAt(_,_) => "ZoneToClientMessage::ActionApprovedWithChange",
            Self::ActionRejected(_) => "ZoneToClientMessage::ActionRejected",
        }
    }

    fn message_type_name() -> &'static str {
        "ZoneToClientMessage"
    }
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

impl Messaging for ZoneToWorldMessage {
    fn message_type_name() -> &'static str {
        "ZoneToWorldMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            ZoneToWorldMessage::Acknowledged(_) => "ZoneToWorldMessage::",
            ZoneToWorldMessage::Error(_) => "ZoneToWorldMessage::Error",
            ZoneToWorldMessage::Connect => "ZoneToWorldMessage::Connect",
            ZoneToWorldMessage::Disconnect => "ZoneToWorldMessage::Disconnect",
            ZoneToWorldMessage::ClientApproval => "ZoneToWorldMessage::ClientApproval",
            ZoneToWorldMessage::ClientConnected => "ZoneToWorldMessage::ClientConnected",
            ZoneToWorldMessage::ClientTransferring => "ZoneToWorldMessage::ClientTransferring",
            ZoneToWorldMessage::ClientTransfered => "ZoneToWorldMessage::ClientTransfered",
            ZoneToWorldMessage::ClientDisconnected => "ZoneToWorldMessage::ClientDisconnected",
        }
    }
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
    WorldBytes(TimeFrame, Vec<u8>),
    Sync(Sync)
}

impl Messaging for WorldToZoneMessage {
    fn message_type_name() -> &'static str {
        "WorldToZoneMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            Self::Acknowledged(_) => "WorldToZoneMessage::Acknowledged",
            Self::Error(_) => "WorldToZoneMessage::Error",
            Self::TimeFrame(_) => "WorldToZoneMessage::TimeFrame",
            Self::Connected => "WorldToZoneMessage::Connected",
            Self::ConnectRejected => "WorldToZoneMessage::ConnectRejected",
            Self::Disconnect => "WorldToZoneMessage::Disconnect",
            Self::ClientApproved => "WorldToZoneMessage::ClientApproved",
            Self::ClientRejected => "WorldToZoneMessage::ClientRejected",
            Self::WorldBytes(_,_) => "WorldToZoneMessage::WorldBytes",
            Self::Sync(_) => "WorldToZoneMessage::Sync"
        }
    }
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