use std::sync::Arc;
use std::net::SocketAddr;

use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;

pub type Id = u64;

#[allow(dead_code)]
pub struct Connection {
    pub id: Id,
    stream: Arc<Mutex<TcpStream>>,
    address: SocketAddr,
    running: bool,
}

impl Connection {
    pub fn new(id: Id, stream: TcpStream, address: SocketAddr) -> Self {
        Connection {
            id,
            stream: Arc::new(Mutex::new(stream)),
            address,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.running {
            return Err("Connection already running".to_string());
        }
        self.running = true;

        let task_stream_mtx = self.stream.clone();

        tokio::spawn(async move {
            let mut task_stream = task_stream_mtx.lock().await;
            let (reader, mut writer) = task_stream.split();
            let mut buffer_reader = BufReader::new(reader);
            loop {
                let mut buf = vec![];
                match buffer_reader.read_until(b'\n', &mut buf).await {
                    Ok(0) => {},
                    Ok(_) => {
                        let s = String::from_utf8_lossy(&buf);
                        writer.write_all(s.as_bytes()).await.unwrap();
                    },
                    Err(e) => panic!("{:?}", e),
                }
            }
        });

        Ok(())
    }
}
