use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Sender};

pub type Id = u64;

const CHANNEL_BUFFER_SIZE: usize = 100;

#[allow(dead_code)]
pub struct Connection {
    pub id: Id,
    address: SocketAddr,
    stream: Arc<Mutex<TcpStream>>,
    running: bool,
}

impl Connection {
    pub fn new(id: Id, stream: TcpStream, address: SocketAddr) -> Self {
        Connection {
            id,
            address,
            stream: Arc::new(Mutex::new(stream)),
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
                echo(&mut buffer_reader, &mut writer, &mut buf).await;
            }
        });

        Ok(())
    }
}

async fn echo<'a>(buffer_reader: &mut BufReader<ReadHalf<'_>>, writer: &mut WriteHalf<'_>, buffer: &mut Vec<u8>) {
    match buffer_reader.read_until(b'\n', buffer).await {
        Ok(0) => {},
        Ok(_) => {
            let s = String::from_utf8_lossy(&buffer);
            writer.write_all(s.as_bytes()).await.unwrap();
        },
        Err(e) => panic!("{:?}", e),
    }
}
