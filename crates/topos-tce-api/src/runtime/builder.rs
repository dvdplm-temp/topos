use futures::Stream;
use std::{collections::HashMap, net::SocketAddr};
use tokio::{
    spawn,
    sync::{mpsc, RwLock},
};
use tokio_stream::wrappers::ReceiverStream;
use topos_core::api::tce::v1::StatusResponse;

use crate::{grpc::builder::ServerBuilder, Runtime, RuntimeClient, RuntimeEvent};

#[derive(Default)]
pub struct RuntimeBuilder {
    grpc_socket_addr: Option<SocketAddr>,
    status: Option<RwLock<StatusResponse>>,
}

impl RuntimeBuilder {
    pub fn serve_addr(mut self, addr: SocketAddr) -> Self {
        self.grpc_socket_addr = Some(addr);

        self
    }

    pub fn tce_status(mut self, status: RwLock<StatusResponse>) -> Self {
        self.status = Some(status);

        self
    }

    pub async fn build_and_launch(self) -> (RuntimeClient, impl Stream<Item = RuntimeEvent>) {
        let (command_sender, internal_runtime_command_receiver) = mpsc::channel(2048);
        let (api_event_sender, api_event_receiver) = mpsc::channel(2048);

        let (health_reporter, tce_status, grpc) = ServerBuilder::default()
            .command_sender(command_sender)
            .serve_addr(self.grpc_socket_addr)
            .build()
            .await;

        let (command_sender, runtime_command_receiver) = mpsc::channel(2048);

        let runtime = Runtime {
            active_streams: HashMap::new(),
            pending_streams: HashMap::new(),
            subnet_subscription: HashMap::new(),
            internal_runtime_command_receiver,
            runtime_command_receiver,
            health_reporter,
            api_event_sender,
        };

        spawn(grpc);
        spawn(runtime.launch());

        (
            RuntimeClient {
                command_sender,
                tce_status,
            },
            ReceiverStream::new(api_event_receiver),
        )
    }

    pub fn set_grpc_socket_addr(mut self, socket: Option<SocketAddr>) -> Self {
        self.grpc_socket_addr = socket;

        self
    }
}
