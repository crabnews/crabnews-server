use crabnews::{
    fetcher::{Fetch, ReqwestFetcher, Resource},
    parser::{Parse, ReqwestParser},
};

/// just mocking a request, where receive an url to fetch
const fn get_request() -> &'static str {
    "https://headcrab.rs/feed.xml"
}

#[tokio::main]
async fn main() {
    let url = get_request();
    let foo = ReqwestFetcher::fetch(std::iter::once(Resource(url))).await;
    let bar = ReqwestParser::parse(foo).await;
    println!("{:#?}", bar);
}
