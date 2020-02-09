use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    /**
     * The address to bind to.
     */
    pub bind_address : String = "127.0.0.1",
    /**
     * The port number to listen on.
     */
    pub port : i16 = 3510,
    /**
     * The path to the SQLite database file to use.
     */
    pub db_location : String = "./woppleblox.sqlite"
}
