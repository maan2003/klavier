use evdev::{InputEvent, Key};
use std::collections::HashMap;
use std::io;

use super::{Rule, RuleCtx};

struct Remaper {
    // a map of keycode to keycode
    remap: HashMap<u16, u16>,
}

pub type Map = Vec<Key>;

pub macro keys($($key:ident)*) {
    vec![$($key),*]
}

pub macro remap($($src:ident => $dst:ident)*) {{
    let src = keys! {$($src)*};
    let dst = keys! {$($dst)*};
    remap(&src, &dst)
}}

pub fn remap(src: &Map, dst: &Map) -> Box<dyn Rule> {
    Box::new(Remaper {
        remap: src
            .iter()
            .zip(dst.iter())
            .map(|(s, d)| (s.code(), d.code()))
            .collect(),
    })
}

impl Rule for Remaper {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        if let Some(&new_code) = self.remap.get(&event.code()) {
            let new_event = InputEvent::new(event.event_type(), new_code, event.value());
            ctx.forward(new_event);
        } else {
            ctx.forward(*event);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::keys::*;
    use crate::test_util::*;

    use super::*;

    test_case! {
        other_keys_fail_thru
        rule = remap!(
            A => B
            B => C
        ),
        input = (
            down C
            down D
            hold C
            up C
            up D
        ),
        output = (
            down C
            down D
            hold C
            up C
            up D
        ),
    }

    test_case! {
        remap_key
        rule = remap!(
            A => B
        ),
        input = (
            down A
            down B
            hold A
            up A
            up B
        ),
        output = (
            down B
            down B
            hold B
            up B
            up B
        ),
    }
}
