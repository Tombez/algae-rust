#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ServerOpcode {
    Update = 16,

}
#[derive(PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ClientOpcode {
    Play = 0,
    Spectate = 1,
    Unknown = 2,
    MousePos = 16,
    Split = 17,
    QKeyDown = 18,
    QKeyUp = 19,
    Eject = 21,
    VersionHandshake = 254,
    Handshake2 = 255,
}
impl From<u8> for ClientOpcode {
    fn from(v: u8) -> ClientOpcode {
        match v {
            0 => ClientOpcode::Play,
            1 => ClientOpcode::Spectate,
            _ => ClientOpcode::Unknown,
        }
    }
}
