pub async fn simple_print() {
    println!("I am async function");
}

pub async fn huge_computation() {
    println!("Huge computation has started");
    for i in 0..100_000_000 {
    }
    println!("Huge computation has completed");
}

pub async fn simple_computation() {
    println!("Simple computation has started");
    println!("Simple computation has completed");
}

// mod async_await;
// use tokio;

// #[tokio::main]
// async fn main() {
//     let lazy_future = async_await::simple_print();
//     println!("The future has not been awaited for yet.");
//     lazy_future.await;

//     let mut handles = vec![];
//     println!("This code is not part of the async block.");
//     let aw1 = tokio::spawn(async move {
//         async_await::huge_computation().await;
//     });
//     handles.push(aw1);

//     let aw2 = tokio::spawn(async move {
//         async_await::simple_computation().await;
//     });
//     handles.push(aw2);

//     for handle in handles {
//         handle.await.unwrap();
//     }
//     println!("All tasks are now completed");

//     // tokio::select
//     let aw1 = tokio::spawn(async move {
//         async_await::huge_computation().await;
//     });

//     let aw2 = tokio::spawn(async move {
//         async_await::simple_computation().await;
//     });

//     tokio::select![
//         _ = aw1 => println!("The huge function is selected."),
//         _ = aw2 => println!("The simple function is selected.")
//     ];

//     // tokio::join
//     tokio::join![async_await::huge_computation(), async_await::simple_computation()];
// }
