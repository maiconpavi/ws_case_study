use aws_config::imds::client::error::BuildError;
use aws_sdk_apigatewaymanagement::error::SdkError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Serde Json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Lambda HTTP error: {0}")]
    LambdaHttp(#[from] lambda_http::http::Error),

    #[error("DynamoDB PutItem error: {0}")]
    PutItem(#[from] SdkError<aws_sdk_dynamodb::operation::put_item::PutItemError>),

    #[error("DynamoDB DeleteItem error: {0}")]
    DeleteItem(#[from] SdkError<aws_sdk_dynamodb::operation::delete_item::DeleteItemError>),

    #[error("DynamoDB ScanItems error: {0}")]
    ScanItems(#[from] SdkError<aws_sdk_dynamodb::operation::scan::ScanError>),

    #[error("Post Connection error: {0}")]
    PostConnection(
        #[from]
        SdkError<
            aws_sdk_apigatewaymanagement::operation::post_to_connection::PostToConnectionError,
        >,
    ),

    #[error("SerdeDynamo error: {0}")]
    SerdeDynamo(#[from] serde_dynamo::Error),

    #[error("AwsSmithyHttp error: {0}")]
    BuildError(#[from] BuildError),

    #[error("{0}")]
    Custom(String),
}

impl Error {
    pub fn custom(msg: impl Into<String>) -> Self {
        Self::Custom(msg.into())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
