use std::fmt::Display;
use strum;

use crate::{timeframe::*, action::*, sync::*, descriptor::*, interface::*};

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
    ZoneToUniverse,
    WorldToUniverse,
    UniverseToWorld,
    UniverseToZone,
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

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConnectMsg {
    pub auth: Auth,
    pub last_downlink_uid: Option<UID>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ListLinkableMsg {
    pub page: u16
}

#[derive(serde::Serialize, serde::Deserialize, Debug, strum::AsRefStr)]
pub enum ClientToZoneMessage {
    /// Request to connect to the world through this server.
    /// Sends an authentication along with an optional downlink UID that was previously used.
    /// Expects responses: ZoneToClientMessage::[Connected, ConnectRejected]
    Connect(ConnectMsg),
    /// Client is about to disconnect.
    /// Expects response: ZoneToClientMessage::Disconnect
    Disconnect,
    /// Requests a paginated (u16, 0-index) list of downlinks that are that are reserved for OR, if none, those
    ///   immediately available to the client's interface.
    ListLinkable(ListLinkableMsg),
    Downlink(UID),
    Unlink,
    Action(Action),
}

impl Messaging for ClientToZoneMessage {
    fn message_type_name() -> &'static str {
        "ClientToZoneMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            ClientToZoneMessage::Connect(_) => "ClientToZoneMessage::Connect",
            ClientToZoneMessage::Disconnect => "ClientToZoneMessage::Disconnect",
            ClientToZoneMessage::ListLinkable(_) => "ClientToZoneMessage::ListLinkable",
            ClientToZoneMessage::Downlink(_) => "ClientToZoneMessage::Downlink",
            ClientToZoneMessage::Unlink => "ClientToZoneMessage::Downlink",
            ClientToZoneMessage::Action(_) => "ClientToZoneMessage::Action",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ConnectedMsg {
    pub interface_uid: UID,
    pub linkable: Vec<(UID, Descriptor)>,
    pub linkable_pages: u16
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LinkableMsg {
    pub page: u16,
    pub linkable: Vec<(UID, Descriptor)>,
    pub pages: u16
}

#[derive(serde::Serialize, serde::Deserialize, Debug, strum::AsRefStr)]
pub enum ZoneToClientMessage {
    TimeFrame(NewTimeFrameMsg), // 2
    /// Response: Request to connect to the world through this server has been accepted.
    /// Returns the interface UID that the Authentication resolved to.
    /// Provides the first page of ListLinkable results
    Connected(ConnectedMsg),
    /// Response: Request to connect to the world through this server has been rejected.
    ConnectRejected,
    InitInterfaceView(TimeFrame, Vec<u8>),
    Sync(Sync),
    Disconnect,
    // Provides a list of downlinks that are that are reserved for or immediately available to the client's interface.
    // Paginated 0-index with page (first u16) and number of pages (last u16)
    // Tuple includes comprised of the character's UID and descriptor.
    Linkable(LinkableMsg),
    DownlinkApproved(Frame,Frame),
    DownlinkRejected(Frame),
    Unlinked(Frame),
    ActionApproved(Frame,Frame),
    ActionRejected(Frame),
}

impl Messaging for ZoneToClientMessage {
    fn message_name(&self) -> &'static str {
        match self {
            Self::TimeFrame(_) => "ZoneToClientMessage::TimeFrame",
            Self::Connected(_) => "ZoneToClientMessage::Connected",
            Self::ConnectRejected => "ZoneToClientMessage::ConnectRejected",
            Self::InitInterfaceView(_, _)=> "ZoneToClientMessage::InitInterfaceView",
            Self::Sync(_) => "ZoneToClientMessage::Sync",
            Self::Disconnect => "ZoneToClientMessage::Disconnect",
            Self::Linkable(_) => "ZoneToClientMessage::Linkable",
            Self::DownlinkApproved(_,_) => "ZoneToClientMessage::DownlinkApproved",
            Self::DownlinkRejected(_) => "ZoneToClientMessage::DownlinkRejected",
            Self::Unlinked(_) => "ZoneToClientMessage::Unlinked",
            Self::ActionApproved(_,_) => "ZoneToClientMessage::ActionApproved",
            Self::ActionRejected(_) => "ZoneToClientMessage::ActionRejected",
        }
    }

    fn message_type_name() -> &'static str {
        "ZoneToClientMessage"
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ZoneToWorldMessage {
    Connect,
    Disconnect,
}

impl Messaging for ZoneToWorldMessage {
    fn message_type_name() -> &'static str {
        "ZoneToWorldMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            Self::Connect => "ZoneToWorldMessage::Connect",
            Self::Disconnect => "ZoneToWorldMessage::Disconnect",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ZoneToUniverseMessage {
    Connect,
    Disconnect,
}

impl Messaging for ZoneToUniverseMessage {
    fn message_type_name() -> &'static str {
        "ZoneToUniverseMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            Self::Connect => "ZoneToUniverseMessage::Connect",
            Self::Disconnect => "ZoneToUniverseMessage::Disconnect",
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum UniverseToZoneMessage {
    Connected,
    ConnectRejected,
    Disconnect,
    UniverseBytes(Vec<u8>),
    Sync(Sync)
}

impl Messaging for UniverseToZoneMessage {
    fn message_type_name() -> &'static str {
        "UniverseToZoneMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            Self::Connected => "UniverseToZoneMessage::Connected",
            Self::ConnectRejected => "UniverseToZoneMessage::ConnectRejected",
            Self::Disconnect => "UniverseToZoneMessage::Disconnect",
            Self::UniverseBytes(_) => "UniverseToZoneMessage::UniverseBytes",
            Self::Sync(_) => "UniverseToZoneMessage::Sync"
        }
    }
}


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum WorldToZoneMessage {
    TimeFrame(NewTimeFrameMsg), // 2
    Connected,
    ConnectRejected,
    Disconnect,
    WorldBytes(TimeFrame, Vec<u8>),
    Sync(Sync)
}

impl Messaging for WorldToZoneMessage {
    fn message_type_name() -> &'static str {
        "WorldToZoneMessage"
    }

    fn message_name(&self) -> &'static str {
        match self {
            Self::TimeFrame(_) => "WorldToZoneMessage::TimeFrame",
            Self::Connected => "WorldToZoneMessage::Connected",
            Self::ConnectRejected => "WorldToZoneMessage::ConnectRejected",
            Self::Disconnect => "WorldToZoneMessage::Disconnect",
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