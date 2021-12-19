use crabnews::fetcher::{Fetch, ReqwestFetcher, Resource};

#[tokio::main]
async fn main() {
    let foo = ReqwestFetcher::fetch(std::iter::once(Resource(
        "https://cat-fact.herokuapp.com/facts",
    )))
    .await;

    println!("{:#?}", foo);
}
