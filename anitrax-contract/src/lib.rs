#![no_std]

extern crate alloc;

use alloc::string::String;
use odra::prelude::*;

#[odra::module(events = [EventRegistered])]
pub struct AnitraxRegistry {
    event_count: Var<u64>,
}

#[odra::event]
pub struct EventRegistered {
    pub event_id: String,
    pub event_type: String,
    pub event_hash: String,
    pub timestamp: u64,
    pub sender: Address,
}

#[odra::module]
impl AnitraxRegistry {
    pub fn register_event(
        &mut self,
        event_id: String,
        event_type: String,
        event_hash: String,
        timestamp: u64,
    ) {
        let caller = self.env().caller();
        let count = self.event_count.get_or_default();
        self.event_count.set(count + 1);
        self.env().emit_event(EventRegistered {
            event_id,
            event_type,
            event_hash,
            timestamp,
            sender: caller,
        });
    }

    pub fn get_event_count(&self) -> u64 {
        self.event_count.get_or_default()
    }
}
