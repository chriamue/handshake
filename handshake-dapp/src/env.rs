#[cfg(not(feature = "github"))]
pub const URL: &str = "http://localhost:8080/";

#[cfg(feature = "github")]
pub const URL: &str = "https://blog.chriamue.de/handshake/";

#[cfg(not(feature = "github"))]
pub const CONTRACT: &str = "5C8iyAnGiuWN2Dc4MZJMwDkw8U6CGYAJsDru5zFK5bUFof4Y";

#[cfg(feature = "github")]
pub const CONTRACT: &str = "5ECXVCvKkg3hafPo9YQEatdUgN5tZ3rtgrStHfHAwEErUgTv";
