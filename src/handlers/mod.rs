use super::data::{
    ActivitiesResponse, ActivityRequest, ActivityResponse, EditActivityRequest, ErrorListResponse,
};
use super::external;
use super::AppState;
use actix_web::{error, Error, HttpRequest, HttpResponse, Json, Path, Responder};
use failure::Fail;

#[derive(Fail, Debug)]
pub enum AnalyzerError {
    #[fail(display = "External Service Error")]
    ExternalServiceError,
    #[fail(display = "Activity Not Found Error")]
    ActivityNotFoundError,
}

impl error::ResponseError for AnalyzerError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AnalyzerError::ExternalServiceError => HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("external service error"),
            AnalyzerError::ActivityNotFoundError => HttpResponse::NotFound()
                .content_type("text/plain")
                .body("activity not found"),
        }
    }
}

pub fn health(_: &HttpRequest<AppState>) -> impl Responder {
    "OK".to_string()
}

pub fn json_error_handler(err: error::JsonPayloadError, _: &HttpRequest<AppState>) -> Error {
    error::InternalError::from_response(
        "",
        HttpResponse::BadRequest()
            .content_type("application/json")
            .body(format!(r#"{{"error":"json error: {}"}}"#, err)),
    )
    .into()
}

pub fn get_activities(
    req: &HttpRequest<AppState>,
) -> Result<Json<ActivitiesResponse>, AnalyzerError> {
    let jwt = &req.state().jwt;
    let log = &req.state().log;
    external::get_activities(jwt)
        .map_err(|e| {
            error!(log, "Get Activities ExternalServiceError {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}

pub fn get_activity(
    (req, activity_id): (HttpRequest<AppState>, Path<String>),
) -> Result<Json<ActivityResponse>, AnalyzerError> {
    let jwt = &req.state().jwt;
    let log = &req.state().log;
    external::get_activity(&activity_id, jwt)
        .map_err(|e| {
            error!(log, "Get Activity Error: {}", e);
            e
        })
        .map(Json)
}

pub fn create_activity(
    (req, activity): (HttpRequest<AppState>, Json<ActivityRequest>),
) -> Result<Json<ActivityResponse>, AnalyzerError> {
    let jwt = &req.state().jwt;
    let log = &req.state().log;
    info!(log, "creating activity {:?}", activity);
    external::create_activity(&activity, jwt)
        .map_err(|e| {
            error!(log, "Create Activity ExternalServiceError {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}

pub fn edit_activity(
    (req, activity, activity_id): (
        HttpRequest<AppState>,
        Json<EditActivityRequest>,
        Path<String>,
    ),
) -> Result<Json<ActivityResponse>, AnalyzerError> {
    let jwt = &req.state().jwt;
    let log = &req.state().log;
    info!(log, "editing activity {:?}", activity);
    external::edit_activity(&activity_id, &activity, jwt)
        .map_err(|e| {
            error!(log, "Edit Activity ExternalServiceError {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}

pub fn delete_activity(
    (req, activity_id): (HttpRequest<AppState>, Path<String>),
) -> Result<Json<ErrorListResponse>, AnalyzerError> {
    let jwt = &req.state().jwt;
    let log = &req.state().log;
    info!(log, "deleting activity {}", activity_id);
    external::delete_activity(&activity_id, jwt)
        .map_err(|e| {
            error!(log, "Delete Activity ExternalServiceError {}", e);
            AnalyzerError::ExternalServiceError
        })
        .map(Json)
}
