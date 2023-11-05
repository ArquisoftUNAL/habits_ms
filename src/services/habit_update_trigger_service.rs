use ::reqwest::blocking::Client;

use graphql_client::{reqwest::post_graphql_blocking, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schema.graphql",
    query_path = "src/graphql/enqueue_habit_update.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct NotifyHabUpdate;

use notify_hab_update::Variables as NotifyHabUpdateVariables;

pub fn notify_habit_update_service(habit_id: uuid::Uuid) -> Result<(), reqwest::Error> {
    // Comunicate with gateway to enqueue reminders of habits
    let gateway_url = std::env::var("GATEWAY_URL").unwrap_or("http://localhost:4000".to_string());

    let client = Client::new();

    let variables = NotifyHabUpdateVariables {
        id: habit_id.to_string(),
    };

    let response_body =
        post_graphql_blocking::<NotifyHabUpdate, _>(&client, gateway_url, variables);

    if response_body.is_err() {
        println!(
            "Error sending habit update notification: {:?}",
            response_body.err()
        );
        return Ok(());
    }

    let response_body = response_body.unwrap();

    if let Some(errors) = response_body.errors {
        for error in errors {
            println!("Error sending habit update notification: {:?}", error);
        }

        return Ok(());
    }

    let response_data = response_body.data;

    if response_data.is_none() {
        println!("Error sending habit update notification: No data returned");
        return Ok(());
    }

    Ok(())
}
