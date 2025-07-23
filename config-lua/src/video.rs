use url::Url;

pub struct Video {
    pub name: String,
    pub description: String,
    pub image: Url,
    pub urls: Vec<Url>,
}
