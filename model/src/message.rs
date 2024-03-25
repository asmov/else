pub mod auth;
use std::fmt::Display;
use strum;
use const_format::concatcp;
use crate::{timeframe::*, action::*, sync::*, descriptor::*};
pub use auth::*;

pub type MessageID = u16;
pub type ErrorCode = u8;

/// A const named MSGTYPENAME must be defined to use this.
macro_rules! msgname {
    ($name:literal) => {
        concatcp!(MSGTYPENAME, "::", $name)
    };
}


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
    const MESSAGE_TYPE_NAME: &'static str;

    fn message_type_name() -> &'static str {
        Self::MESSAGE_TYPE_NAME
    }

    fn message_name(&self) -> &'static str;
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ProtocolHeader {
    pub protocol: Protocol,
    pub version: u8
}

pub const PROTOCOL_VERSION: u8 = 1;

impl Messaging for ProtocolHeader {
    const MESSAGE_TYPE_NAME: &'static str = "ProtocolHeader";

    fn message_name(&self) -> &'static str {
        Self::MESSAGE_TYPE_NAME
    }
}

impl ProtocolHeader {
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
    pub last_downlink_uid: Option<UID>
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ListLinkableMsg {
    pub page: u16
}

#[derive(serde::Serialize, serde::Deserialize, Debug, strum::AsRefStr)]
pub enum ClientToZoneMessage {
    AuthRegister(AuthRegisterMsg),
    AuthRequest(AuthRequestMsg),
    AuthAnswer(AuthAnswerMsg),
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
    const MESSAGE_TYPE_NAME: &'static str = "ClientToZoneMessage";

    fn message_name(&self) -> &'static str {
        const MSGTYPENAME: &'static str = ClientToZoneMessage::MESSAGE_TYPE_NAME;
        match self {
            Self::AuthRegister(_) => msgname!("AuthRegister"),
            Self::AuthRequest(_) => msgname!("AuthRequest"),
            Self::AuthAnswer(_) => msgname!("AuthAnswer"),
            Self::Disconnect => msgname!("Disconnect"),
            Self::ListLinkable(_) => msgname!("ListLinkable"),
            Self::Downlink(_) => msgname!("Downlink"),
            Self::Unlink => msgname!("Downlink"),
            Self::Action(_) => msgname!("Action"),
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
    /// Response: Request to connect to the world through this server has been accepted.
    Connected,
    /// Response: Request to connect to the world through this server has been rejected.
    ConnectRejected,
    /// A timeframe (tick) has elapsed
    TimeFrame(NewTimeFrameMsg),
    /// The client has to further prove its identity to authenticate.
    AuthChallenge(AuthChallengeMsg),
    /// Authentication has either succeeded or failed.
    Authorized(AuthorizedMsg),
    AuthRejected,
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
    const MESSAGE_TYPE_NAME: &'static str = "ZoneToClientMessage";

    fn message_name(&self) -> &'static str {
        const MSGTYPENAME: &'static str = ZoneToClientMessage::MESSAGE_TYPE_NAME;
        match self {
            Self::TimeFrame(_) => msgname!("TimeFrame"),
            Self::AuthChallenge(_) => msgname!("AuthResponse"),
            Self::Authorized(_) => msgname!("AuthResult"),
            Self::Connected => msgname!("Connected"),
            Self::ConnectRejected => msgname!("ConnectRejected"),
            Self::InitInterfaceView(_, _)=> msgname!("InitInterfaceView"),
            Self::Sync(_) => msgname!("Sync"),
            Self::Disconnect => msgname!("Disconnect"),
            Self::Linkable(_) => msgname!("Linkable"),
            Self::DownlinkApproved(_,_) => msgname!("DownlinkApproved"),
            Self::DownlinkRejected(_) => msgname!("DownlinkRejected"),
            Self::Unlinked(_) => msgname!("Unlinked"),
            Self::ActionApproved(_,_) => msgname!("ActionApproved"),
            Self::ActionRejected(_) => msgname!("ActionRejected"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ZoneToWorldMessage {
    Connect,
    Disconnect,
}

impl Messaging for ZoneToWorldMessage {
    const MESSAGE_TYPE_NAME: &'static str = "ZoneToWorldMessage";

    fn message_name(&self) -> &'static str {
        const MSGTYPENAME: &'static str = ZoneToWorldMessage::MESSAGE_TYPE_NAME;
        match self {
            Self::Connect => msgname!("Connect"),
            Self::Disconnect => msgname!("Disconnect"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum ZoneToUniverseMessage {
    Connect,
    Disconnect,
    AuthRequest(AuthRequestMsg),
}

impl Messaging for ZoneToUniverseMessage {
    const MESSAGE_TYPE_NAME: &'static str = "ZoneToUniverseMessage";

    fn message_name(&self) -> &'static str {
        const MSGTYPENAME: &'static str = ZoneToUniverseMessage::MESSAGE_TYPE_NAME;
        match self {
            Self::Connect => msgname!("Connect"),
            Self::Disconnect => msgname!("Disconnect"),
            Self::AuthRequest(_) => msgname!("AuthRequest"),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum UniverseToZoneMessage {
    Connected,
    ConnectRejected,
    Disconnect,
}

impl Messaging for UniverseToZoneMessage {
    const MESSAGE_TYPE_NAME: &'static str = "UniverseToZoneMessage";

    fn message_name(&self) -> &'static str {
        const MSGTYPENAME: &'static str = UniverseToZoneMessage::MESSAGE_TYPE_NAME;
        match self {
            Self::Connected => msgname!("Connected"),
            Self::ConnectRejected => msgname!("ConnectRejected"),
            Self::Disconnect => msgname!("Disconnect"),
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
    const MESSAGE_TYPE_NAME: &'static str = "WorldToZoneMessage";

    fn message_name(&self) -> &'static str {
        const MSGTYPENAME: &'static str = WorldToZoneMessage::MESSAGE_TYPE_NAME;
        match self {
            Self::TimeFrame(_) => msgname!("TimeFrame"),
            Self::Connected => msgname!("Connected"),
            Self::ConnectRejected => msgname!("ConnectRejected"),
            Self::Disconnect => msgname!("Disconnect"),
            Self::WorldBytes(_,_) => msgname!("WorldBytes"),
            Self::Sync(_) => msgname!("Sync")
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