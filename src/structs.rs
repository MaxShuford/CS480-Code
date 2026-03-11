#[derive(Debug, Clone)]
pub struct User {
    pub username: String,
    pub pw_hash: String,
}

#[derive(Debug, Clone)]
pub struct ChangePassword {
    pub uuid: i64,
    pub pw_hash: String,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub route_id: i32,
    pub wp: Vec<Waypoint>,
}

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone)]
pub struct AddFavorite {
    pub uuid: i64,
    pub route: Route,
}

#[derive(Debug, Clone)]
pub struct DeleteFavorite {
    pub route_id: i32,
    pub uuid: i64,
}

#[derive(Debug, Clone)]
pub struct RetrieveFavorites {
    pub uuid: i64,
}

#[derive(Debug, Clone)]
pub struct Favorite {
    pub uuid: i64,
    pub route_id: i32,
}
