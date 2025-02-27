use std::env;

use anyhow::Result;
use tokio::io::{stdin, AsyncReadExt};
use zbus::zvariant::OwnedObjectPath;
use zbus::{proxy, Connection};

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

#[proxy(
  interface = "org.freedesktop.systemd1.Manager",
  default_service = "org.freedesktop.systemd1",
  default_path = "/org/freedesktop/systemd1"
)]
trait Systemd1Manager {
  fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
}
