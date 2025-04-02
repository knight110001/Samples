use std::io::Error;

use tokio::time::{sleep, Duration};
use signal_hook::consts::signal::*;
use signal_hook_tokio::Signals;

use futures::stream::StreamExt;
use std::sync::atomic::{AtomicU8, Ordering};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref EXIT_FLAG: AtomicU8 = AtomicU8::new(0);
}

async fn handle_signals(mut signals: Signals) {
    while let Some(signal) = signals.next().await {
        match signal {
            SIGHUP => {
                // Reload configuration
                // Reopen the log file
            }
            SIGTERM | SIGINT | SIGQUIT => {
                // Shutdown the system;
                EXIT_FLAG.store(1, Ordering::Relaxed);
                println!("SIG {} is comming !!!", signal);
            },
            _ => unreachable!(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let signals = Signals::new(&[
        SIGHUP,
        SIGTERM,
        SIGINT,
        SIGQUIT,
    ])?;
    let handle = signals.handle();

    let signals_task = tokio::spawn(handle_signals(signals));

    // Execute your main program logic
    println!("hello world!");

    while EXIT_FLAG.load(Ordering::Acquire) == 0 {
        println!("working...");
        sleep(Duration::from_millis(1000)).await;
    }

    // Terminate the signal stream.
    handle.close();
    signals_task.await?;

    println!("bye-bye!");
    Ok(())
}

