extern crate anyhow;
extern crate sd_notify;
extern crate tokio;

use core::str;
use std::env;
use std::io::ErrorKind;
use std::os::unix::net::{SocketAddr, UnixStream as StdStream};
use std::path::Path;

use anyhow::{Error, Result, bail};
use sd_notify::NotifyState;
use tokio::io::{AsyncBufReadExt, BufReader, BufWriter, copy_bidirectional};
use tokio::net::{UnixListener, UnixStream};

#[tokio::main]
async fn main() -> Result<()> {
  // Set up listener against file descriptor(s)
  let mut listenfd = listenfd::ListenFd::from_env();
  let listener = UnixListener::from_std(
    listenfd
      .take_unix_listener(0)?
      .expect("Couldn't create listener"),
  )?;
  sd_notify::notify(false, &[NotifyState::Ready])?;

  // Handle incoming connections
  while let Ok((mut sock, _)) = listener.accept().await {
    tokio::spawn(async move {
      handle_connection(&mut sock).await?;
      Ok::<_, Error>(())
    });
  }

  Ok(())
}

async fn handle_connection(incoming_rpc_socket: &mut UnixStream) -> Result<()> {
  let mut reader = BufReader::new(incoming_rpc_socket);
  let (remote_domain, _) = parse_qrexec_input(&mut reader).await?;
  handle_factory(remote_domain.as_str()).await?;

  let daemon_sock = UnixStream::connect(format!(
    "{}/split-onlykey/onlykey-agent/{}/S.ssh-agent",
    env::var("RUNTIME_DIRECTORY")?,
    remote_domain
  ))
  .await?;
  let mut writer = BufWriter::new(daemon_sock);

  // Connect reader/writer socket buffers
  copy_bidirectional(&mut reader, &mut writer).await?;

  Ok(())
}

async fn parse_qrexec_input(reader: &mut BufReader<&mut UnixStream>) -> Result<(String, String)> {
  // Set up buffers
  let mut buffer: Vec<u8> = vec![];

  // Read the until header delimiter
  let n = reader.read_until(b'\0', &mut buffer).await?;

  // Parse the header fields
  let [rpc_service, remote_domain] = str::from_utf8(&buffer[..n - 1])?
    .split(" ")
    .collect::<Vec<_>>()[..2]
  else {
    bail!("Couldn't read socket input into buffer")
  };

  Ok((String::from(remote_domain), String::from(rpc_service)))
}

async fn get_factory_socket_addr() -> Result<SocketAddr> {
  let runtime_path = env::var("RUNTIME_DIRECTORY")?;
  let path = Path::new(&runtime_path).with_file_name("daemon-factory.sock");
  let sock_addr = SocketAddr::from_pathname(path)?;

  Ok(sock_addr)
}

async fn handle_factory(remote_domain: &str) -> Result<String> {
  let socket_addr = get_factory_socket_addr().await?;
  let std_stream = StdStream::connect_addr(&socket_addr)?;
  let stream = UnixStream::from_std(std_stream)?;
  let mut msg = vec![0u8; 1024];

  loop {
    stream.writable().await?;
    match stream.try_write(remote_domain.as_bytes()) {
      Ok(_) => {
        break;
      }

      Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
        continue;
      }

      Err(e) => {
        return Err(e.into());
      }
    }
  }

  loop {
    stream.readable().await?;
    match stream.try_read(&mut msg) {
      Ok(n) => {
        msg.truncate(n);
        break;
      }

      Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
        continue;
      }

      Err(e) => {
        return Err(e.into());
      }
    }
  }

  Ok(String::from_utf8(msg)?)
}
