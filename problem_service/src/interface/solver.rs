use anyhow::Result;
use async_trait::async_trait;
use contracts::SolveRequest;
use contracts::SolveResponse;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[async_trait]
pub trait SolverClient: Send + Sync {
    async fn solve(&self, request: SolveRequest) -> Result<SolveResponse>;
}

pub struct RemoteSolverClient {
    addr: &'static str,
}

impl RemoteSolverClient {
    pub fn new(addr: &'static str) -> Self {
        Self { addr }
    }
}

impl Default for RemoteSolverClient {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:4000",
        }
    }
}

#[async_trait]
impl SolverClient for RemoteSolverClient {
    async fn solve(&self, request: SolveRequest) -> Result<SolveResponse> {
        let mut stream = TcpStream::connect(&self.addr).await?;

        let req_bytes = wincode::serialize(&request)?;
        let len = (req_bytes.len() as u64).to_be_bytes();

        stream.write_all(&len).await?;
        stream.write_all(&req_bytes).await?;

        let mut len_buf = [0u8; 8];
        stream.read_exact(&mut len_buf).await?;

        let resp_len = u64::from_be_bytes(len_buf);

        let mut resp_buf = vec![0u8; resp_len as usize];
        stream.read_exact(&mut resp_buf).await?;

        let resp: SolveResponse = wincode::deserialize(&resp_buf)?;

        Ok(resp)
    }
}
