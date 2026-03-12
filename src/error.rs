use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(()),
    // -- api call errors
    InvalidLocation,

    // -- user model errors
    LoginFailed { username: String },
}
