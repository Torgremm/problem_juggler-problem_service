pub mod problem;
pub mod solver;
pub mod user;

use anyhow::Result;
use async_trait::async_trait;
use std::convert::Infallible;
use std::fmt::Debug;
use std::future::Future;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
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

#[async_trait]
pub trait Listener: Send + Sync {
    type Recv: Debug + Send + for<'de> wincode::SchemaRead<'de, Dst = Self::Recv>;

    async fn listen<F, Fut>(&self, on_read: Arc<F>) -> Result<Infallible, anyhow::Error>
    where
        F: Fn(Self::Recv, &mut TcpStream) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let listener: TcpListener = TcpListener::bind(self.get_addr()).await?;
        let sem = Arc::new(Semaphore::new(100));
        const MAX_FRAME: u64 = 4 * 1024 * 1024;

        loop {
            let permit = match sem.clone().acquire_owned().await {
                Ok(p) => p,
                Err(e) => {
                    log::error!("Failed to aquire permit from semaphore: {}", e);
                    continue;
                }
            };
            let on_read = Arc::clone(&on_read);
            let (mut socket, _) = listener.accept().await?;
            tokio::spawn(async move {
                let _permit = permit;
                let mut len_buf = [0u8; 8];
                if socket.read_exact(&mut len_buf).await.is_err() {
                    return;
                }

                let len = u64::from_be_bytes(len_buf);
                if len > MAX_FRAME {
                    log::error!("Frame too large: {}", len);
                    return;
                }
                let mut buf = vec![0u8; len as usize];
                if let Err(e) = socket.read_exact(&mut buf).await {
                    log::error!("Failed to read exact from Tcp: {}", e);
                    return;
                }
                let r = match wincode::deserialize(&buf) {
                    Ok(r) => r,
                    Err(e) => {
                        log::error!("Failed to serialize a request: {}", e);
                        return;
                    }
                };
                (on_read)(r, &mut socket).await;
            });
        }
    }

    fn get_addr(&self) -> &str;
}
