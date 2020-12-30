extern crate crossbeam_channel as cb_channel;

mod common;
mod dispatch;
mod internal_senders;
mod macros;
mod manager;

pub use dispatch::Dispatch;
pub use internal_senders::Senders;
pub use manager::{Message, EventsManager, start};
