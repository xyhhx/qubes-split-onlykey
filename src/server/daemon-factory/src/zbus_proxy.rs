use futures_util::stream::StreamExt;
use zbus::{proxy, zvariant::OwnedObjectPath, Connection};

#[proxy(
  interface = "org.freedesktop.systemd1",
  default_service = "org.freedesktop.systemd1.Manager",
  default_path = "/org/freedesktop/systemd1"
)]
pub trait Systemd1Manager {
  #[zbus(signal)]
  fn job_removed(&self, id: u32, job: OwnedObjectPath, unit: String) -> zbus::Result<()>;

  fn start_unit(&self, unit: &str) -> zbus::Result<u32>;
}

pub async fn watch_systemd_jobs() -> zbus::Result<()> {
  let connection = Connection::system().await?;
  let systemd_proxy = Systemd1ManagerProxy::new(&connection).await?;
  let mut removed_jobs_stream = systemd_proxy.receive_job_removed().await?;

  while let Some(msg) = removed_jobs_stream.next().await {
    let args: JobRemovedArgs = msg.args().expect("Error parsing message");
    println!(
      "JobRemoved received: unit={} id={} path={}",
      args.unit, args.id, args.job
    );
  }

  Ok(())
}
