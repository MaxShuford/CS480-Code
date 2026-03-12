use derive_more::From;

pub type AppResult<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    Custom(()),
    // -- api call errors
    InvalidLocation,

    // -- user model errors
    LoginFailed { username: String },
    UserExists { username: String },
    IncorrectPassword,

    // -- favorite model errors
    MaxRoutesExceeded,
    DeleteUnsuccessful,
    RouteNotFound,

    // -- translate mysql errors into custom error type
    #[from]
    MySql(mysql::Error),
}
