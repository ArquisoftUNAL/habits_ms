use crate::db::{DBManager, PostgresPool};
use crate::models::api::{
    category_api_models::CategoryCreateSchema, data_api_models::HabitDataCreateSchema,
    habit_api_models::HabitCreateSchema,
};
use crate::models::database::HabFreqTypeEnum;
use bigdecimal::BigDecimal;
use fake::{faker, Fake, Faker};
use std::str::FromStr;
use uuid::Uuid;

const BASE_QUANTITY: i32 = 100;

pub fn seed_database(pool: PostgresPool) -> Result<(), String> {
    let manager = DBManager::new(pool);

    println!("Seeding users");
    // Randomize user ids
    let mut user_ids: Vec<String> = Vec::new();

    for _ in 0..5 {
        let user_id = Faker.fake::<String>().chars().take(24).collect::<String>();
        user_ids.push(user_id);
    }

    println!("Seeding categories");
    // Create (base / 10 ) categories
    let mut categories_ids: Vec<Uuid> = Vec::new();

    for _ in 0..10 {
        let category: CategoryCreateSchema = CategoryCreateSchema {
            name: Faker.fake::<String>().chars().take(45).collect::<String>(),
        };

        let category_id = manager.add_category(category);

        if category_id.is_err() {
            return Err(format!(
                "[Seeder] Error creating category: {:?}",
                category_id.err().unwrap()
            ));
        }
        categories_ids.push(category_id.unwrap());
    }

    println!("Seeding habits");

    // Create (base) habits
    let mut habits_ids: Vec<Uuid> = Vec::new();

    for i in 0..BASE_QUANTITY {
        let user_id = user_ids[(i % user_ids.len() as i32) as usize].clone();
        let habit = HabitCreateSchema {
            name: Faker.fake::<String>().chars().take(45).collect::<String>(),
            description: Faker.fake::<String>().chars().take(45).collect::<String>(),
            is_favorite: Faker.fake(),
            units: Faker.fake::<String>().chars().take(5).collect::<String>(),
            is_yn: Faker.fake(),
            goal: BigDecimal::from_str(
                faker::number::en::NumberWithFormat("##.##")
                    .fake::<String>()
                    .as_str(),
            )
            .unwrap(),
            frequency_type: match (i % 3) as i32 {
                0 => HabFreqTypeEnum::daily {},
                1 => HabFreqTypeEnum::weekly {},
                2 => HabFreqTypeEnum::monthly {},
                _ => HabFreqTypeEnum::daily {},
            },
            color: Faker.fake::<String>().chars().take(6).collect::<String>(),
            category: categories_ids[(i % categories_ids.len() as i32) as usize].clone(),
            location: Some(Faker.fake::<String>()),
        };

        let habit_id = manager.add_habit(user_id, habit);

        if habit_id.is_err() {
            return Err(format!(
                "[Seeder] Error creating habit: {:?}",
                habit_id.err().unwrap()
            ));
        }

        habits_ids.push(habit_id.unwrap());
    }

    println!("Seeding habits data");

    // Create (base) data
    for i in 0..BASE_QUANTITY * 100 {
        let data = HabitDataCreateSchema {
            amount: BigDecimal::from_str(
                faker::number::en::NumberWithFormat("##.##")
                    .fake::<String>()
                    .as_str(),
            )
            .unwrap(),
            collected_at: Some(
                chrono::Local::now().date_naive()
                    + chrono::Duration::days((i - BASE_QUANTITY * 100 - 10) as i64),
            ),
            habit_id: habits_ids[(i % habits_ids.len() as i32) as usize].clone(),
        };

        let data_id = manager.add_habit_data(data);

        if data_id.is_err() {
            return Err(format!(
                "[Seeder] Error creating data: {:?}",
                data_id.err().unwrap()
            ));
        }
    }

    Ok(())
}
