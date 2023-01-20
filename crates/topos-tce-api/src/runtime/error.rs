use thiserror::Error;
use topos_core::uci::SubnetId;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("The pending stream {0} was not found")]
    PendingStreamNotFound(Uuid),

    #[error("Unable to push peer list")]
    UnableToPushPeerList,

    #[error("Unable to get source head certificate for subnet id {0:?}")]
    UnableToGetSourceHead(SubnetId),
}
