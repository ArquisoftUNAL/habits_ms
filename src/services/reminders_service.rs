use ::reqwest::Client;
use chrono::NaiveDate;

use graphql_client::{reqwest::post_graphql, GraphQLQuery};

use crate::models::database::Habit;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/enqueue_notification.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct NotifyReminder;

use notify_reminder::{NotificationInsert, Variables as NotifyReminderVariables};

pub async fn enqueue_reminders_service(habits: Vec<Habit>) -> Result<(), reqwest::Error> {
    println!("Testing notification enqueueing x1.5");
    // Comunicate with gateway to enqueue reminders of habits
    let gateway_url = std::env::var("GATEWAY_URL").unwrap_or("http://localhost:4000".to_string());
    println!("Testing notification enqueueing x1.6");
    let client = Client::new();
    println!("Testing notification enqueueing x1.7");

    let mut notifications: Vec<NotificationInsert> = Vec::new();

    let current_date: NaiveDate = chrono::Local::now().naive_local().date();

    println!("Testing notification enqueueing x1.7");
    for habit in habits {
        notifications.push(NotificationInsert {
            title: format!("Reminder for habit {}", habit.hab_name),
            body: format!("Your habit just restarted its period! Remember to do it today!"),
            init_date: current_date.to_string(),
            user_id: habit.usr_id,
            should_email: false,
        });
    }

    println!("Testing notification enqueueing x1.8");
    let variables = NotifyReminderVariables {
        input: notifications,
    };

    println!("Testing notification enqueueing x2");

    let response_body = post_graphql::<NotifyReminder, _>(&client, gateway_url, variables).await;

    if response_body.is_err() {
        println!(
            "Error sending reminder notifications: {:?}",
            response_body.err()
        );
        return Ok(());
    }

    let response_body = response_body.unwrap();

    if let Some(errors) = response_body.errors {
        for error in errors {
            println!("Error sending reminder notifications: {:?}", error);
        }

        return Ok(());
    }

    let response_data = response_body.data;

    if response_data.is_none() {
        println!("Error sending reminder notifications: No data returned");
        return Ok(());
    }

    println!("{:#?}", response_data);

    Ok(())
}
