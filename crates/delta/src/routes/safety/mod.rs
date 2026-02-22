use voxly_rocket_okapi::voxly_okapi::openapi3::OpenApi;
use rocket::Route;

mod report_content;

pub fn routes() -> (Vec<Route>, OpenApi) {
    openapi_get_routes_spec![
        // Reports
        report_content::report_content,
    ]
}
