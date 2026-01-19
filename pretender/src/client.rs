use anyhow::Result;
use async_trait::async_trait;
use std::fmt::Debug;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
#[async_trait]
pub trait Client: Send + Sync {
    type Req: Debug + Send + wincode::SchemaWrite<Src = Self::Req>;
    type Recv: Debug + Send + for<'de> wincode::SchemaRead<'de, Dst = Self::Recv>;

    async fn req(&self, request: Self::Req) -> Result<Self::Recv> {
        let mut stream = TcpStream::connect(&self.get_addr()).await?;

        let req_bytes = wincode::serialize(&request)?;
        let len = (req_bytes.len() as u64).to_be_bytes();

        stream.write_all(&len).await?;
        stream.write_all(&req_bytes).await?;

        let mut len_buf = [0u8; 8];
        stream.read_exact(&mut len_buf).await?;

        let resp_len = u64::from_be_bytes(len_buf);

        let mut resp_buf = vec![0u8; resp_len as usize];
        stream.read_exact(&mut resp_buf).await?;

        let resp: Self::Recv = wincode::deserialize(&resp_buf)?;

        log::info!("Received valid response: {:?}", resp);

        Ok(resp)
    }

    fn get_addr(&self) -> &str;
}
