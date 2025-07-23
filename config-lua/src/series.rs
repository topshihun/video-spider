use url::Url;

pub struct Episode {
    pub name: String,
    pub addr: Url,
}

pub struct Series {
    pub name: String,
    pub description: String,
    pub image: Url,
    pub episodes: Vec<Episode>,
}
