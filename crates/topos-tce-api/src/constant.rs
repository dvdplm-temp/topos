use lazy_static::lazy_static;

lazy_static! {
    /// Size of the grpc internal command channel
    pub static ref GRPC_COMMAND_CHANNEL_SIZE: usize =
        std::env::var("TOPOS_API_COMMAND_CHANNEL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2048);
}
