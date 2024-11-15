use std::fmt::Display;
use std::process::Output;

use tokio::process::{Child, Command};

use crate::{debug, error, info, Result};

pub async fn dispatch(name: impl AsRef<str> + Display + Send + 'static, commander: Vec<String>) -> Result<()> {
    let (tx_stop, rx_stop) = tokio::sync::oneshot::channel();

    let cmd = Command::new(&*crate::DEFAULT_EXE_PATH);

    spawn_child_proc(tx_stop, cmd, commander).await;

    let child = rx_stop.await.unwrap();

    let output = tokio::task::spawn(async move {
        let after = child.wait_with_output().await.unwrap();
        CheckStatus::from(after).check_status(name);
    });

    tokio::pin!(output);
    let _ = (&mut output).await;

    // CheckStatus::from(after).check_status(name);

    Ok(())
}

async fn spawn_child_proc(tx_stop: tokio::sync::oneshot::Sender<Child>, mut cmd: Command, commander: Vec<String>) {
    debug!("->> {:<12} - {:?}", "DISPATCH:: Commander", &commander);
    info!("{:<12}", "DISPATCH:: Child proc");

    let cmd_for_print = commander.clone();
    debug!("->> {:<12} - {:?}", "DISPATCH:: Commander", &cmd_for_print);

    // match tokio::spawn(async move {
    tokio::task::spawn_blocking(move || {
        info!("->> {:<12} - {}", "DISPATCH:: Spawned", "Child Proc");
        let child = cmd
            .args(commander.clone())
            .spawn()
            .map_err(crate::error::Error::from)
            .expect("Failed to spawn");

        tx_stop.send(child).unwrap();
    })
    .await
    .unwrap();

    // .unwrap()
    // .await;
    // .expect("Failed to wait on child");

    // info!("Before match output");

    // let output = match output {
    //     Ok(o) => {
    //         info!("->> {:<12} - {}", "DISPATCH:: OK", "Task dispatched!");
    //         o
    //     }
    //     Err(e) => return error!("->> {:<12} - Tried to run {cmd_for_print:?} - {e}", "DISPATCH:: ERR"),
    // };

    // tx_stop.send(output).unwrap();
    // })
    // .await
}

enum CheckStatus {
    Success(Output),
    Fail(Output),
}

impl From<Output> for CheckStatus {
    #[inline]
    fn from(output: Output) -> Self {
        match output.status.success() {
            true => CheckStatus::Success(output),
            false => CheckStatus::Fail(output),
        }
    }
}

impl CheckStatus {
    #[inline]
    fn check_status(&self, name: impl AsRef<str> + Display) {
        match self {
            CheckStatus::Success(output) => {
                info!(
                    "->> {:<12} - {}: {name} - with output: {}",
                    "DISPATCH:: OK", "Job is now running on", output.status
                )
            }
            CheckStatus::Fail(output) => {
                error!("->> {:<12} - {}: {name} - {output:?}", "DISPATCH:: ERR", "Job has failed to start running on")
            }
        };
    }
}
