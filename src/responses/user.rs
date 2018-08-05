use super::raw::user;

#[derive(Clone, Debug)]
pub struct User {}

impl From<user::User> for User {
    fn from(user: user::User) -> Self {
        unimplemented!()
    }
}