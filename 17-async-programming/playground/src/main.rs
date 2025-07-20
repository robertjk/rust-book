use std::{
    pin::{Pin, pin},
    thread,
    time::{Duration, Instant},
};
use trpl::{self, Either, ReceiverStream, Stream, StreamExt};

fn main() {
    trpl::run(async {
        timeout_test().await;
        println!("-------------------");
        race().await;
        println!("-------------------");
        messages().await;
        println!("-------------------");
        join().await;
        println!("-------------------");
        benchmark_sleep_vs_yield().await;
        println!("-------------------");
        streams_iterator().await;
        println!("-------------------");
        streams_main().await;
    });
}

async fn timeout_test() {
    let slow = async {
        trpl::sleep(Duration::from_secs(5)).await;
        "I finished!"
    };

    match timeout(slow, Duration::from_secs(7)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }
}

async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

async fn race() {
    let a = async {
        println!("'a' started");
        slow_operation("a", 30);
        trpl::yield_now().await;
        slow_operation("a", 10);
        trpl::yield_now().await;
        slow_operation("a", 20);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'a' finished");
    };

    let b = async {
        println!("'b' started");
        slow_operation("b", 75);
        trpl::yield_now().await;
        slow_operation("b", 10);
        trpl::yield_now().await;
        slow_operation("b", 15);
        trpl::yield_now().await;
        slow_operation("b", 350);
        trpl::sleep(Duration::from_millis(50)).await;
        println!("'b' finished");
    };

    trpl::race(a, b).await;
}

fn slow_operation(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

async fn messages() {
    let (tx1, mut rx) = trpl::channel();
    let tx2 = tx1.clone();

    let tx_fut1 = pin!(async move {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });

    let tx_fut2 = pin!(async move {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1500)).await;
        }
    });

    let rx_fut = pin!(async {
        while let Some(value) = rx.recv().await {
            println!("Got: {value}");
        }
    });

    let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx_fut1, tx_fut2, rx_fut];
    trpl::join_all(futures).await;
}

async fn join() {
    let fut1 = async {
        for i in 1..10 {
            println!("Hi number {i} from the first task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    let fut2 = async {
        for i in 1..5 {
            println!("Hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    };

    trpl::join(fut1, fut2).await;
}

async fn benchmark_sleep_vs_yield() {
    let one_ns = Duration::from_nanos(1);
    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::sleep(one_ns).await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!("'sleep' version finished in {} seconds", time.as_secs_f32());

    let start = Instant::now();
    async {
        for _ in 1..1000 {
            trpl::yield_now().await;
        }
    }
    .await;
    let time = Instant::now() - start;
    println!("'yield' version finished in {} seconds", time.as_secs_f32());
}

async fn streams_iterator() {
    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let stream = trpl::stream_from_iter(iter);

    let mut stream_filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

    while let Some(value) = stream_filtered.next().await {
        println!("The value was: {value}");
    }
}

async fn streams_main() {
    let messages = get_messages().timeout(Duration::from_millis(200));
    let intervals = get_intervals()
        .map(|count| format!("Interval: {count}"))
        .throttle((Duration::from_millis(100)))
        .timeout(Duration::from_secs(10));
    let merged = messages.merge(intervals).take(20);
    let mut stream = pin!(merged);

    while let Some(result) = stream.next().await {
        match result {
            Ok(message) => println!("{message}"),
            Err(reason) => eprintln!("Problem: {reason:?}"),
        }
    }
}

fn get_messages() -> impl Stream<Item = String> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let messages = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"];
        for (index, message) in messages.into_iter().enumerate() {
            let time_to_sleep = if index % 2 == 0 { 100 } else { 300 };
            trpl::sleep(Duration::from_millis(time_to_sleep)).await;

            if let Err(send_error) = tx.send(format!("Message: '{message}'")) {
                eprintln!("Cannot send message '{message}': {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}

fn get_intervals() -> impl Stream<Item = u32> {
    let (tx, rx) = trpl::channel();

    trpl::spawn_task(async move {
        let mut count = 0;
        loop {
            trpl::sleep(Duration::from_millis(1)).await;
            count += 1;

            if let Err(send_error) = tx.send(count) {
                eprintln!("Could not send interval {count}: {send_error}");
                break;
            }
        }
    });

    ReceiverStream::new(rx)
}
