use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::WriteHalf;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::sync::Mutex;
use tokio_util::sync::CancellationToken;

pub type Id = u64;

pub const CHANNEL_BUFFER_SIZE: usize = 100;

#[allow(dead_code)]
pub struct Connection {
    pub id: Id,
    address: SocketAddr,
    stream: Arc<Mutex<TcpStream>>,
    sender: Option<Sender<String>>,
    running: bool,
}

impl Connection {
    pub fn new(id: Id, stream: TcpStream, address: SocketAddr) -> Self {
        Connection {
            id,
            address,
            stream: Arc::new(Mutex::new(stream)),
            sender: None,
            running: false,
        }
    }

    fn build_channel(&mut self) -> Result<Receiver<String>, String> {
        match self.sender {
            Some(_) => Err("Sender already set".to_string()),
            None => {
                let (sender, receiver) = mpsc::channel(CHANNEL_BUFFER_SIZE);
                self.sender = Some(sender);

                Ok(receiver)
            }
        }
    }

    pub async fn send_msg(&self, msg: String) -> Result<(), String> {
        self.sender
            .as_ref()
            .expect("Sender not initiated")
            .send(msg.clone())
            .await
            .expect("Failed to send message to connection");

        Ok(())
    }

    pub fn start(
        &mut self,
        server_sender: Sender<String>,
        cancel_token: CancellationToken,
    ) -> Result<(), String> {
        if self.running {
            return Err("Connection already running".to_string());
        }
        self.running = true;

        let mut receiver = self.build_channel()?;
        let task_stream_mtx = self.stream.clone();

        tokio::spawn(async move {
            let mut task_stream = task_stream_mtx.lock().await;
            let (reader, mut writer) = task_stream.split();
            let mut buffer_reader = BufReader::new(reader);

            loop {
                let mut buf = vec![];

                let buffer_read = buffer_reader.read_until(b'\n', &mut buf);
                let recv_msg = receiver.recv();

                tokio::select! {
                    reader_res = buffer_read => message_server(reader_res, &server_sender, &mut buf).await,
                    msg_res = recv_msg => listen_others(msg_res, &mut writer).await,
                    _ = cancel_token.cancelled() => {
                        shutdown_client(&mut writer).await;
                        break;
                    },
                }
            }
        });

        Ok(())
    }

    pub fn shutdown(self) {
        if let Some(sender) = self.sender {
            drop(sender);
        }
    }
}

async fn message_server<'a>(
    reader_result: io::Result<usize>,
    server_sender: &Sender<String>,
    buffer: &mut Vec<u8>,
) {
    match reader_result {
        Ok(0) => {}
        Ok(_) => {
            let s = String::from_utf8_lossy(&buffer);
            server_sender.send(s.to_string()).await.unwrap();
        }
        Err(e) => panic!("{:?}", e),
    }
}

async fn listen_others(msg: Option<String>, writer: &mut WriteHalf<'_>) {
    if let Some(msg) = msg {
        writer.write_all(msg.as_bytes()).await.unwrap();
    }
}

async fn shutdown_client(writer: &mut WriteHalf<'_>) {
    let shutdown_msg = "Shutting Down".to_string();
    writer.write_all(shutdown_msg.as_bytes()).await.unwrap();
}
