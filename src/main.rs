pub mod config;
pub mod error;
pub mod handlers;
pub mod schema;

use error::{Error, Result};
use lambda_http::{
    aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    lambda_runtime::{self, run, service_fn, LambdaEvent},
};

async fn parse_event(
    dynamodb_client: aws_sdk_dynamodb::Client,
    api_gateway_client: aws_sdk_apigatewaymanagement::Client,
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse> {
    tracing::info!("event: {:?}", event);
    let response = match event
        .payload
        .request_context
        .route_key
        .as_ref()
        .ok_or_else(|| Error::custom("route_key not found"))?
        .as_str()
    {
        "$connect" => handlers::connect::handler(dynamodb_client, event).await,
        "$disconnect" => handlers::disconnect::handler(dynamodb_client, event).await,
        "sendmessage" => {
            handlers::sendmessage::handler(dynamodb_client, api_gateway_client, event).await
        }
        _ => Err(Error::custom("route_key not found"))?,
    };
    response
}

#[tokio::main]
async fn main() -> std::result::Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let env_config = config::get();

    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);
    let endpoint_url = format!(
        "https://{api_id}.execute-api.{aws_region}.amazonaws.com/{stage}",
        api_id = env_config.api_id,
        aws_region = env_config.aws_region,
        stage = "default"
    );
    let api_management_config = aws_sdk_apigatewaymanagement::config::Builder::from(&config)
        .endpoint_url(endpoint_url)
        .build();
    let api_gateway_client = aws_sdk_apigatewaymanagement::Client::from_conf(api_management_config);

    run(service_fn(|event| async {
        match Box::pin(parse_event(
            dynamodb_client.clone(),
            api_gateway_client.clone(),
            event,
        ))
        .await
        {
            Ok(response) => Ok(response),
            Err(err) => {
                tracing::error!("error: {:?}", err);
                Err(err)
            }
        }
    }))
    .await
}
