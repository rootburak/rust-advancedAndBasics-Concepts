

use tokio::time::{sleep, Duration};

use tokio::{self, join, select};
use tokio::sync::mpsc;

async fn veri_al() -> &'static str {
    "burak"
}

async fn veri_al2() -> &'static str {
    "emre"
}

fn veri_al3() -> i32 {
    let mut a = 0;
    for i in 1..10000 {
        a = i + 1;
    }
    a
}

#[tokio::main]
async fn main() {
    let veri: &'static str = veri_al().await;
    let (ilk, ikinci) = join!(veri_al(), veri_al2());
    println!("{}", veri);
    println!("join ile {} {}", ilk, ikinci);

    //heavy tasks
    let task_for_heavy_jobs = tokio::task::spawn_blocking(|| {
        let result = veri_al3(); // Sonucu al
        println!("finished with result: {}", result);
    }).await.unwrap();


    //channels
    let (sender,mut recaiver) = mpsc::channel(100);


    tokio::spawn(async move{
       sender.send("ben burak").await.unwrap(); 
    });

    if let Some(message) = recaiver.recv().await{
        println!("new message {}",message)
    };


    // cancel thread

    let task_cancel =tokio::spawn(async {
        select! {
            _ = tokio::signal::ctrl_c()=>{
                println!("iptal sinyali")
            }
            _ = async {sleep(Duration::from_secs(2)).await;} => {}
         }
         
    });

    task_cancel.await.unwrap();

    let async_block = async{
        println!("async block is running")
    };

    let async_clouser = |name:String| async move {
        println!("async clouser name {}",name)
    };
    async_clouser(String::from("burak")).await;
    async_block.await;

}