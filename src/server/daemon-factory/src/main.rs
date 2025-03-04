use std::env;
extern crate anyhow;
extern crate sd_notify;
extern crate tokio;
extern crate zbus;
use anyhow::Result;
use sd_notify::NotifyState;
use server_lib::Systemd1ManagerProxy;
use tokio::io::{AsyncReadExt, stdin};
use zbus::Connection;

#[tokio::main]
async fn main() -> Result<()> {
  let connection = Connection::session().await?;
  dbg!(&connection);
  dbg!(env::vars());
  // notify systemd and we're ready to start
  sd_notify::notify(false, &[NotifyState::Ready])?;

  // read stdin to get desired socket/service name
  let mut input = String::new();
  stdin().read_to_string(&mut input).await?;
  dbg!(&input);
  // sanitization
  let unit = format!(
    // TODO: This should be configurable
    "daemon@{}.socket",
    &input.trim_matches(|c| !char::is_alphanumeric(c) && !char::is_ascii_punctuation(&c))
  );

  // start a new daemon instance with the requested name
  let systemd_proxy = Systemd1ManagerProxy::new(&connection).await?;
  let name = &unit;
  let mode = "replace";
  println!("calling start_unit with args:");
  dbg!((name, mode));
  let path = systemd_proxy.start_unit(name, mode).await?;
  println!("started the daemon at job path {:?}", path);

  // we're done
  sd_notify::notify(false, &[NotifyState::Stopping])?;

  Ok(())
}
