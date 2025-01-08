#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Keycode {
    No = 0x00,
    A = 0x04,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M, // 0x10
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N1,
    N2,
    N3, // 0x20
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
    Enter,
    Escape,
    Backspace,
    Tab,
    Space,
    Minus,
    Equal,
    LeftBracket,
    RightBracket, // 0x30
    Backslash,
    NonUsHash,
    Semicolon,
    Apostrofe,
    Grave,
    Comma,
    Dot,
    Slash,
    CapsLock,
    F1,
    F2,
    F3,
    F4, // 0x40
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home, // 0x50
    PageUp,
    Delete,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLock,
    KpSlash,
    KpAsterisk,
    KpMinus, // 0x60
    KpPlus,
    KpEnter,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    Kp0, // 0x70
    KpDot,
    NonUsBackslash,
    Application,
    Power,
    KpEqual,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19, // 0x80
    F20,
    F21,
    F22,
    F23,
    F24,
    Execute,
    Help,
    Menu,
    Select,
    Stop,
    Again,
    Undo, // 0x90
    Cut,
    Copy,
    Paste,
    Find,
    Mute,
    VolumeUp,
    VolumeDown,
    LockingCapsLock,
    LockingNumLock,
    LockingScrollLock,
    KpComma, // 0xA0
    KpEqualSign,
    International1,
    International2,
    International3,
    International4,
    International5,
    International6,
    International7,
    International8,
    International9,
    Lang1,
    Lang2, // 0xB0
    Lang3,
    Lang4,
    Lang5,
    Lang6,
    Lang7,
    Lang8,
    Lang9,
    AlternateErase,
    SysReq,
    Cancel,
    Clear, // 0xC0
    Prior,
    Return,
    Separator,
    Out,
    Oper,
    ClearAgain,
    CrSel,
    ExSel,

    // Modifiers
    LCtrl = 0xE0,
    LShift,
    LAlt,
    LGui,
    RCtrl,
    RShift,
    RAlt,
    RGui, // 0xE7
}

impl Keycode {
    pub fn is_modifier(&self) -> bool {
        matches!(
            *self,
            Keycode::LCtrl
                | Keycode::LShift
                | Keycode::LAlt
                | Keycode::LGui
                | Keycode::RCtrl
                | Keycode::RShift
                | Keycode::RAlt
                | Keycode::RGui
        )
    }

    pub fn to_modifier_bitfield(&self) -> u8 {
        match *self {
            Keycode::LCtrl => 0x01,
            Keycode::LShift => 0x02,
            Keycode::LAlt => 0x04,
            Keycode::LGui => 0x08,
            Keycode::RCtrl => 0x10,
            Keycode::RShift => 0x20,
            Keycode::RAlt => 0x40,
            Keycode::RGui => 0x80,
            _ => 0x00,
        }
    }
}

impl From<Keycode> for u8 {
    fn from(value: Keycode) -> Self {
        value as u8
    }
}
