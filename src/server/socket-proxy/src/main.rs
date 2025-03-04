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
use server_lib::Systemd1ManagerProxy;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, Interest, copy, stdout};
use tokio::net::{UnixListener, UnixStream};
use zbus::{Connection, MatchRule};

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
  let mut writer = BufWriter::new(stdout());
  let rpc_call = get_rpc_info_from_socket_input(incoming_rpc_socket).await?;
  // TODO: validate input

  // TODO: Spawn a new daemon, get its socket
  let connection = Connection::session().await?;
  let systemd_proxy = Systemd1ManagerProxy::new(&connection).await?;
  handle_factory(rpc_call.remote_domain.as_str());

  // start a daemon factory: pass the remote domain name to the factory.socket
  // wait for it to finish, then find the spawned daemon socket/service
  // connect to that socket, forwarding to the input socket here

  // TODO: Proxy to that socket instead

  // Connect reader/writer socket buffers
  copy(&mut reader, &mut writer).await?;

  Ok(())
}

async fn get_rpc_info_from_socket_input(
  incoming_rpc_socket: &mut UnixStream,
) -> Result<QubesRPCCall> {
  // Set up buffers
  let mut buffer: Vec<u8> = vec![];
  let mut reader = BufReader::new(incoming_rpc_socket);

  // Read the until header delimiter
  let n = reader.read_until(b'\0', &mut buffer).await?;

  // Parse the header fields
  let [rpc_service, remote_domain] = str::from_utf8(&buffer[..n - 1])?
    .split(" ")
    .collect::<Vec<_>>()[..2]
  else {
    bail!("Couldn't read socket input into buffer")
  };

  Ok(QubesRPCCall {
    remote_domain: String::from(remote_domain),
    rpc_service: String::from(rpc_service),
  })
}

async fn get_factory_socket_addr() -> Result<SocketAddr> {
  let runtime_path = env::var("RUNTIME_DIRECTORY")?;
  let path = Path::new(&runtime_path).with_file_name("daemon-factory.sock");
  let sock_addr = SocketAddr::from_pathname(path)?;

  Ok(sock_addr)
}

async fn make_match_rule() -> Result<MatchRule<'static>> {
  Ok(
    MatchRule::builder()
      .msg_type(zbus::message::Type::Signal)
      .sender("org.freedesktop.systemd1")?
      .interface("org.freedesktop.systemd1.Manager")?
      .member("JobRemoved")?
      .build(),
  )
}

async fn handle_factory(remote_domain: &str) -> Result<&str> {
  let socket_addr = get_factory_socket_addr().await?;
  let std_stream = StdStream::connect_addr(&socket_addr)?;
  let stream = UnixStream::from_std(std_stream)?;

  // TODO: write the remote_domain to the socket
  // TODO: wait for output from socket, read value (the job path of the new daemon service)
  // TODO: return the job path
}

struct QubesRPCCall {
  remote_domain: String,
  rpc_service: String,
}
