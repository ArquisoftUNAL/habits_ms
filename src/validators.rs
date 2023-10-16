use crate::utils::MAX_DAYS_OFFSET;
use bigdecimal::BigDecimal;
use validator::ValidationError;

pub fn validate_bigdecimal(value: &BigDecimal) -> Result<(), ValidationError> {
    if value.sign() != bigdecimal::num_bigint::Sign::Plus
        && value.sign() != bigdecimal::num_bigint::Sign::NoSign
    {
        return Err(ValidationError::new("Negative / zero value"));
    }

    Ok(())
}

pub fn validate_habdata_collected_at(value: &chrono::NaiveDate) -> Result<(), ValidationError> {
    let now = chrono::Utc::now().naive_utc().date();

    if value > &now {
        return Err(ValidationError::new(
            "Habit data completion date is in the future",
        ));
    }

    let grace_period_end = now + chrono::Duration::days(MAX_DAYS_OFFSET);

    if value > &grace_period_end {
        return Err(ValidationError::new(
            "Habit data completion date is outside the grace period",
        ));
    }

    Ok(())
}
