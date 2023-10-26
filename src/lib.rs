#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "nightly", feature(slice_take))]
#![feature(async_closure)]
#![feature(io_error_more)]
#![feature(io_error_uncategorized)]
// doc warnings that aren't on by default
#![warn(missing_docs)]
#![warn(rustdoc::unescaped_backticks)]

mod ffi;
mod rust;

// All types should be at the root as `uniffi` only exposes one namespace to foreign code
pub use ffi::channel::statuses::{ChannelStatusJoinError, ChannelStatuses};
pub use ffi::channel::{
    CallError, Channel, ChannelJoinError, ChannelStatus, EventPayload, Events, EventsError,
};
pub use ffi::io::error::IoError;
pub use ffi::json::{JSONDeserializationError, JSON};
pub use ffi::message::{Event, Payload, PhoenixEvent};
pub use ffi::socket::{ConnectError, Socket, SocketStatus};
pub use ffi::topic::Topic;
pub use ffi::web_socket::error::WebSocketError;
pub use ffi::web_socket::protocol::WebSocketMessage;
pub use ffi::Error;

uniffi::include_scaffolding!("phoenix_channels_client");
