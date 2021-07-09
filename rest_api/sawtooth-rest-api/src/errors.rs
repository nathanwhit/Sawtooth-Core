use core::fmt;
use serde_derive::{Deserialize, Serialize};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum RestApiErrorKind {
    BadProtobufSubmitted,
    BatchNotFound,
    BatchQueueFull,
    BlockNotFound,
    CountInvalid,
    HeadNotFound,
    InvalidResourceId,
    InvalidStateAddress,
    NoBatchesSubmitted,
    PagingInvalid,
    ReceiptBodyInvalid,
    ReceiptIdQueryInvalid,
    ReceiptNotFound,
    ReceiptWrongContentType,
    ResourceHeaderInvalid,
    SendBackoffTimeout,
    SortInvalid,
    StateNotFound,
    StatusBodyInvalid,
    StatusIdQueryInvalid,
    StatusResponseMissing,
    StatusWrongContentType,
    SubmissionWrongContentType,
    SubmittedBatchesInvalid,
    TransactionNotFound,
    UnknownValidator,
    ValidatorDisconnected,
    ValidatorNotReady,
    ValidatorResponseInvalid,
    ValidatorTimedOut,
}

impl fmt::Display for RestApiErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(&self, f)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RestApiError {
    info: Option<String>,
    kind: RestApiErrorKind,
}

impl From<RestApiErrorKind> for RestApiError {
    fn from(kind: RestApiErrorKind) -> Self {
        Self { info: None, kind }
    }
}

impl fmt::Display for RestApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(&self, f)
    }
}

impl RestApiError {
    fn to_response(&self) -> RestApiErrorResponse {
        RestApiErrorResponse {
            code: self.kind.api_code(),
            title: self.kind.title().to_owned(),
            message: if let Some(info) = &self.info {
                format!("{}{}", self.kind.message(), info)
            } else {
                self.kind.message().to_owned()
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
struct RestApiErrorResponse {
    code: u8,
    message: String,
    title: String,
}

impl ResponseError for RestApiError {
    fn status_code(&self) -> StatusCode {
        self.kind.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.to_response())
    }
}

impl RestApiErrorKind {
    pub fn status_code(self) -> StatusCode {
        use RestApiErrorKind::*;
        StatusCode::from_u16(match self {
            BadProtobufSubmitted => 400,
            BatchNotFound => 404,
            BatchQueueFull => 429,
            BlockNotFound => 404,
            CountInvalid => 400,
            HeadNotFound => 404,
            InvalidResourceId => 400,
            InvalidStateAddress => 400,
            NoBatchesSubmitted => 400,
            PagingInvalid => 400,
            ReceiptBodyInvalid => 400,
            ReceiptIdQueryInvalid => 400,
            ReceiptNotFound => 404,
            ReceiptWrongContentType => 400,
            ResourceHeaderInvalid => 500,
            SendBackoffTimeout => 408,
            SortInvalid => 400,
            StateNotFound => 404,
            StatusBodyInvalid => 400,
            StatusIdQueryInvalid => 400,
            StatusResponseMissing => 500,
            StatusWrongContentType => 400,
            SubmissionWrongContentType => 400,
            SubmittedBatchesInvalid => 400,
            TransactionNotFound => 404,
            UnknownValidator => 500,
            ValidatorDisconnected => 503,
            ValidatorNotReady => 503,
            ValidatorResponseInvalid => 500,
            ValidatorTimedOut => 503,
        })
        .unwrap()
    }

    pub fn api_code(self) -> u8 {
        use RestApiErrorKind::*;
        match self {
            BadProtobufSubmitted => 35,
            BatchNotFound => 71,
            BatchQueueFull => 31,
            BlockNotFound => 70,
            CountInvalid => 53,
            HeadNotFound => 50,
            InvalidResourceId => 60,
            InvalidStateAddress => 62,
            NoBatchesSubmitted => 34,
            PagingInvalid => 54,
            ReceiptBodyInvalid => 82,
            ReceiptIdQueryInvalid => 83,
            ReceiptNotFound => 80,
            ReceiptWrongContentType => 81,
            ResourceHeaderInvalid => 21,
            SendBackoffTimeout => 19,
            SortInvalid => 57,
            StateNotFound => 75,
            StatusBodyInvalid => 46,
            StatusIdQueryInvalid => 66,
            StatusResponseMissing => 27,
            StatusWrongContentType => 43,
            SubmissionWrongContentType => 42,
            SubmittedBatchesInvalid => 30,
            TransactionNotFound => 72,
            UnknownValidator => 10,
            ValidatorDisconnected => 18,
            ValidatorNotReady => 15,
            ValidatorResponseInvalid => 20,
            ValidatorTimedOut => 17,
        }
    }

    pub fn title(self) -> &'static str {
        use RestApiErrorKind::*;
        match self {
            BadProtobufSubmitted => "Protobuf Not Decodable",
            BatchNotFound => "Batch Not Found",
            BatchQueueFull => "Unable to Accept Batches",
            BlockNotFound => "Block Not Found",
            CountInvalid => "Invalid Count Query",
            HeadNotFound => "Head Not Found",
            InvalidResourceId => "Invalid Resource Id",
            InvalidStateAddress => "Invalid State Address",
            NoBatchesSubmitted => "No Batches Submitted",
            PagingInvalid => "Invalid Paging Query",
            ReceiptBodyInvalid => "Bad Receipts Request",
            ReceiptIdQueryInvalid => "Id Query Invalid or Missing",
            ReceiptNotFound => "Transaction Receipt Not Found",
            ReceiptWrongContentType => "Wrong Content Type",
            ResourceHeaderInvalid => "Invalid Resource Header",
            SendBackoffTimeout => "Send timed out",
            SortInvalid => "Invalid Sort Query",
            StateNotFound => "State Not Found",
            StatusBodyInvalid => "Bad Status Request",
            StatusIdQueryInvalid => "Id Query Invalid or Missing",
            StatusResponseMissing => "Unable to Fetch Statuses",
            StatusWrongContentType => "Wrong Content Type",
            SubmissionWrongContentType => "Wrong Content Type",
            SubmittedBatchesInvalid => "Submitted Batches Invalid",
            TransactionNotFound => "Transaction Not Found",
            UnknownValidator => "Unknown Validator Error",
            ValidatorDisconnected => "Validator Disconnected",
            ValidatorNotReady => "Validator Not Ready",
            ValidatorResponseInvalid => "Invalid Validator Response",
            ValidatorTimedOut => "Validator Timed Out",
        }
    }

    pub fn message(self) -> &'static str {
        use RestApiErrorKind::*;
        match self {
            BadProtobufSubmitted => "The protobuf BatchList you submitted was malformed and could not be read.",
            BatchNotFound => "There is no batch with the id specified in the blockchain.",
            BatchQueueFull => "The validator cannot currently accept more batches, due to a full queue.  Please submit your request again.",
            BlockNotFound => "There is no block with the id specified in the blockchain.",
            CountInvalid => "The 'count' query parameter must be a positive, non-zero integer.",
            HeadNotFound => "There is no block with the id specified in the 'head' query parameter.",
            InvalidResourceId => "Blockchain items are identified by 128 character hex-strings. A submitted block, batch, or transaction id was invalid: ",
            InvalidStateAddress => "The state address submitted was invalid. To fetch specific state data, you must submit the full 70-character address.",
            NoBatchesSubmitted => "The protobuf BatchList you submitted was empty and contained no Batches. You must submit at least one Batch.",
            PagingInvalid => "Paging request failed as written. One or more of the 'min', 'max', or 'count' query parameters were invalid or out of range.",
            ReceiptBodyInvalid => "Requests for transaction receipts sent as a POST must have a JSON formatted body with an array of at least one id string.",
            ReceiptIdQueryInvalid => "Requests for transaction receipts sent as a GET request must have an 'id' query parameter with a comma-separated list of at least one transaction id.",
            ReceiptNotFound => "There is no transaction receipt for the transaction id specified in the receipt store.",
            ReceiptWrongContentType => "Requests for transaction receipts sent as a POST must have a 'Content-Type' header of 'application/json'.",
            ResourceHeaderInvalid => "The resource fetched from the validator had an invalid header, and may be corrupted.",
            SendBackoffTimeout => "Sending message to validator timed out. Retry limit reached. Try your request again later.",
            SortInvalid => "The sort request failed as written. Some of the keys specified were not valid.",
            StateNotFound => "There is no state data at the address specified.",
            StatusBodyInvalid => "Requests for batch statuses sent as a POST must have a JSON formatted body with an array of at least one id string.",
            StatusIdQueryInvalid => "Requests for batch statuses sent as a GET request must have an 'id' query parameter with a comma-separated list of at least one batch id.",
            StatusResponseMissing => "An unknown error occurred while attempting to fetch batch statuses, and nothing was returned.",
            StatusWrongContentType => "Requests for batch statuses sent as a POST must have a 'Content-Type' header of 'application/json'.",
            SubmissionWrongContentType => "Batches must be submitted in a BatchList protobuf binary, with a 'Content-Type' header of 'application/octet-stream'.",
            SubmittedBatchesInvalid => "The submitted BatchList was rejected by the validator. It was poorly formed, or has an invalid signature.",
            TransactionNotFound => "There is no transaction with the id specified in the blockchain.",
            UnknownValidator => "An unknown error occurred with the validator while processing your request.",
            ValidatorDisconnected => "The validator disconnected before sending a response. Try your request again later.",
            ValidatorNotReady => "The validator has no genesis block, and is not yet ready to be queried. Try your request again later.",
            ValidatorResponseInvalid => "The response from the validator could not be decoded. It may have been corrupted or compromised.",
            ValidatorTimedOut => "The request timed out while waiting for a response from the validator. Your request may or may not have been processed.",
        }
    }
}
