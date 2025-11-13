use utoipa::{Modify, openapi::{OpenApi, security::{ApiKey, ApiKeyValue, SecurityScheme}}};

pub struct TokenSecurity;

impl Modify for TokenSecurity {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi.components.get_or_insert_default();

        components.add_security_scheme(
            "Session-Token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new(
                "X-Session-Token".to_string(),
            ))),
        );

        components.add_security_scheme(
            "Bot-Token",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Bot-Ticket".to_string()))),
        );
    }
}