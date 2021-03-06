use crate::request::prelude::*;
use twilight_model::user::User;

/// Get a user's information by id.
pub struct GetUser<'a> {
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
    target_user: String,
}

impl<'a> GetUser<'a> {
    pub(crate) fn new(http: &'a Client, target_user: impl Into<String>) -> Self {
        Self {
            fut: None,
            http,
            target_user: target_user.into(),
        }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetUser {
            target_user: self.target_user.clone(),
        });

        self.fut.replace(Box::pin(self.http.request_bytes(request)));

        Ok(())
    }
}

poll_req!(opt, GetUser<'_>, User);
