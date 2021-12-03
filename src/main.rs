use klavier::{keys::*, mods::*, rules::*, Remaper};
use std::error::Error;

const DEVICE_PATH: &str = "/dev/input/by-path/platform-i8042-serio-0-event-kbd";

fn rules() -> Vec<Box<dyn Rule>> {
    let src = keys!(
        TAB     Q     W     E     R     T     Y     U     I     O     P      LBRACE  RBRACE
        CAPS    A     S     D     F     G     H     J     K     L     SMCLN  SQT
        LSHT    Z     X     C     V     B     N     M     COMA  DOT   SLSH   RSHT
        LCTRL   SUP  LALT            SPC                RALT  RCTRL
    );
    // the colemak dh layout
    let cmk = keys!(
        TAB     Q     W     F     P     B     J     L     U     Y     SMCLN  LBRACE  RBRACE
        CAPS    A     R     S     T     G     M     N     E     I     O      SQT
        LSHT    X     C     D     V     Z     K     H     COMA  DOT   SLSH   RSHT
        LCTRL   SUP  LALT            SPC                RALT  RCTRL
    );

    let ext = remap!(                                        U => UP
                             T => LCTRL         N => LEFT   E => DOWN   I => RIGHT   O => BS
        X => CTRL + Z   C => CTRL + X   D => CTRL + C   V => CTRL + V
    );

    let ws_map = remap!(
        N => F13   E => F14   I => F15   O => F16
    );

    let syms = remap!(
        // left side
        A => SHIFT + COMA     // <
        Q => SHIFT + DOT      // >
        R => LBRACE           // [
        W => RBRACE           // ]
        S => SHIFT + LBRACE   // {
        F => SHIFT + RBRACE   // }
        T => SHIFT + N9       // (
        P => SHIFT + N0       // )
        // puncts
        G => SQT              // '
        V => SHIFT + SQT      // "
        D => SHIFT + N7       // &
        C => SHIFT + BLSH     // |

        // top right
        J => SHIFT + EQUAL    // +
        L => GRV              // `
        U => SHIFT + SLSH     // ?
        Y => SHIFT + N1       // !
        SMCLN => SHIFT + N4   // $

        // home right
        M => MINUS
        N => SHIFT + SMCLN    // -
        E => COMA
        I => DOT
        O => SHIFT + MINUS    // _

        // bottom right
        K => SHIFT + N8       // *
        H => EQUAL
        Z => SLSH
    );

    vec![
        remap(&src, cmk),
        magic_shift(),
        mod_or_key(CAPS, CAPS, F9),
        mod_or_key(RALT, RALT, RET),
        if_held(CAPS, [if_held(S, [ws_map], [ext])], []),
        if_held(RALT, [syms], []),
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    Remaper::new(DEVICE_PATH).rules(rules())
        .run()
}
