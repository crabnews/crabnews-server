use std::{convert::{From, TryFrom}, future::Future};

use rss::{Channel, Item};

use crate::error::{CrabError, CrabResult};

pub trait Parser<P> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn parse(item: P) -> Self::Future;
}

#[derive(Debug)]
pub struct ReqwestParser;

impl Parser<CrabResult<reqwest::Response>> for ReqwestParser {
    type Output = CrabResult<RssChannel>;
    type Future = impl Future<Output = Self::Output>;

    fn parse(item: CrabResult<reqwest::Response>) -> Self::Future {
        async {
            match item {
                Ok(response) => {
                    try {
                        let status = response.status();
                        println!("status: {:?}", status);
                        let bytes = response.bytes().await?;
                        let rss = Channel::read_from(&bytes[..])?;
                        rss.into()
                    }
                }
                Err(_err) => {
                    todo!("Retry policy not implemented yet");
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct RssChannel {
    pub title: String,
    pub entries: Vec<RssEntry>,
}

#[derive(Debug)]
pub struct RssEntry {
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub date: Option<String>,
}

impl TryFrom<Item> for RssEntry {
    type Error = CrabError;

    fn try_from(entry: Item) -> Result<Self, Self::Error> {
        try {
            Self {
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
            title: channel.title,
            entries: channel.items.into_iter().filter_map(|item| item.try_into().ok()).collect(),
        }
    }
}
