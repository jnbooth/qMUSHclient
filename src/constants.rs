pub mod branding {
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const APPNAME: &str = env!("CARGO_PKG_NAME");
    pub const AUTHOR: &str = "Joshua Booth";
    pub const REPO: &str = env!("CARGO_PKG_REPOSITORY");
    pub const MUD_LIST: &str = "https://www.topmudsites.com/";
}

pub mod config {
    pub const MAX_RECENT: usize = 5;
    pub const COMPRESS_BUFFER: usize = 1024 * 10;
    pub const SOCKET_BUFFER: usize = 1024 * 9; // needs to be <= COMPRESS_BUFFER
}