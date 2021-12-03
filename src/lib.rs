// not really needed, just using for ease of use
#![feature(decl_macro)]

mod emit_event;
mod key_state;
pub mod keys;
mod layer;
pub mod rules;

pub mod mods {
    pub use crate::emit_event::{ALT, CTRL, SHIFT, SUPER};
}

#[cfg(test)]
mod test_util;

use evdev::uinput::VirtualDeviceBuilder;
use evdev::{AttributeSet, Device};
use layer::Layer;
use rules::Rule;
use rules::RuleCtx;
use std::error::Error;

pub struct Remaper {
    rules: Vec<Box<dyn Rule>>,
    dev_path: String,
    virtual_dev: String,
}

impl Remaper {
    pub fn new(dev_path: impl Into<String>) -> Remaper {
        Remaper {
            rules: Vec::new(),
            dev_path: dev_path.into(),
            virtual_dev: String::from("klavier"),
        }
    }

    pub fn rules(mut self, rules: Vec<Box<dyn Rule>>) -> Remaper {
        self.rules = rules;
        self
    }

    pub fn run(self) -> Result<(), Box<dyn Error>> {
        let mut dev = Device::open(&self.dev_path)?;
        let mut all_keys = AttributeSet::new();
        for keys in dev.supported_keys().unwrap().iter() {
            all_keys.insert(keys);
        }
        for key in keys::ALL_KEYS {
            all_keys.insert(*key);
        }

        let mut out = VirtualDeviceBuilder::new()?
            .name(&self.virtual_dev)
            .with_keys(&all_keys)?
            .build()?;

        dev.grab()?;

        let mut root_layer = Layer::new(self.rules);
        loop {
            root_layer.event_device(&mut dev, &mut out)?;
        }
    }
}
