use libp2p::{
    identity::{self, Keypair},
    Multiaddr, PeerId,
};
use rstest::fixture;
use tokio::spawn;
use topos_test_sdk::networking::get_available_port;

use crate::{network::NetworkBuilder, NetworkClient, Runtime};

pub mod macros;

pub type PeerAddr = (PeerId, Multiaddr);

#[fixture]
pub async fn dummy_peer() -> (NetworkClient, PeerAddr) {
    let (key, addr_dummy) = local_peer(1);
    let dummy_peer = (key.public().to_peer_id(), addr_dummy.clone());

    let (client, _stream, runtime): (_, _, Runtime) = NetworkBuilder::default()
        .peer_key(key)
        .listen_addr(addr_dummy.clone())
        .exposed_addresses(addr_dummy)
        .build()
        .await
        .unwrap();

    spawn(runtime.run());
    (client, dummy_peer)
}

pub fn keypair_from_byte(seed: u8) -> Keypair {
    let mut bytes = [0u8; 32];
    bytes[0] = seed;

    identity::Keypair::ed25519_from_bytes(bytes).expect("Invalid keypair")
}

pub fn local_peer(peer_index: u8) -> (Keypair, Multiaddr) {
    let peer_id: Keypair = keypair_from_byte(peer_index);
    let port = get_available_port();
    let local_listen_addr: Multiaddr = format!("/ip4/127.0.0.1/tcp/{port}").parse().unwrap();
    (peer_id, local_listen_addr)
}
