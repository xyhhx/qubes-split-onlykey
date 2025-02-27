use std::env;
extern crate anyhow;
extern crate tokio;
extern crate zbus;
use anyhow::Result;
use server_lib::Systemd1ManagerProxy;
use tokio::io::{AsyncReadExt, stdin};
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
  let connection = Connection::session().await?;
  dbg!(&connection);
  dbg!(env::vars());

  let mut input = String::new();
  stdin().read_to_string(&mut input).await?;
  dbg!(&input);
  let unit = format!(
    "daemon@{}.socket",
    &input.trim_matches(|c| !char::is_alphanumeric(c) && !char::is_ascii_punctuation(&c))
  );

  let systemd_proxy = Systemd1ManagerProxy::new(&connection).await?;
  let name = &unit;
  let mode = "replace";
  println!("calling start_unit with args:");
  dbg!((name, mode));

  let path = systemd_proxy.start_unit(name, mode).await?;
  println!("started the daemon at job path {:?}", path);

  Ok(())
}
