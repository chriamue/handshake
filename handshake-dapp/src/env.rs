#[cfg(not(feature = "github"))]
pub const URL: &str ="http://localhost:8080";

#[cfg(feature = "github")]
pub const URL: &str ="https://github.com/";
