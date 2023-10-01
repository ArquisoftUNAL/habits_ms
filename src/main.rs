// https://levelup.gitconnected.com/building-an-api-using-warp-and-tokio-26a52173860a

// Disable camelcase warnings, due to postgres enum types being allways in lowercase
#![allow(non_camel_case_types)]

mod db;
mod error;
mod handlers;
mod jobs;
mod models;
mod queries;
mod routes;
mod schema;
mod seeders;
mod services;
mod tests;
mod utils;
mod validators;

// Read parameters passed from cargo to see if we are just seeding the database
use std::env;

use tokio_cron_scheduler::{Job, JobScheduler};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
//#[tokio::main(flavor = "current_thread")]
async fn main() {
    let pool = db::create_pool();

    if pool.is_err() {
        println!("[MAIN] Error creating pool: {:?}", pool.err());

        // Throw error and exit (in order to force container to restart)
        panic!();
    }

    let pool = pool.unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args[1] == "seed" {
            println!("Seeding database");
            let result = seeders::seed_database(pool.clone());

            if result.is_err() {
                println!("[SEEDING] Error seeding database: {:?}", result.err());
            }
            return;
        }
    }

    // Initialize jobs
    let sched = JobScheduler::new().await;

    if !sched.is_err() {
        let sched = sched.unwrap();
        let job = Job::new_async("* 1 * * * * *", move |_, _| {
            let jobs_pool = db::create_pool();

            if jobs_pool.is_err() {
                println!("[JOBS] Error creating pool: {:?}", jobs_pool.err());
                return Box::pin(async move {});
            }

            let jobs_pool = jobs_pool.unwrap();

            Box::pin(async move {
                jobs::check_reminders_update(jobs_pool.clone()).await;
            })
        });

        if !job.is_err() {
            let job = job.unwrap();
            let result = sched.add(job).await;

            if !result.is_err() {
                // Start the scheduler
                tokio::spawn(async move {
                    let result = sched.start().await;
                    if !result.is_err() {
                        println!("Scheduler exited with result: {:?}", result);
                    } else {
                        println!("Scheduler exited with error: {:?}", result.err());
                    }
                });
            } else {
                println!("Error adding job: {:?}", result.err());
            }
        } else {
            println!("Error creating job: {:?}", job.err());
        }
    } else {
        println!("Error creating scheduler: {:?}", sched.err());
    }

    let routes = routes::get_routes(pool);
    println!("Preparing server to listen on port 3030");

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}
