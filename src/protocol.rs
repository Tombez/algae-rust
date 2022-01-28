#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ServerOpcode { // From server to client
    Update = 16,
}
#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ClientOpcode {
    Play = 0,
    Spectate = 1,
    MousePos = 16,
    Split = 17,
    QKeyDown = 18,
    QKeyUp = 19,
    Eject = 21,
    Unknown,
    VersionHandshake = 254,
    Handshake2 = 255,
}
impl From<u8> for ClientOpcode {
    fn from(v: u8) -> ClientOpcode {
        match v {
            0 => ClientOpcode::Play,
            1 => ClientOpcode::Spectate,
            16 => ClientOpcode::MousePos,
            17 => ClientOpcode::Split,
            18 => ClientOpcode::QKeyDown,
            19 => ClientOpcode::QKeyUp,
            21 => ClientOpcode::Eject,
            254 => ClientOpcode::VersionHandshake,
            255 => ClientOpcode::Handshake2,
            _ => ClientOpcode::Unknown,
        }
    }
}
