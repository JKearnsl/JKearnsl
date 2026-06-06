use std::collections::HashMap;
use crate::application::common::exceptions::ApplicationError;
use server_fn::error::ServerFnErrorErr;

impl server_fn::error::FromServerFnError for ApplicationError {
    type Encoder = server_fn::codec::JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        match value {
            ServerFnErrorErr::Args(msg) => {
                ApplicationError::ValidationError(HashMap::from([("args".to_string(), msg)]))
            }
            ServerFnErrorErr::MissingArg(field) => {
                ApplicationError::ValidationError(HashMap::from([(field, "обязательное поле".to_string())]))
            }
            ServerFnErrorErr::ServerError(msg) => ApplicationError::UnexpectedError(msg),
            other => ApplicationError::UnexpectedError(other.to_string()),
        }
    }
}
