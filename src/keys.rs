#![allow(dead_code)]
use evdev::Key;

macro_rules! def_keys {
    ($($name:ident => $value:ident),* $(,)?) => {
        $(
            pub const $name: Key = Key::$value;
        )*
        pub const ALL_KEYS: &[Key] = &[
            $(Key::$value),*
        ];
    };
}

def_keys! {
    __ => KEY_RESERVED,
    ESC => KEY_ESC,
    N1 => KEY_1,
    N2 => KEY_2,
    N3 => KEY_3,
    N4 => KEY_4,
    N5 => KEY_5,
    N6 => KEY_6,
    N7 => KEY_7,
    N8 => KEY_8,
    N9 => KEY_9,
    N0 => KEY_0,

    Q => KEY_Q,
    W => KEY_W,
    E => KEY_E,
    R => KEY_R,
    T => KEY_T,
    Y => KEY_Y,
    U => KEY_U,
    I => KEY_I,
    O => KEY_O,
    P => KEY_P,
    A => KEY_A,
    S => KEY_S,
    D => KEY_D,
    F => KEY_F,
    G => KEY_G,
    H => KEY_H,
    J => KEY_J,
    K => KEY_K,
    L => KEY_L,
    Z => KEY_Z,
    X => KEY_X,
    C => KEY_C,
    V => KEY_V,
    B => KEY_B,
    N => KEY_N,
    M => KEY_M,

    SPC => KEY_SPACE,
    BS => KEY_BACKSPACE,
    TAB => KEY_TAB,
    RET => KEY_ENTER,

    SMCLN => KEY_SEMICOLON,
    SQT => KEY_APOSTROPHE,
    LSHT => KEY_LEFTSHIFT,
    GRV => KEY_GRAVE,
    BLSH => KEY_BACKSLASH,
    COMA => KEY_COMMA,
    DOT => KEY_DOT,
    LBRACE => KEY_LEFTBRACE,
    RBRACE => KEY_RIGHTBRACE,
    MINUS => KEY_MINUS,
    EQUAL => KEY_EQUAL,
    SLSH => KEY_SLASH,

    LCTRL => KEY_LEFTCTRL,
    SUP => KEY_LEFTMETA,
    LALT => KEY_LEFTALT,
    RALT => KEY_RIGHTALT,
    RCTRL => KEY_RIGHTCTRL,
    RSHT => KEY_RIGHTSHIFT,
    CAPS => KEY_CAPSLOCK,

    F1 => KEY_F1,
    F2 => KEY_F2,
    F3 => KEY_F3,
    F4 => KEY_F4,
    F5 => KEY_F5,
    F6 => KEY_F6,
    F7 => KEY_F7,
    F8 => KEY_F8,
    F9 => KEY_F9,
    F10 => KEY_F10,
    F11 => KEY_F11,
    F12 => KEY_F12,
    F13 => KEY_F13,
    F14 => KEY_F14,
    F15 => KEY_F15,
    F16 => KEY_F16,
    F17 => KEY_F17,
    F18 => KEY_F18,
    F19 => KEY_F19,
    F20 => KEY_F20,
    F21 => KEY_F21,
    F22 => KEY_F22,
    F23 => KEY_F23,
    F24 => KEY_F24,

    LEFT => KEY_LEFT,
    RIGHT => KEY_RIGHT,
    UP => KEY_UP,
    DOWN => KEY_DOWN,
}
