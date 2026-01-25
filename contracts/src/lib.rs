pub mod problem;
pub mod solver;
pub mod user;

use anyhow::Result;
use std::any::type_name;
use std::fmt::Debug;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Semaphore;
pub trait Client: Send + Sync {
    type Req: Debug + Send + wincode::SchemaWrite<Src = Self::Req>;
    type Recv: Debug + Send + for<'de> wincode::SchemaRead<'de, Dst = Self::Recv>;

    fn req(&self, request: Self::Req) -> impl Future<Output = Result<Self::Recv>> {
        async move {
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

            log::info!("{}\nReceived valid response", type_name::<Self>());

            Ok(resp)
        }
    }

    fn get_addr(&self) -> &str;
}

pub trait Listener: Send + Sync {
    type Recv: Debug + Send + for<'de> wincode::SchemaRead<'de, Dst = Self::Recv>;

    fn listen<F, Fut>(&self, on_read: F) -> impl Future<Output = Result<()>>
    where
        F: Fn(Self::Recv, TcpStream) -> Fut + Clone + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        async move {
            let listener: TcpListener = TcpListener::bind(self.get_addr()).await?;
            log::info!("{} listening on {}", type_name::<Self>(), self.get_addr());
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
                let (mut socket, _) = listener.accept().await?;
                let on_read = on_read.clone();
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
                    (on_read)(r, socket).await;
                });
            }
        }
    }

    fn get_addr(&self) -> &str;
}
