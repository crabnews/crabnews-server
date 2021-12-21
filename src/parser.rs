use std::future::Future;

use futures::future::join_all;
use rss::{Channel, Item};
use uuid::Uuid;

use crate::{
    error::{CrabError, CrabResult},
    subscription::{RssChannel, RssEntry},
};

pub trait Parse<P> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn parse(item: P) -> Self::Future;
}

struct Response(reqwest::Response);

#[derive(Debug)]
pub struct ReqwestParser;

impl Parse<Response> for ReqwestParser {
    type Output = CrabResult<RssChannel>;
    type Future = impl Future<Output = Self::Output>;

    fn parse(item: Response) -> Self::Future {
        let item = item.0;
        async {
            try {
                let status = item.status();
                println!("status: {:?}", status);
                let bytes = item.bytes().await?;
                let rss = Channel::read_from(&bytes[..])?;
                rss.into()
            }
        }
    }
}

// impl Parse<Vec<CrabResult<reqwest::Response>>> for ReqwestParser {
//     type Output = Vec<CrabResult<RssChannel>>;
//     type Future = impl Future<Output = Self::Output>;

//     fn parse(items: Vec<CrabResult<reqwest::Response>>) -> Self::Future {
//         async { join_all(items.into_iter().map(ReqwestParser::parse)).await }
//     }
// }
impl<I, T, P> Parse<I> for P
where
    P: Parse<T>,
    I: Iterator<Item = T>,
{
    type Output = Vec<P::Output>;
    type Future = impl Future<Output = Self::Output>;

    fn parse(items: I) -> Self::Future {
        async { join_all(items.into_iter().map(P::parse)).await }
    }
}

impl TryFrom<Item> for RssEntry {
    type Error = CrabError;

    fn try_from(entry: Item) -> Result<Self, Self::Error> {
        try {
            Self {
                uuid: Uuid::new_v4(),
                title: entry.title.expect(""),
                link: entry.link.expect(""),
                description: entry.description,
                date: entry.pub_date,
            }
        }
    }
}

impl From<Channel> for RssChannel {
    fn from(channel: Channel) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title: channel.title,
            entries: channel
                .items
                .into_iter()
                .filter_map(|item| item.try_into().ok())
                .collect(),
        }
    }
}
