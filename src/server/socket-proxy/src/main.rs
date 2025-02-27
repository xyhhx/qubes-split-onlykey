extern crate anyhow;
extern crate tokio;

use core::str;
use std::env;
use std::os::unix::net::{SocketAddr, UnixStream as StdStream};
use std::path::Path;

use anyhow::{bail, Error, Result};
use server_lib::Systemd1ManagerProxy;
use tokio::io::{copy, stdout, AsyncBufReadExt, BufReader, BufWriter};
use tokio::net::{UnixListener, UnixStream};
use zbus::Connection;

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

async fn handle_connection(incoming_rpc_socket: &mut UnixStream) -> Result<()> {
  // Set up buffers
  let mut initial_buf: Vec<u8> = vec![];
  let mut writer = BufWriter::new(stdout());
  let mut reader = BufReader::new(incoming_rpc_socket);

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

  // TODO: validate input

  // TODO: Spawn a new daemon, get its socket

  let factory_socket_addr = get_factory_socket_addr().await?;

  let std_stream = StdStream::connect_addr(&factory_socket_addr)?;
  let factory_socket = UnixStream::from_std(std_stream)?;

  factory_socket.writable().await?;

  factory_socket
    .try_write(remote_domain.as_bytes())
    .unwrap_err();

  let connection = Connection::session().await?;
  let systemd_proxy = Systemd1ManagerProxy::new(&connection).await?;

  // systemd_proxy.start_unit(, mode)

  // start a daemon factory: pass the remote domain name to the factory.socket
  // wait for it to finish, then find the spawned daemon socket/service
  // connect to that socket, forwarding to the input socket here

  // TODO: Proxy to that socket instead

  // Connect reader/writer socket buffers
  copy(&mut reader, &mut writer).await?;

  Ok(())
}

async fn get_factory_socket_addr() -> Result<SocketAddr> {
  let runtime_path = env::var("RUNTIME_DIRECTORY")?;
  let path = Path::new(&runtime_path).with_file_name("daemon-factory.sock");

  let sock_addr = SocketAddr::from_pathname(path)?;

  Ok(sock_addr)
}
