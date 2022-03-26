use klavier::{keys::*, mods::*, rules::*, Remaper};
use std::{error::Error, env};

const DEVICE_PATH: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";

fn rules() -> Vec<Box<dyn Rule>> {
    let _src = keys!(
        TAB     Q     W     E     R     T     Y     U     I     O     P      LBRACE  RBRACE
        CAPS    A     S     D     F     G     H     J     K     L     SMCLN  SQT
        LSHT    Z     X     C     V     B     N     M     COMA  DOT   SLSH   RSHT
        LCTRL   SUP  LALT            SPC                RALT  RCTRL
    );
    // the colemak dh layout
    let _cmk = keys!(
        TAB     Q     W     F     P     B     J     L     U     Y     SMCLN  LBRACE  RBRACE
        CAPS    A     R     S     T     G     M     N     E     I     O      SQT
        LSHT    X     C     D     V     Z     K     H     COMA  DOT   SLSH   RSHT
        LCTRL   SUP  LALT            SPC                RALT  RCTRL
    );

    let ext = || {
        remap!(                                        I => UP
                                 F => LCTRL         J => LEFT   K => DOWN   L => RIGHT   SMCLN => BS
            W => CTRL + W
            X => CTRL + Z   C => CTRL + X   D => CTRL + C   V => CTRL + V
        )
    };

    let ws_map = || {
        remap!(
            U => ALT + N1     I => ALT + N2     O => ALT + N3
            J => F13   K => F14   L => F15   SMCLN => F16
        )
    };

    let syms = remap!(
        // left side
        A => SHIFT + COMA     // <
        Q => SHIFT + DOT      // >
        S => LBRACE           // [
        W => RBRACE           // ]
        D => SHIFT + LBRACE   // {
        E => SHIFT + RBRACE   // }
        F => SHIFT + N9       // (
        R => SHIFT + N0       // )
        // puncts
        G => SQT              // '
        V => SHIFT + SQT      // "
        C => SHIFT + N7       // &
        X => SHIFT + BLSH     // |

        // top right
        Y => SHIFT + EQUAL    // +
        U => GRV              // `
        I => SHIFT + SLSH     // ?
        O => SHIFT + N1       // !
        P => SHIFT + N4   // $

        // home right
        H => MINUS
        J => SHIFT + SMCLN    // -
        K => COMA
        L => DOT
        SMCLN => SHIFT + MINUS    // _

        // bottom right
        N => SHIFT + N8       // *
        M => EQUAL
        B => SLSH
    );

    vec![
        // remap(&src, cmk),
        mod_or_key(CAPS, CAPS, F9),
        mod_or_key(RALT, RALT, RET),
        if_held(CAPS, [if_held(S, [ws_map()], [ext()])], []),
        mod_or_key(SMCLN, F13, SMCLN),
        if_held(F13, [if_held(RALT, [ext()], [ws_map()])], []),
        if_held(RALT, [syms], []),
        // remap(&_src, _cmk),
        // rr_macro(KPSTAR, KPMINUS),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    let device_path = env::args().nth(1).unwrap_or_else(|| DEVICE_PATH.to_string());
    Remaper::new(device_path).rules(rules()).run()
}
