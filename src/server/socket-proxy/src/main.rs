use anyhow::{bail, Error, Result};
use core::str;
use tokio::{
  io::{copy, stdout, AsyncBufReadExt, BufReader, BufWriter},
  net::{UnixListener, UnixStream},
};

#[tokio::main]
async fn main() -> Result<()> {
  // Set up listener against file descriptor(s)
  let mut listenfd = listenfd::ListenFd::from_env();
  let listener = UnixListener::from_std(
    listenfd
      .take_unix_listener(0)?
      .expect("Couldn't create listener"),
  )?;

  // Handle incoming connections
  while let Ok((mut sock, _)) = listener.accept().await {
    tokio::spawn(async move {
      handle_connection(&mut sock).await?;
      Ok::<_, Error>(())
    });
  }

  Ok(())
}

async fn handle_connection(socket: &mut UnixStream) -> Result<()> {
  // Set up buffers
  let mut initial_buf: Vec<u8> = vec![];
  let mut writer = BufWriter::new(stdout());
  let mut reader = BufReader::new(socket);

  // Read the until header delimiter
  let n = reader.read_until(b'\0', &mut initial_buf).await?;

  // Parse the header fields
  let [rpc_service, remote_domain] = str::from_utf8(&initial_buf[..n - 1])?
    .split(" ")
    .collect::<Vec<_>>()[..2]
  else {
    bail!("Couldn't read service descriptor header");
  };

  // Logging
  println!(
    "qrexec call: rpc_service={:?} remote_domain={:?}",
    &rpc_service, &remote_domain
  );

  // TODO: Spawn a new daemon, get its socket
  // TODO: Proxy to that socket instead

  // Connect reader/writer socket buffers
  copy(&mut reader, &mut writer).await?;

  Ok(())
}
