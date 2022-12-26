struct Package {
    pub name: String,
    pub version: String,
    pub fetch_url: String,
}
struct DepTree {
    pub packages: Vec<Package>,
}

impl DepTree {
    
}