use tokio::stream::StreamExt;
use tokio::sync::mpsc::unbounded_channel;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = unbounded_channel();

    task::spawn(async move {
        while let Some(msg) = rx.next().await {
            println!("recv: {}", msg);
        }

        println!("tx dropped");
    });

    let _ = tx.send("hello");
    let _ = tx.send("darkness");
    let _ = tx.clone().send("my");
    let _ = tx.clone().send("old");
    let _ = tx.send("friend");
}
