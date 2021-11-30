use std::collections::VecDeque;
use std::io;

use evdev::{uinput::VirtualDevice, Device, InputEvent};

use super::{Rule, RuleCtx};

pub struct Layer {
    rules: Vec<Box<dyn Rule>>,
    buf: VecDeque<InputEvent>,
}

impl Layer {
    pub fn new(rules: Vec<Box<dyn Rule>>) -> Self {
        Self {
            rules,
            buf: VecDeque::new(),
        }
    }

    // processes the events in self.buf, and puts the final events back in self.buf
    fn exec_rules(&mut self) -> io::Result<()> {
        for rule in &mut self.rules {
            // pop first evs.len() event, new events will be added by RuleCtx
            for _ in 0..self.buf.len() {
                let ev = self.buf.pop_front().unwrap();
                let mut ctx = RuleCtx::new(&mut self.buf);
                rule.event(&mut ctx, &ev)?;
            }
        }
        Ok(())
    }

    pub fn event(&mut self, ctx: &mut RuleCtx, ev: &InputEvent) -> io::Result<()> {
        self.buf.push_back(*ev);
        self.exec_rules()?;
        ctx.events.extend(self.buf.drain(..));
        Ok(())
    }

    pub fn event_device(&mut self, dev: &mut Device, out: &mut VirtualDevice) -> io::Result<()> {
        self.buf.extend(dev.fetch_events()?);
        self.exec_rules()?;

        // emit the events at the end
        let slices = self.buf.as_slices();
        out.emit(slices.0)?;
        out.emit(slices.1)?;
        self.buf.clear();
        Ok(())
    }
}
