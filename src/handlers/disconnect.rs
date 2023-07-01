use aws_sdk_dynamodb::types::AttributeValue;

use lambda_http::{
    aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    lambda_runtime::LambdaEvent,
};

use crate::{
    config,
    error::{Error, Result},
};

/// handler for disconnect event
/// # Errors
/// Returns [`IError`] if `connection_id` is not found.
pub async fn handler(
    dynamodb_client: aws_sdk_dynamodb::Client,
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse> {
    let connection_id = event
        .payload
        .request_context
        .connection_id
        .ok_or_else(|| Error::custom("connection_id not found"))?;

    dynamodb_client
        .delete_item()
        .table_name(config::get().table_name.as_ref())
        .key("connection_id", AttributeValue::S(connection_id))
        .send()
        .await?;
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        ..Default::default()
    })
}
