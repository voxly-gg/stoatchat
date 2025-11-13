use std::net::{Ipv4Addr, SocketAddr};

use axum::Router;

use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod api;
pub mod requests;
pub mod website_embed;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Configure logging and environment
    revolt_config::configure!(proxy);

    // Configure API schema
    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::root,
            api::proxy,
            api::embed
        ),
        components(
            schemas(
                api::RootResponse,
                revolt_result::Error,
                revolt_result::ErrorType,
                revolt_models::v0::ImageSize,
                revolt_models::v0::Image,
                revolt_models::v0::Video,
                revolt_models::v0::TwitchType,
                revolt_models::v0::LightspeedType,
                revolt_models::v0::BandcampType,
                revolt_models::v0::Special,
                revolt_models::v0::WebsiteMetadata,
                revolt_models::v0::Text,
                revolt_models::v0::Embed
            )
        )
    )]
    struct ApiDoc;

    // Configure Axum and router
    let app = Router::new()
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        // .route("/scalar", {
        //     let html = Scalar::new(ApiDoc::openapi()).to_html();

        //     get(move || {
        //         let html = html.clone();
        //         async { Html(html) }
        //     })
        // })
        .nest("/", api::router().await);

    // Configure TCP listener and bind
    tracing::info!("Listening on 0.0.0.0:14705");
    tracing::info!("Play around with the API: http://localhost:14705/scalar");
    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 14705));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, app.into_make_service()).await
}
