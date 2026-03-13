use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub username: String,
    pub pw_hash: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePassword {
    pub uuid: i64,
    pub old_pw: String,
    pub new_pw: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Route {
    pub route_id: i32,
    pub wp: Vec<Waypoint>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Waypoint {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddFavorite {
    pub uuid: i64,
    pub route: Route,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeleteFavorite {
    pub route_id: i32,
    pub uuid: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RetrieveFavorites {
    pub uuid: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Favorite {
    pub uuid: i64,
    pub route_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteReturn {
    pub name: String,
    pub route_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RouteToMap {
    pub route: Route,
    pub geometry: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DirectionOptions {
    pub code: String,
    pub routes: Vec<RouteWithDirections>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RouteWithDirections {
    pub waypoints: Vec<Waypoint>,
    pub directions: Vec<String>,
    pub geometry: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserEnteredLocation {
    pub city: String,
    pub state: String,
}

#[derive(Debug, Clone)]
pub struct APIKeys {
    pub geocoding: String,
    pub mapbox: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Base64Image {
    pub image_type: String,
    pub image: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct FavoritesList {
    pub favorites: Vec<FavoriteReturn>,
}

