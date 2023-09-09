use serde_derive::Serialize;
use std::convert::Infallible;
use thiserror::Error;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};
use warp::{http::StatusCode, Rejection, Reply};

#[derive(Error, Debug)]
pub enum Error {
    #[error("JSON path error: {0}")]
    JSONPathError(String),

    #[error("Data validation error: {0}")]
    ValidationError(ValidationErrors),

    #[error("Database connection error: {0}")]
    DBConnectionError(#[from] diesel::r2d2::PoolError),

    #[error("Query error: {0}")]
    QueryError(#[from] diesel::result::Error),
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
    println!("err: {:?}", err);
    let (code, message, errors) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found".to_string(), None)
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::JSONPathError(_) => (StatusCode::BAD_REQUEST, e.to_string(), None),
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
                                .map(|fe| format!("{}: {:?}", fe.code, fe.params))
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
                    "field errors".to_string(),
                    Some(errors),
                )
            }
            Error::DBConnectionError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database connection error: {}", error),
                None,
            ),
            Error::QueryError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Query error: {}", error),
                None,
            ),
        }
    // } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
    //     (
    //         StatusCode::METHOD_NOT_ALLOWED,
    //         "Method Not Allowed".to_string(),
    //         None,
    //     )
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
