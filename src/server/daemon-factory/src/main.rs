use anyhow::Result;
use daemon_factory::{log::install_systemd_journal_logger, zbus_proxy::watch_systemd_jobs};
use log::LevelFilter;

#[tokio::main]
async fn main() -> Result<()> {
  install_systemd_journal_logger(Some(LevelFilter::Trace))?;
  watch_systemd_jobs().await?;

  // TODO: get job, wait for it to end
  // TODO: remove it
  // TODO: start new job using dbus
  // ???
  // profit

  Ok(())
}
