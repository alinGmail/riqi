use riqi::update_logic::{update_holiday, update_meta, UpdateFlag};
use std::sync::{Arc, Mutex};
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    // 1. Create the shared flag, initially set to `false`.
    let update_flag: UpdateFlag = Arc::new(Mutex::new(false));

    println!("--- First Attempt ---");
    println!("Attempting to update holidays before meta is complete.");

    // Spawn tasks to run concurrently
    let holiday_task_1 = tokio::spawn(update_holiday(Arc::clone(&update_flag)));
    
    // Wait for the first holiday task to complete
    holiday_task_1.await.unwrap();

    println!("\n--- Second Attempt ---");
    println!("Now, let's run the meta update first.");

    // Run the meta update
    let meta_task = tokio::spawn(update_meta(Arc::clone(&update_flag)));
    meta_task.await.unwrap();

    // Give it a moment to ensure the flag is set before the next check
    time::sleep(Duration::from_millis(100)).await;

    println!("\nAttempting to update holidays again after meta is complete.");
    
    // Run the holiday update again
    let holiday_task_2 = tokio::spawn(update_holiday(Arc::clone(&update_flag)));
    holiday_task_2.await.unwrap();
    
    println!("\nDemonstration finished.");
}
