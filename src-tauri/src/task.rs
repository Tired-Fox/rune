use std::sync::Arc;

use tauri::{Manager, ResourceId, ResourceTable, Runtime, Webview};
use tokio::sync::oneshot::Sender;

pub(crate) struct AbortSender(pub(crate) Sender<()>);
impl tauri::Resource for AbortSender {}
impl AbortSender {
  fn abort(self) {
    let _ = self.0.send(());
  }
}

pub(crate) trait Aborter {
  fn add_task_aborter(&mut self, sender: AbortSender) -> ResourceId;
}

impl Aborter for ResourceTable {
  fn add_task_aborter(&mut self, sender: AbortSender) -> ResourceId {
    self.add(sender)
  }
}

#[tauri::command]
pub fn cancel_task<R: Runtime>(
  webview: Webview<R>,
  rid: ResourceId,
) -> Result<(), tauri::Error> {
  let mut rt = webview.resources_table();
  let abort_tx = rt.take::<AbortSender>(rid)?;
  if let Some(abort_tx) = Arc::into_inner(abort_tx) {
    abort_tx.abort();
  }
  Ok(())
}
