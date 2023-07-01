use aws_sdk_dynamodb::types::AttributeValue;
use lambda_http::{
    aws_lambda_events::apigw::{ApiGatewayProxyResponse, ApiGatewayWebsocketProxyRequest},
    lambda_runtime::LambdaEvent,
};

use crate::{
    config,
    error::{Error, Result},
    schema::{self, connection::Connection},
};
use aws_sdk_apigatewaymanagement::primitives::Blob;
use serde_dynamo::from_items;

/// handler for sendmessage event
/// # Errors
/// Returns [`IError`] if `body` is not found.
/// Returns [`IError`] the `body` is not valid [`Message`].
pub async fn handler(
    dynamodb_client: aws_sdk_dynamodb::Client,
    api_gateway_client: aws_sdk_apigatewaymanagement::Client,
    event: LambdaEvent<ApiGatewayWebsocketProxyRequest>,
) -> Result<ApiGatewayProxyResponse> {
    let raw_message = event
        .payload
        .body
        .ok_or_else(|| Error::custom("body not found"))?;

    let message: schema::message::Message = serde_json::from_str(&raw_message)?;

    let blob = Blob::new(serde_json::to_vec(&message)?);

    let connections = from_items::<_, Connection>(
        dynamodb_client
            .scan()
            .table_name(config::get().table_name.as_ref())
            .send()
            .await?
            .items
            .ok_or_else(|| Error::custom("items not found"))?,
    )?;

    futures::future::join_all(
        connections
            .into_iter()
            .map(|connection| async {
                let connection = connection;
                match api_gateway_client
                    .post_to_connection()
                    .connection_id(connection.connection_id.as_ref())
                    .data(blob.clone())
                    .send()
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        tracing::warn!(
                            "failed to send message to connection_id: {}, the connection will be closed, error: {}",
                            connection.connection_id,
                            e
                        );
                        dynamodb_client
                            .delete_item()
                            .table_name(config::get().table_name.as_ref())
                            .key(
                                "connection_id",
                                AttributeValue::S(connection.connection_id.to_string()),
                            )
                            .send()
                            .await
                            .map(|_| ())
                    }
                }
            })
            .collect::<Vec<_>>(),
    )
    .await
    .into_iter()
    .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(ApiGatewayProxyResponse {
        status_code: 200,
        ..Default::default()
    })
}
