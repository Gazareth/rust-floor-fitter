use aws_lambda_events::event::apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse};
use aws_lambda_events::encodings::Body;

use http::header::HeaderMap;
use lambda_runtime::{handler_fn, Context, Error};



pub async fn start_listening() -> Result<(), Error> {
    let func = handler_fn(lambda_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

pub(crate) async fn lambda_handler(event: ApiGatewayProxyRequest, _ctx: Context) -> Result<ApiGatewayProxyResponse, Error> {
    let path = event.path.unwrap();

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(Body::Text(format!("Hello from '{}'", path))),
        is_base64_encoded: Some(false),
    };

    Ok(resp)
}
