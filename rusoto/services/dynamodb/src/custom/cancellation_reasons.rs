use rusoto_core::request::BufferedHttpResponse;
use rusoto_core::{proto, RusotoError};
use serde::Deserialize;
#[cfg(any(test, feature = "serialize_structs"))]
use serde::Serialize;

use crate::{AttributeValue, DynamoDbClient, TransactWriteItemsInput, TransactWriteItemsOutput};
use std::error::Error;
use std::fmt;

/// <p>An ordered list of errors for each item in the request which caused the transaction to get cancelled. The values of the list are ordered according to the ordering of the <code>TransactWriteItems</code> request parameter. If no error occurred for the associated item an error with a Null code and Null message will be present. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancellationReason {
    /// <p>Status code for the result of the cancelled transaction.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>Item in the request which caused the transaction to get cancelled.</p>
    #[serde(rename = "Item")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<::std::collections::HashMap<String, AttributeValue>>,
    /// <p>Cancellation reason message description.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Errors returned by TransactWriteItems, including the cancellation reasons
#[derive(Debug, PartialEq)]
pub enum TransactWriteItemsErrorWithCancellation {
    /// <p>DynamoDB rejected the request because you retried a request with a different payload but with an idempotent token that was already used.</p>
    IdempotentParameterMismatch(String),
    /// <p>An error occurred on the server side.</p>
    InternalServerError(String),
    /// <p>Your request rate is too high. The AWS SDKs for DynamoDB automatically retry requests that receive this exception. Your request is eventually successful, unless your retry queue is too large to finish. Reduce the frequency of requests and use exponential backoff. For more information, go to <a href="https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/Programming.Errors.html#Programming.Errors.RetryAndBackoff">Error Retries and Exponential Backoff</a> in the <i>Amazon DynamoDB Developer Guide</i>.</p>
    ProvisionedThroughputExceeded(String),
    /// <p>Throughput exceeds the current throughput quota for your account. Please contact AWS Support at <a href="https://aws.amazon.com/support">AWS Support</a> to request a quota increase.</p>
    RequestLimitExceeded(String),
    /// <p>The operation tried to access a nonexistent table or index. The resource might not be specified correctly, or its status might not be <code>ACTIVE</code>.</p>
    ResourceNotFound(String),
    /// <p><p>The entire transaction request was canceled.</p> <p>DynamoDB cancels a <code>TransactWriteItems</code> request under the following circumstances:</p> <ul> <li> <p>A condition in one of the condition expressions is not met.</p> </li> <li> <p>A table in the <code>TransactWriteItems</code> request is in a different account or region.</p> </li> <li> <p>More than one action in the <code>TransactWriteItems</code> operation targets the same item.</p> </li> <li> <p>There is insufficient provisioned capacity for the transaction to be completed.</p> </li> <li> <p>An item size becomes too large (larger than 400 KB), or a local secondary index (LSI) becomes too large, or a similar validation error occurs because of changes made by the transaction.</p> </li> <li> <p>There is a user error, such as an invalid data format.</p> </li> </ul> <p>DynamoDB cancels a <code>TransactGetItems</code> request under the following circumstances:</p> <ul> <li> <p>There is an ongoing <code>TransactGetItems</code> operation that conflicts with a concurrent <code>PutItem</code>, <code>UpdateItem</code>, <code>DeleteItem</code> or <code>TransactWriteItems</code> request. In this case the <code>TransactGetItems</code> operation fails with a <code>TransactionCanceledException</code>.</p> </li> <li> <p>A table in the <code>TransactGetItems</code> request is in a different account or region.</p> </li> <li> <p>There is insufficient provisioned capacity for the transaction to be completed.</p> </li> <li> <p>There is a user error, such as an invalid data format.</p> </li> </ul> <note> <p>If using Java, DynamoDB lists the cancellation reasons on the <code>CancellationReasons</code> property. This property is not set for other languages. Transaction cancellation reasons are ordered in the order of requested items, if an item has no error it will have <code>NONE</code> code and <code>Null</code> message.</p> </note> <p>Cancellation reason codes and possible error messages:</p> <ul> <li> <p>No Errors:</p> <ul> <li> <p>Code: <code>NONE</code> </p> </li> <li> <p>Message: <code>null</code> </p> </li> </ul> </li> <li> <p>Conditional Check Failed:</p> <ul> <li> <p>Code: <code>ConditionalCheckFailed</code> </p> </li> <li> <p>Message: The conditional request failed. </p> </li> </ul> </li> <li> <p>Item Collection Size Limit Exceeded:</p> <ul> <li> <p>Code: <code>ItemCollectionSizeLimitExceeded</code> </p> </li> <li> <p>Message: Collection size exceeded.</p> </li> </ul> </li> <li> <p>Transaction Conflict:</p> <ul> <li> <p>Code: <code>TransactionConflict</code> </p> </li> <li> <p>Message: Transaction is ongoing for the item.</p> </li> </ul> </li> <li> <p>Provisioned Throughput Exceeded:</p> <ul> <li> <p>Code: <code>ProvisionedThroughputExceeded</code> </p> </li> <li> <p>Messages: </p> <ul> <li> <p>The level of configured provisioned throughput for the table was exceeded. Consider increasing your provisioning level with the UpdateTable API.</p> <note> <p>This Message is received when provisioned throughput is exceeded is on a provisioned DynamoDB table.</p> </note> </li> <li> <p>The level of configured provisioned throughput for one or more global secondary indexes of the table was exceeded. Consider increasing your provisioning level for the under-provisioned global secondary indexes with the UpdateTable API.</p> <note> <p>This message is returned when provisioned throughput is exceeded is on a provisioned GSI.</p> </note> </li> </ul> </li> </ul> </li> <li> <p>Throttling Error:</p> <ul> <li> <p>Code: <code>ThrottlingError</code> </p> </li> <li> <p>Messages: </p> <ul> <li> <p>Throughput exceeds the current capacity of your table or index. DynamoDB is automatically scaling your table or index so please try again shortly. If exceptions persist, check if you have a hot key: https://docs.aws.amazon.com/amazondynamodb/latest/developerguide/bp-partition-key-design.html.</p> <note> <p>This message is returned when writes get throttled on an On-Demand table as DynamoDB is automatically scaling the table.</p> </note> </li> <li> <p>Throughput exceeds the current capacity for one or more global secondary indexes. DynamoDB is automatically scaling your index so please try again shortly.</p> <note> <p>This message is returned when when writes get throttled on an On-Demand GSI as DynamoDB is automatically scaling the GSI.</p> </note> </li> </ul> </li> </ul> </li> <li> <p>Validation Error:</p> <ul> <li> <p>Code: <code>ValidationError</code> </p> </li> <li> <p>Messages: </p> <ul> <li> <p>One or more parameter values were invalid.</p> </li> <li> <p>The update expression attempted to update the secondary index key beyond allowed size limits.</p> </li> <li> <p>The update expression attempted to update the secondary index key to unsupported type.</p> </li> <li> <p>An operand in the update expression has an incorrect data type.</p> </li> <li> <p>Item size to update has exceeded the maximum allowed size.</p> </li> <li> <p>Number overflow. Attempting to store a number with magnitude larger than supported range.</p> </li> <li> <p>Type mismatch for attribute to update.</p> </li> <li> <p>Nesting Levels have exceeded supported limits.</p> </li> <li> <p>The document path provided in the update expression is invalid for update.</p> </li> <li> <p>The provided expression refers to an attribute that does not exist in the item.</p> </li> </ul> </li> </ul> </li> </ul></p>
    TransactionCanceled {
        cancellation_reasons: Vec<CancellationReason>,
        cause: String,
    },
    /// <p>The transaction with the given request token is already in progress.</p>
    TransactionInProgress(String),
}

#[derive(Deserialize)]
struct CancellationReasons {
    #[serde(rename = "CancellationReasons")]
    cancellation_reasons: Option<Vec<CancellationReason>>,
}

impl TransactWriteItemsErrorWithCancellation {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<TransactWriteItemsErrorWithCancellation> {
        if let Some((err, ext)) = proto::json::Error::parse_ext::<CancellationReasons>(&res) {
            match err.typ.as_str() {
                "IdempotentParameterMismatchException" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::IdempotentParameterMismatch(
                            err.msg,
                        ),
                    )
                }
                "InternalServerError" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::InternalServerError(err.msg),
                    )
                }
                "ProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::ProvisionedThroughputExceeded(
                            err.msg,
                        ),
                    )
                }
                "RequestLimitExceeded" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::RequestLimitExceeded(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::ResourceNotFound(err.msg),
                    )
                }
                "TransactionCanceledException" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::TransactionCanceled {
                            cancellation_reasons: ext.cancellation_reasons.unwrap_or_else(Vec::new),
                            cause: err.msg,
                        },
                    )
                }
                "TransactionInProgressException" => {
                    return RusotoError::Service(
                        TransactWriteItemsErrorWithCancellation::TransactionInProgress(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TransactWriteItemsErrorWithCancellation {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransactWriteItemsErrorWithCancellation::IdempotentParameterMismatch(ref cause) => {
                write!(f, "{}", cause)
            }
            TransactWriteItemsErrorWithCancellation::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            TransactWriteItemsErrorWithCancellation::ProvisionedThroughputExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            TransactWriteItemsErrorWithCancellation::RequestLimitExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            TransactWriteItemsErrorWithCancellation::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            TransactWriteItemsErrorWithCancellation::TransactionCanceled { ref cause, .. } => {
                write!(f, "{}", cause)
            }
            TransactWriteItemsErrorWithCancellation::TransactionInProgress(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for TransactWriteItemsErrorWithCancellation {}

impl DynamoDbClient {
    pub async fn transact_write_items_with_cancellation(
        &self,
        input: TransactWriteItemsInput,
    ) -> Result<TransactWriteItemsOutput, RusotoError<TransactWriteItemsErrorWithCancellation>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "DynamoDB_20120810.TransactWriteItems");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                TransactWriteItemsErrorWithCancellation::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TransactWriteItemsOutput, _>()
    }
}

#[test]
fn deserialize_error_with_cancellation() {
    use http::StatusCode;

    // Source: https://github.com/boto/boto3/issues/2657
    let payload = r#"{"__type":"com.amazonaws.dynamodb.v20120810#TransactionCanceledException","CancellationReasons":[{"Code":"None"},{"Code":"ConditionalCheckFailed","Item":{"sk":{"S":"**********"},"gsi1sk":{"S":"**********"},"pk":{"S":"**********"},"gsi1pk":{"S":"**********"},"type":{"S":"**********"}},"Message":"The conditional request failed"}],"Message":"Transaction cancelled, please refer cancellation reasons for specific reasons [None, ConditionalCheckFailed]"}"#;
    let response = BufferedHttpResponse {
        status: StatusCode::OK,
        body: payload.into(),
        headers: Default::default(),
    };

    let (error, ext) = proto::json::Error::parse_ext::<CancellationReasons>(&response).unwrap();

    assert_eq!(error.typ, "TransactionCanceledException");
    assert_eq!(
        error.msg,
        "Transaction cancelled, please refer cancellation reasons for specific reasons [None, ConditionalCheckFailed]"
    );
    assert_eq!(
        ext.cancellation_reasons,
        Some(vec![
            CancellationReason {
                code: Some("None".to_string()),
                item: None,
                message: None
            },
            CancellationReason {
                code: Some("ConditionalCheckFailed".to_string()),
                item: Some(
                    vec![
                        ("sk", "**********"),
                        ("gsi1sk", "**********"),
                        ("pk", "**********"),
                        ("gsi1pk", "**********"),
                        ("type", "**********"),
                    ]
                    .into_iter()
                    .map(|(name, value)| (
                        name.to_string(),
                        AttributeValue {
                            s: Some(value.to_string()),
                            ..Default::default()
                        }
                    ))
                    .collect()
                ),
                message: Some("The conditional request failed".to_string())
            }
        ]),
    );
}
