use bigdecimal::BigDecimal;
use validator::ValidationError;

pub fn validate_bigdecimal(value: &BigDecimal) -> Result<(), ValidationError> {
    if value.sign() != bigdecimal::num_bigint::Sign::Plus {
        return Err(ValidationError::new("Negative / zero value"));
    }

    Ok(())
}
