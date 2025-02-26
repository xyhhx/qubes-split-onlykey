use anyhow::Result;
use std::env;
use tokio::io::{copy, stdin, BufReader, BufWriter};
use tokio::net::UnixStream;

// simply proxies stdin to whatever socket is available on $SSH_AUTH_SOCK

#[tokio::main]
async fn main() -> Result<()> {
  let ssh_auth_sock = env::var("SSH_AUTH_SOCK").expect("SSH_AUTH_SOCK must be defined");
  let mut stream = UnixStream::connect(ssh_auth_sock).await?;
  let mut reader = BufReader::new(stdin());
  let mut writer = BufWriter::new(&mut stream);

  copy(&mut reader, &mut writer).await?;

  Ok(())
}
