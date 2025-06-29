use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

/// A flag to indicate whether the meta update is complete.
/// Arc allows safe sharing across async tasks.
/// Mutex ensures that only one thread can access the boolean at a time.
pub type UpdateFlag = Arc<Mutex<bool>>;

/// Simulates updating meta data.
///
/// This function will lock the flag, print a message, wait for a bit to simulate work,
/// and then set the flag to `true` to indicate completion.
pub async fn update_meta(flag: UpdateFlag) {
    // Lock the mutex in a separate scope to ensure the lock is released
    // before any .await calls.
    {
        let meta_updated = flag.lock().unwrap();
        if *meta_updated {
            println!("Meta is already updated.");
            return;
        }
    }

    println!("Starting meta update...");
    // Simulate network/file IO
    sleep(Duration::from_secs(2)).await;

    // Re-acquire the lock to modify the flag.
    let mut meta_updated = flag.lock().unwrap();
    *meta_updated = true;
    println!("Meta update finished.");
}

/// Simulates updating holiday data.
///
/// This function checks the flag first. If the meta update is not complete,
/// it will exit early. Otherwise, it proceeds with its logic.
pub async fn update_holiday(flag: UpdateFlag) {
    // Lock the mutex to check the flag's value.
    // The lock is released at the end of the scope.
    {
        let meta_updated = flag.lock().unwrap();
        if !*meta_updated {
            println!("Holiday update cannot start: meta update is not complete. Exiting.");
            return;
        }
    }

    println!("Starting holiday update...");
    // Simulate network/file IO
    sleep(Duration::from_secs(1)).await;
    println!("Holiday update finished.");

}



