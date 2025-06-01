use crossbeam::channel;

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = tokio::sync::mpsc::channel(20);
    let (tx2, mut rx2) = tokio::sync::mpsc::channel(20);
    tokio::task::spawn_blocking(move || {
        println!("Blocking task started!, thread id: {:?}", std::thread::current().id());
        loop {
            let mut closed_channels = 0;
            let mut empty_channels = 0;
            match rx1.try_recv() {
                Ok(msg) => println!("blocking thread: received 1: '{}'", msg),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                    // println!("blocking thread - recv 1 err: empty");
                    empty_channels += 1;
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                    // println!("blocking thread - recv 1 err: disconnected");
                    closed_channels += 1;
                }
            }
            match rx2.try_recv() {
                Ok(msg) => println!("blocking thread: received 2: '{}'", msg),
                Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {
                    // println!("blocking thread - recv 2 err: empty");
                    empty_channels += 1;
                }
                Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                    // println!("blocking thread - recv 2 err: disconnected");
                    closed_channels += 1;
                }
            }
            if closed_channels == 2 {
                break;
            }
            if empty_channels == 2 || (empty_channels == 1 && closed_channels == 1) {
                // println!("blocking thread - sleep 5ms");
                std::thread::sleep(std::time::Duration::from_millis(5));
            }
        }

        // crossbeam::channel::select! {
        //         recv(rx1) -> _ => println!("kdfjld")
        // }

        println!("blocking thread finished");
    });

    println!("Non-blocking task started!, thred id: {:?}", std::thread::current().id());
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let _ = tx1.send("Hello from non-blocking task").await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let _ = tx2.send("Hello from non-blocking task tx2").await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    drop(tx1);
    println!("dropped tx1");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    drop(tx2);
    println!("dropped tx2");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Non-blocking task completed!");
}
