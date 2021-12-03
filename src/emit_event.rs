use evdev::Key;
use std::ops::Add;

use crate::keys::*;
use crate::{key_state::KeyState, rules::RuleCtx};

pub trait EmitEvent {
    fn emit(&self, ctx: &mut RuleCtx, state: KeyState);
}

impl EmitEvent for Key {
    fn emit(&self, ctx: &mut RuleCtx, state: KeyState) {
        match state {
            KeyState::Press => ctx.key_down(*self),
            KeyState::Release => ctx.key_up(*self),
            KeyState::Hold => ctx.key_hold(*self),
        }
    }
}

impl<E: EmitEvent + ?Sized> EmitEvent for Box<E> {
    fn emit(&self, ctx: &mut RuleCtx, state: KeyState) {
        (**self).emit(ctx, state)
    }
}
pub const SHIFT: Mod = Mod(LSHT);
pub const CTRL: Mod = Mod(LCTRL);
pub const ALT: Mod = Mod(LALT);
pub const SUPER: Mod = Mod(SUP);

pub struct Mod(Key);

impl<E: EmitEvent> Add<E> for Mod {
    type Output = WithMod<E>;

    fn add(self, rhs: E) -> Self::Output {
        WithMod {
            mod_: self,
            inner: rhs,
        }
    }
}

pub struct WithMod<E> {
    inner: E,
    mod_: Mod,
}

impl<E> EmitEvent for WithMod<E>
where
    E: EmitEvent,
{
    fn emit(&self, ctx: &mut RuleCtx, state: KeyState) {
        ctx.key_down(self.mod_.0);
        self.inner.emit(ctx, state);
        ctx.key_up(self.mod_.0);
    }
}
