use chrono::NaiveDateTime;
use lambda_http::{
    aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    lambda_runtime::LambdaEvent,
};
use serde_dynamo::to_item;

use crate::{
    config,
    error::{Error, Result},
    schema,
};

/// handler for connect event
/// # Errors
/// Returns [`IError`] if `connection_id` or `connected_at` is not found.
pub async fn handler(
    dynamodb_client: aws_sdk_dynamodb::Client,
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse> {
    let request_context = event.payload.request_context;
    let connection = schema::connection::Connection {
        connection_id: request_context
            .connection_id
            .ok_or_else(|| Error::custom("connection_id not found"))?
            .into(),
        connected_at: NaiveDateTime::from_timestamp_millis(request_context.connected_at)
            .ok_or_else(|| Error::custom("connected_at is invalid timestamp"))?,
    };

    dynamodb_client
        .put_item()
        .table_name(config::get().table_name.as_ref())
        .set_item(Some(to_item(connection)?))
        .send()
        .await?;
    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        ..Default::default()
    })
}
