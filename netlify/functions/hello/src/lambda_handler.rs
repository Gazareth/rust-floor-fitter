use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;

use http::header::HeaderMap;
use http::header::HeaderValue;
use lambda_runtime::{handler_fn, Context, Error};

use std::env;

pub async fn start_listening() -> Result<(), Error> {
    let func = handler_fn(lambda_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn lambda_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let password = env::var("AUTHENTICATION_PASSWORD").expect("AUTHENTICATION_PASSWORD is not set.");

    assert!(event.headers.contains_key("password"));

    // Good response
        let resp = ApiGatewayProxyResponse {
            status_code: 200,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(format!("Hello from Rust"))),
            is_base64_encoded: Some(false),
        };

    let provided_password = event.headers.get("password");

    // If password no match, bad response!
    if provided_password != Some(&HeaderValue::from_str(&password)?) {
        let bad_resp = ApiGatewayProxyResponse {
            status_code: 401,
            headers: HeaderMap::new(),
            multi_value_headers: HeaderMap::new(),
            body: Some(Body::Text(format!("Unauthorized"))),
            is_base64_encoded: Some(false),
        };

        return Ok(bad_resp);
    }

    Ok(resp)
}
