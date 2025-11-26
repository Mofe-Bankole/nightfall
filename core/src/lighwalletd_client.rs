use std::time::Duration;
// Use tonic to define a lightwalletd client (or use an existing generated client if you have proto)
use tonic::{
    IntoRequest, Response,
    transport::{Channel, ClientTlsConfig, Endpoint},
};
use zcash_client_backend::proto::service::{
    BlockId, ChainSpec, compact_tx_streamer_client::CompactTxStreamerClient,
};

#[derive(Debug)]
pub struct LightwalletdClient {
    channel: Endpoint,
    tls_config: ClientTlsConfig,
    grpc: String,
    port: u16,
    use_tls: bool,
}

impl Clone for LightwalletdClient {
    fn clone(&self) -> Self {
        Self {
            tls_config: self.tls_config.clone(),
            channel: self.channel.clone(),
            grpc: self.grpc.clone(),
            port: self.port,
            use_tls: self.use_tls,
        }
    }
}

impl LightwalletdClient {
    pub fn new(grpc: String, port: u16) -> Self {
        Self {
            channel: Channel::from_shared(format!("{}:{}", grpc, port)).unwrap(),
            tls_config: ClientTlsConfig::new().with_native_roots(),
            grpc,
            port,
            use_tls: false,
        }
    }

    pub async fn connect(&self) -> anyhow::Result<CompactTxStreamerClient<Channel>> {
        let _endpoint = format!("{}:{}", self.grpc, self.port);

        Ok(CompactTxStreamerClient::new(
            self.channel
                .clone()
                .tls_config(self.tls_config.clone())?
                .connect_timeout(Duration::from_secs(10))
                .connect()
                .await?,
        ))
    }

    pub async fn get_latest_block_raw(&self) -> anyhow::Result<Response<BlockId>> {
        let mut client = self.connect().await?;

        // get_latest_block expects a ChainSpec not a BlockId.a as time of writing we use default (empty) ChainSpec.
        let response = client
            .get_latest_block(ChainSpec::default().into_request())
            .await?;

        // height is returned as u64, so cast as u32.
        Ok(response)
    }

    pub fn broadcast_transaction(&mut self) -> anyhow::Result<Response<BlockId>> {
        todo!()
    }
}
