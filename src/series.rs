use url::Url;

#[derive(PartialEq)]
pub struct Episode {
    pub name: String,
    pub addr: Url,
}

#[derive(PartialEq)]
pub struct Series {
    pub name: String,
    pub description: String,
    pub image: Url,
    pub episodes: Vec<Episode>,
}
