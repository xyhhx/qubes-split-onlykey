extern crate zbus;
use zbus::{proxy, zvariant::OwnedObjectPath};

#[proxy(
  interface = "org.freedesktop.systemd1.Manager",
  default_service = "org.freedesktop.systemd1",
  default_path = "/org/freedesktop/systemd1"
)]
pub trait Systemd1Manager {
  fn start_unit(&self, name: &str, mode: &str) -> zbus::Result<OwnedObjectPath>;
}
