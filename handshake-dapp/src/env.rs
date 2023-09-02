#[cfg(not(feature = "github"))]
pub const URL: &str = "http://localhost:8080/";

#[cfg(feature = "github")]
pub const URL: &str = "https://blog.chriamue.de/handshake/";

#[cfg(not(feature = "github"))]
pub const CONTRACT: &str = "5C93YodqcxnCBRq6V7fCofvwLiLshkFqrhVJTzjoXMvjQRo7";

#[cfg(feature = "github")]
pub const CONTRACT: &str = "5C93YodqcxnCBRq6V7fCofvwLiLshkFqrhVJTzjoXMvjQRo7";
