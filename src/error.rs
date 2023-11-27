use serde_derive::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use validator::{ValidationErrors, ValidationErrorsKind};
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Data validation error: {0}")]
    ValidationError(ValidationErrors),

    #[error("Database connection error: {0}")]
    DBConnectionError(#[from] diesel::r2d2::PoolError),

    #[error("Database connection error: {0}")]
    DBConnectionError2(#[from] diesel::ConnectionError),

    #[error("Query error: {0}")]
    QueryError(#[from] diesel::result::Error),

    #[error("Error: {0}")]
    DBError(#[from] diesel::r2d2::Error),

    #[error("Authentication error: {0}")]
    AuthorizationError(String),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

#[derive(Serialize)]
struct FieldError {
    field: String,
    field_errors: Vec<String>,
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    errors: Option<Vec<FieldError>>,
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    let (code, message, errors) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string(), None)
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::ValidationError(val_errs) => {
                let errors: Vec<FieldError> = val_errs
                    .errors()
                    .iter()
                    .map(|error_kind| FieldError {
                        field: error_kind.0.to_string(),
                        field_errors: match error_kind.1 {
                            ValidationErrorsKind::Struct(struct_err) => {
                                validation_errs_to_str_vec(struct_err)
                            }
                            ValidationErrorsKind::Field(field_errs) => field_errs
                                .iter()
                                .map(|fe| format!("Check {}", fe.code))
                                .collect(),
                            ValidationErrorsKind::List(vec_errs) => vec_errs
                                .iter()
                                .map(|ve| {
                                    format!(
                                        "{}: {:?}",
                                        ve.0,
                                        validation_errs_to_str_vec(ve.1).join(" | "),
                                    )
                                })
                                .collect(),
                        },
                    })
                    .collect();

                (
                    StatusCode::BAD_REQUEST,
                    "Validation error, check fields values".to_string(),
                    Some(errors),
                )
            }
            Error::DBConnectionError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", error),
                None,
            ),
            Error::DBConnectionError2(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", error),
                None,
            ),
            Error::QueryError(error) => match error {
                diesel::result::Error::NotFound => {
                    (StatusCode::NOT_FOUND, "Record not found".to_string(), None)
                }

                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Database error: {}", error),
                    None,
                ),
            },
            Error::AuthorizationError(error) => (
                StatusCode::UNAUTHORIZED,
                format!("Authorization error: {}", error),
                None,
            ),
            Error::BadRequest(error) => (
                StatusCode::BAD_REQUEST,
                format!("Bad request: {}", error),
                None,
            ),
            Error::DBError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database Error: {}", error),
                None,
            ),
        }
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid body: {}", e),
            None,
        )
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        // Have care with this errror, it will catch all method not allowed errors (including those from other routes, so all other
        // possible errors should be cached from here)
        (
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed".to_string(),
            None,
        )
    } else if let Some(_) = err.find::<warp::reject::InvalidQuery>() {
        (
            StatusCode::BAD_REQUEST,
            "Invalid Query String".to_string(),
            None,
        )
    } else {
        eprintln!("unhandled error: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {:?}", err),
            None,
        )
    };

    let json = warp::reply::json(&ErrorResponse {
        message: message.into(),
        errors,
    });

    Ok(warp::reply::with_status(json, code))
}

fn validation_errs_to_str_vec(ve: &ValidationErrors) -> Vec<String> {
    ve.field_errors()
        .iter()
        .map(|fe| {
            format!(
                "{}: errors: {}",
                fe.0,
                fe.1.iter()
                    .map(|ve| format!("{}: {:?}", ve.code, ve.params))
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        })
        .collect()
}
