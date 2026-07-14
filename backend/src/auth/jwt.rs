mod state;
use state::User
use jsonwebtoken::

pub fn create_token(user: &User) -> Result<String, jsonwebtoken::