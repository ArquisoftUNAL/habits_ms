use crate::{
    db::{DBManager, PostgresPool},
    services::reminders_service::enqueue_reminders_service,
};

pub async fn check_reminders_update(pool: PostgresPool) {
    let manager = DBManager::new(pool);

    let updated_habits = manager.get_update_pending_habits();

    if updated_habits.is_err() {
        println!(
            "Error getting updated habits: {:?}",
            updated_habits.err().unwrap()
        );
        return;
    }

    let updated_habits = updated_habits.unwrap();

    if updated_habits.is_empty() {
        println!("No updated habits");
        return;
    }

    let result = enqueue_reminders_service(updated_habits).await;

    if result.is_err() {
        println!("Error enqueuing reminders: {:?}", result.err().unwrap());
        return;
    }

    println!("Enqueued reminders");
}
