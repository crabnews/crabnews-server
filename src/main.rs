use crabnews::{
    fetcher::{Fetch, ReqwestFetcher, Resource},
    parser::{Parser, ReqwestParser},
};

#[tokio::main]
async fn main() {
    let foo = ReqwestFetcher::fetch(Resource("https://headcrab.rs/feed.xml")).await;
    let bar = ReqwestParser::parse(foo).await;
    println!("{:#?}", bar);
}
