#[derive(Debug, Clone)]
pub struct User {
    pub username : String,
    pub hash : String,
}

pub struct CreateAcct {
    pub username : String,
    pub pass : String,
}

pub struct ChangePassword {
    pub uuid : i64,
    pub pass : String,
}

pub struct Route {
    pub route_id : i32,
    pub wp : Vec<Waypoint>,
}

pub struct Waypoint {
    pub waypoint_id : i32,
    pub latitude : f64,
    pub longitude : f64,
}

pub struct AddFavorite {
    pub uuid : i64,
    pub route : Route,
}

pub struct DeleteFavorite {
    pub route_id : i32,
    pub uuid : i64,
}

pub struct RetrieveFavorites {
    pub uuid : i64,
}

pub struct Favorite {
    pub uuid : i64,
    pub route_id : i32,
}
