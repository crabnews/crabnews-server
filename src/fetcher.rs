use std::{fmt::Debug, future::Future};

use futures::future::join_all;

use crate::error::CrabResult;

pub trait Fetch<R: ?Sized + Debug> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn fetch(resource: R) -> Self::Future;
}

#[derive(Debug)]
pub struct Resource(pub &'static str);

pub struct ReqwestFetcher;

impl Fetch<Resource> for ReqwestFetcher {
    type Output = CrabResult<reqwest::Response>;
    type Future = impl Future<Output = Self::Output>;

    fn fetch(resource: Resource) -> Self::Future {
        tracing::debug!("fetching resource {:?}", &resource);
        async move { try { reqwest::get(resource.0).await? } }
    }
}

impl<I> Fetch<I> for ReqwestFetcher
where
    I: Iterator<Item = Resource> + Debug,
{
    type Output = Vec<CrabResult<reqwest::Response>>;
    type Future = impl Future<Output = Self::Output>;

    fn fetch(resources: I) -> Self::Future {
        tracing::debug!("fetching iterator");
        async { join_all(resources.map(Self::fetch)).await }
    }
}
