use evdev::{InputEvent, Key};
use std::collections::HashMap;
use std::io;

use crate::{emit_event::EmitEvent, key_state::KeyEventExt};

use super::{Rule, RuleCtx};

pub struct Remaper<E> {
    // a map of keycode to keycode
    remap: HashMap<u16, E>,
}

pub type Map<E> = Vec<E>;

pub macro keys($($key:ident)*) {
    vec![$($key),*]
}

pub macro remap($($src:ident => $dst:expr)*) {{
    let src = keys![$($src)*];
    let dst = vec![$(Box::new($dst) as Box<dyn EmitEvent>),*];

    remap(&src, dst)
}}

pub fn remap<E: EmitEvent + 'static>(src: &Map<Key>, dst: Map<E>) -> Box<dyn Rule> {
    Box::new(Remaper {
        remap: src
            .iter()
            .zip(dst.into_iter())
            .map(|(s, d)| (s.code(), d))
            .collect(),
    })
}

impl<E: EmitEvent> Rule for Remaper<E> {
    fn event(&mut self, ctx: &mut RuleCtx, event: &InputEvent) -> io::Result<()> {
        if let Some(em) = self.remap.get(&event.code()) {
            let state = event.key_state().unwrap();
            em.emit(ctx, state);
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
