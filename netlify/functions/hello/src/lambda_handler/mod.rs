use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;

use http::header::HeaderMap;
use http::header::HeaderValue;
use lambda_runtime::{handler_fn, Context, Error};

use std::env;

mod mongo;

pub async fn start_listening() -> Result<(), Error> {
    let func = handler_fn(lambda_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn lambda_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let password = env::var("AUTHENTICATION_PASSWORD").expect("AUTHENTICATION_PASSWORD is not set.");

    println!("Headers have password?...");
    assert!(event.headers.contains_key("password"));
    println!("Headers have password!!!");

    // Good response
    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!("Hello from Rust"))),
        is_base64_encoded: Some(false),
    };

    let provided_password = event.headers.get("password");
    println!("Password is {}", provided_password.ok_or("Couldn't parse 'provided_password' to debug :'(")?.to_str()?);

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

    mongo::get_floorboards_collection("FloorBoards", "SampleFloors").await?;

    Ok(resp)
}
