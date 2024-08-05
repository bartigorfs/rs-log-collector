use tokio::signal;
use tokio::sync::watch::Sender;

pub async fn get_graceful_signal(shutdown_tx: Sender<()>) {
    let shutdown_signal = async {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};

            let mut sigint =
                signal(SignalKind::interrupt()).expect("Failed to create SIGINT signal handler");
            let mut sigterm =
                signal(SignalKind::terminate()).expect("Failed to create SIGTERM signal handler");

            tokio::select! {
                _ = sigint.recv() => {
                    println!("Received SIGINT (Ctrl+C)");
                }
                _ = sigterm.recv() => {
                    println!("Received SIGTERM (termination signal)");
                }
            }
        }

        #[cfg(windows)]
        {
            let ctrl_c = signal::ctrl_c();

            ctrl_c.await.expect("Failed to listen for Ctrl+C");
            println!("Received Ctrl+C");
        }

        let _ = shutdown_tx.send(());
    };

    shutdown_signal.await;
}