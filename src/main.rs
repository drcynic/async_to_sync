#[tokio::main]
async fn main() {
    // variant_try_recv().await;
    variant_crossbeam_select().await;
}

#[allow(dead_code)]
async fn variant_crossbeam_select() {
    let (tx1, rx1) = crossbeam::channel::unbounded::<&str>();
    let (tx2, rx2) = crossbeam::channel::unbounded::<&str>();
    tokio::task::spawn_blocking(move || {
        println!("Blocking task started!, thread id: {:?}", std::thread::current().id());
        let mut rx1_closed = false;
        let mut rx2_closed = false;
        loop {
            // println!("inside the loop");
            crossbeam::channel::select! {
                    recv(rx1) -> msg => {
                        match msg {
                            Ok(msg) => {
                                println!("rx1: {}", msg);
                                std::thread::sleep(std::time::Duration::from_secs(3));
                                println!("rx1 task done after 3s");
                            }
                            Err(e) => {
                                // println!("rx1 error: {}", e);
                                rx1_closed = true;
                            }
                        }
                    },
                    recv(rx2) -> msg => {
                        match msg {
                            Ok(msg) => {
                                println!("rx2: {}", msg);
                            }
                            Err(e) => {
                                // println!("rx2 error: {}", e);
                                rx2_closed = true;
                            }
                        }
                    },
                    default => std::thread::sleep(std::time::Duration::from_millis(1))
            }
            if rx1_closed && rx2_closed {
                break;
            }
        }

        println!("blocking thread finished");
    });

    println!("Non-blocking task started!, thred id: {:?}", std::thread::current().id());
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Send to tx1");
    let _ = tx1.send("Hello from non-blocking task");
    println!("Send to tx1 finished");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Send to tx2");
    let _ = tx2.send("Hello from non-blocking task tx2");
    println!("Send to tx2 finished");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("dropping tx1");
    drop(tx1);
    println!("dropped tx1");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("dropping tx2");
    drop(tx2);
    println!("dropped tx2");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("Non-blocking task completed!");
}

#[allow(dead_code)]
async fn variant_try_recv() {
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
