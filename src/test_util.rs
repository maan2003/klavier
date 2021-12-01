use evdev::{EventType, InputEvent};
use std::collections::VecDeque;

use crate::ext::KeyEvent;
use crate::ext::KeyEventExt;
use crate::rules::Rule;
use crate::rules::RuleCtx;
use pretty_assertions::Comparison;

pub macro events {
    (
        value = [{ $value: expr }]
        input = [{ up $key:ident $($rest:tt)* }]
    ) => {
        $value.push_back(key_event_to_input_event(KeyEvent::Release($key)));
        events!(value = [{ $value }] input = [{ $($rest)* }]);
    },

    (
        value = [{ $value: expr }]
        input = [{ down $key:ident $($rest:tt)* }]
    ) => {
        $value.push_back(key_event_to_input_event(KeyEvent::Press($key)));
        events!(value = [{ $value }] input = [{ $($rest)* }]);
    },

    (
        value = [{ $value: expr }]
        input = [{ $key:ident $($rest:tt)* }]
    ) => {
        $value.push_back(key_event_to_input_event(KeyEvent::Press($key)));
        $value.push_back(key_event_to_input_event(KeyEvent::Release($key)));
        events!(value = [{ $value }] input = [{ $($rest)* }]);
    },

    (
        value = [{ $value: expr }]
        input = [{ }]
    ) => {},

    ($($tt:tt)*) => {{
        let mut value = VecDeque::new();
        events!(value = [{ value }] input = [{ $($tt)* }]);
        value
    }}
}

pub fn key_event_to_input_event(ev: KeyEvent) -> InputEvent {
    match ev {
        KeyEvent::Press(key) => InputEvent::new(EventType::KEY, key.code(), 1),
        KeyEvent::Hold(key) => InputEvent::new(EventType::KEY, key.code(), 2),
        KeyEvent::Release(key) => InputEvent::new(EventType::KEY, key.code(), 0),
    }
}

pub macro assert_events($left:expr, $right:expr) {
    let left = $left;
    let right = $right;
    let mut eq = true;
    if left.len() != right.len() {
        eq = false;
    }
    for (left, right) in left.iter().zip(right.iter()) {
        if !cmp_event(*left, *right) {
            eq = false;
        }
    }

    if !eq {
        panic!(
            "assertion failed!\n{}",
            Comparison::new(&Printer(&left), &Printer(&right))
        );
    }
}

pub fn cmp_event(left: InputEvent, right: InputEvent) -> bool {
    left.kind() == right.kind() && left.code() == right.code() && left.value() == right.value()
}

pub struct Printer<'a>(pub &'a VecDeque<InputEvent>);

impl std::fmt::Debug for Printer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for event in self.0.iter() {
            match event.key_event().unwrap() {
                KeyEvent::Press(key) => writeln!(f, "Down {:?}", key)?,
                KeyEvent::Release(key) => writeln!(f, "Up {:?}", key)?,
                KeyEvent::Hold(key) => writeln!(f, "Hold {:?}", key)?,
            }
        }
        Ok(())
    }
}

pub macro test_case(
    $name:ident
    rule = $rule:expr,
    input = ($($input:tt)*),
    output = ($($output:tt)*),
) {
    #[test]
    fn $name() {
        test_case_impl($rule, events!($($input)*), events!($($output)*));
    }
}

pub fn test_case_impl(
    mut rule: Box<dyn Rule>,
    mut input: VecDeque<InputEvent>,
    output: VecDeque<InputEvent>,
) {
    for _ in 0..input.len() {
        let event = input.pop_front().unwrap();
        let mut ctx = RuleCtx::new(&mut input);
        rule.event(&mut ctx, &event).unwrap();
    }

    assert_events!(input, output);
}
