use rocket::Route;

mod report_content;

#[derive(OpenApi)]
#[openapi(
    paths(
        report_content::report_content
    )
)]
pub struct ApiDoc;

pub fn routes() -> Vec<Route> {
    routes![
        // Reports
        report_content::report_content,
    ]
}
