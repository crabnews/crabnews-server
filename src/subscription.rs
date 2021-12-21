use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Client {
    pub uuid: Uuid,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct LastRead {
    pub client: Client,
    pub channel: RssChannel,
    pub post: Option<RssEntry>,
}

#[derive(Debug, Clone)]
pub struct RssChannel {
    pub uuid: Uuid,
    pub title: String,
    pub entries: Vec<RssEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RssEntry {
    pub uuid: Uuid,
    pub title: String,
    pub link: String,
    pub description: Option<String>,
    pub date: Option<String>,
}

