pub use crate::{
    client::Client,
    error::Result,
    pending::{Pending, PendingBody},
    routing::Route,
};
pub use http::Method;
pub use serde::{Deserialize, Serialize};
pub use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
pub(super) use super::Request;