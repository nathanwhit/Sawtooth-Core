use core::fmt;
use std::ops::{Deref, DerefMut};

use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, ResponseError};
use futures_util::future::LocalBoxFuture;
use protobuf::Message;

pub struct ProtoBuf<T: Message>(pub T);

impl<T: Message> Deref for ProtoBuf<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Message> DerefMut for ProtoBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Message + fmt::Debug> fmt::Debug for ProtoBuf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ProtoBuf: {:?}", self.0)
    }
}

impl<T: Message + fmt::Display> fmt::Display for ProtoBuf<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

pub struct ProtoBufConfig {
    client_max_size: usize,
}

impl ProtoBufConfig {
    fn client_max_size(&mut self, client_max_size: usize) -> &mut Self {
        self.client_max_size = client_max_size;
        self
    }
}

impl Default for ProtoBufConfig {
    fn default() -> Self {
        Self {
            client_max_size: crate::MAX_SIZE_DEFAULT,
        }
    }
}

impl<T> FromRequest for ProtoBuf<T>
where
    T: Message + Default + 'static,
{
    type Error = Error;

    type Future = LocalBoxFuture<'static, Result<Self, Error>>;

    type Config = ProtoBufConfig;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let max_size = req
            .app_data::<ProtoBufConfig>()
            .map(|conf| conf.client_max_size)
            .unwrap_or(crate::MAX_SIZE_DEFAULT);
        todo!()
    }
}

// pub struct ProtoBufMessage<T: Message + Default> {
//     max_size: usize,
//     length: Option<usize>,
//     stream: Option<Payload>,
//     err: Option<Error>,
//     fut: Option<LocalBoxFuture<'static, Result<T, Error>>>,
// }

// impl<T: Message + Default> ProtoBufMessage<T> {
//     pub fn new(req: &HttpRequest, payload: &mut Payload) -> Self {

//     }
// }

include!(concat!(env!("OUT_DIR"), "/proto/mod.rs"));
