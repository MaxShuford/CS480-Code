mod api_service;
mod error;
mod favorite_model;
mod routes;
pub mod structs;
mod user_model;

use error::Error::*;
use mysql::*;

use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

// === Static File Handler ===
fn serve_static_file(path: &str) {
    // Determine base directory and relative path
    let (base_dir, rel_path) = if path.starts_with("/html/") {
        ("html", &path[6..])
    } else if path.starts_with("/js/") {
        ("js", &path[4..])
    } else if path.starts_with("/css/") {
        ("css", &path[5..])
    } else {
        println!("Unknown static file prefix: {}", path);
        return;
    };

    // Reject any path that contains ".." or additional slashes
    if rel_path.contains("..") || rel_path.contains('/') {
        println!("Security: invalid file path '{}'", rel_path);
        return;
    }

    let full_path = format!("{}/{}", base_dir, rel_path);
    println!("Serving static file: {}", full_path);

    // TODO: read file and return HTTP response?
}

// === Helper ===
// Extract file name from GET request line, ex: "GET /html/Login.html" -> ("GET/html", "Login.html")
// "GET/html" will be used for matching and "Login.html" will be used for file retrieving
// 3 match conditions and 3 ways to retrieve files
fn extract_file_name(request_line: &str) -> Option<(String, String)> {
    if &request_line[..] == "GET / HTTP/1.1" {
        return None;
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next()?;
    let path = parts.next()?;

    if method != "GET" {
        return None;
    }

    // Support three static prefixes: /html/, /js/, /css/
    // Extract the prefix and the rest of the path
    let (prefix, rest) = if let Some(rest) = path.strip_prefix("/html/") {
        ("/html", rest)
    } else if let Some(rest) = path.strip_prefix("/js/") {
        ("/js", rest)
    } else if let Some(rest) = path.strip_prefix("/css/") {
        ("/css", rest)
    } else {
        return None;
    };

    // ignore requests looking for parent or subdirectories
    if rest.contains("..") || rest.contains("/") {
        return None;
    }

    let match_key = format!("{} {}", method, prefix); // ex: "GET /html"
    println!("{match_key}");
    Some((match_key, rest.to_string()))
}

fn handle_404() {
    println!("404 Not Found");
}

// === Router ===
fn route_request(method: &str, path: &str) {
    let route = format!("{} {}", method, path);

    match route.as_str() {
        // HTML pages
        routes::GET_LOGIN_PAGE => serve_static_file("/html/Login.html"),
        routes::GET_CREATE_ACCOUNT_PAGE => serve_static_file("/html/CreateAccount.html"),
        routes::GET_CHANGE_PASSWORD_PAGE => serve_static_file("/html/ChangePassword.html"),
        routes::GET_SELECT_WAYPOINTS_PAGE => serve_static_file("/html/SelectWaypoints.html"),
        routes::GET_SELECT_ROUTE_PAGE => serve_static_file("/html/SelectRoute.html"),
        routes::GET_VIEW_ROUTE_PAGE => serve_static_file("/html/ViewRoute.html"),

        // API endpoints
        /*
                routes::POST_LOGIN => handle_login(),
                routes::POST_CREATE_ACCOUNT => handle_create_account(),
                routes::POST_CHANGE_PASSWORD => handle_change_password(),
                routes::POST_ADD_FAVORITE => handle_add_favorite(),
                routes::POST_DELETE_FAVORITE => handle_delete_favorite(),
                routes::POST_RETRIEVE_FAVORITES => handle_retrieve_favorites(),
                routes::POST_RETRIEVE_FAVORITE => handle_retrieve_favorite(),
        */
        // Static assets (JS, CSS)
        _ if method == "GET" && (path.starts_with("/js/") || path.starts_with("/css/")) => {
            serve_static_file(path)
        }

        // Fallback
        _ => handle_404(),
    }
}

fn main() {
    let api_keys = fs::read_to_string("api_keys.txt").expect("Unable to read api_keys");
    let mut lines = api_keys.lines();
    let geocoding = lines.next().unwrap_or("");
    let mapbox = lines.next().unwrap_or("");

    let api_keys = structs::APIKeys {
        geocoding: geocoding.to_string(),
        mapbox: mapbox.to_string(),
    };

    // listen to local hosted
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream, &api_keys);
    }

    route_request("GET", "/html/Login.html");
    route_request("POST", "/login");
    route_request("GET", "/html/SelectWaypoints.html");
    route_request("GET", "/js/main.js");
    route_request("GET", "/css/header.css");
    route_request("POST", "/retrieveFavorites");
    route_request("GET", "/nonexistent");

    let raw = "GET /html/Login.html HTTP/1.1";
    if let Some(file) = extract_file_name(raw) {}
}

fn handle_stream(mut stream: TcpStream, api_keys: &structs::APIKeys) {
    println!("request recieved");
    let mut reader = BufReader::new(&mut stream);

    // read request line
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();
    println!("=== Request Line ===\n{}", request_line);

    // read the headers
    let mut headers = Vec::new();
    let mut content_length = 0;

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();

        if line == "\r\n" || line == "\n" {
            break;
        }

        if line.to_lowercase().starts_with("content-length:") {
            content_length = line
                .split(':')
                .nth(1)
                .and_then(|v| v.trim().parse::<usize>().ok())
                .unwrap_or(0);
        }
        headers.push(line.trim().to_string());
    }

    println!("=== Headers ===");
    for h in headers {
        println!("{}", h);
    }

    // read the body
    let mut body_content = String::new();
    if content_length > 0 {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        let body = String::from_utf8_lossy(&body);
        body_content.push_str(body.as_ref());
        println!("=== Body ===\n{}", body_content);
    } else {
        println!("=== No Body ===");
    }

    let (status_line, content_type, response_body) =
        handle_request(request_line.as_str(), body_content.as_str(), api_keys);

    // read the body
    let mut body_content = String::new();
    if content_length > 0 {
        let mut body = vec![0; content_length];
        reader.read_exact(&mut body).unwrap();
        let body = String::from_utf8_lossy(&body);
        body_content.push_str(body.as_ref());
        println!("=== Body ===\n{}", body_content);
    } else {
        println!("=== No Body ===");
    }

    let request_line = request_line.trim_end().to_string();
    // hard code image response cuz we need it done :)
    if &request_line[..] == "GET /css/images/SearchIcon.png HTTP/1.1" {
        let contents = fs::read("css/Images/SearchIcon.png").unwrap();
        let content_type = "image/png";
        let length = contents.len();
        let response = format! {"HTTP/1.1 200 OK
Content-Type: {content_type}
Content-Length: {length}\r\n\r\n"};
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        return;
    } else if &request_line[..] == "GET /css/Images/UserIcon.png HTTP/1.1" {
        let contents = fs::read("css/Images/UserIcon.png").unwrap();
        let content_type = "image/png";
        let length = contents.len();
        let response = format! {"HTTP/1.1 200 OK
Content-Type: {content_type}
Content-Length: {length}\r\n\r\n"};
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        return;
    } else if &request_line[..] == "GET /css/Images/ArrowIcon.png HTTP/1.1" {
        let contents = fs::read("css/Images/ArrowIcon.png").unwrap();
        let content_type = "image/png";
        let length = contents.len();
        let response = format! {"HTTP/1.1 200 OK
Content-Type: {content_type}
Content-Length: {length}\r\n\r\n"};
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        return;
    } else if &request_line[..] == "GET /css/Images/BackIcon.png HTTP/1.1" {
        let contents = fs::read("css/Images/BackIcon.png").unwrap();
        let content_type = "image/png";
        let length = contents.len();
        let response = format! {"HTTP/1.1 200 OK
Content-Type: {content_type}
Content-Length: {length}\r\n\r\n"};
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        return;
    } else if &request_line[..] == "GET /css/Images/TrashIcon.png HTTP/1.1" {
        let contents = fs::read("css/Images/TrashIcon.png").unwrap();
        let content_type = "image/png";
        let length = contents.len();
        let response = format! {"HTTP/1.1 200 OK
Content-Type: {content_type}
Content-Length: {length}\r\n\r\n"};
        stream.write(response.as_bytes()).unwrap();
        stream.write(&contents).unwrap();
        return;
    }

    let length = response_body.len();
    let response = format! {
        "{status_line}
Content-Type: {content_type}
Content-Length: {length}\r\n\r\n\
    {response_body}"
    };
    println!("{response}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("response sent");
}

fn handle_request(
    request_line: &str,
    body_content: &str,
    api_keys: &structs::APIKeys,
) -> (String, String, String) {
    let (request_code, file_name) = match extract_file_name(request_line) {
        Some((file_type, file_name)) => (format!("{file_type}"), file_name),
        None => (String::from(request_line.trim_end()), String::from("")),
    };
    println!("Request Code: {request_code}, File_name: {file_name}");

    //create database connection
    let url = "mysql://user:password@localhost:3306/testdb";
    // TODO: Test dbc connecter with db
    /*
        let pool = Pool::new(url).expect("Failed to create database pool");
        let mut conn = pool.get_conn().expect("Failed to get database connection");
    */

    let (status_code, content_type, body) = match &request_code[..] {
        // Initial page request
        "GET / HTTP/1.1" => {
            println!("LANDING PAGE RETRIEVAL");
            let contents = file_retrieve("html", "Login.html");
            if contents.as_str() == "File not Found" {
                (
                    "HTTP/1.1 404 Not Found",
                    "text/plain",
                    "Not Found.".to_string(),
                )
            } else {
                ("HTTP/1.1 200 Ok", "text/html", contents)
            }
        }
        // any html page request
        "GET /html" => {
            let contents = file_retrieve("html", file_name.as_str());
            if contents.as_str() == "File not Found" {
                (
                    "HTTP/1.1 404 Not Found",
                    "text/plain",
                    "Not Found.".to_string(),
                )
            } else {
                ("HTTP/1.1 200 Ok", "text/html", contents)
            }
        }
        // any js page request
        "GET /js" => {
            let contents = file_retrieve("js", file_name.as_str());
            if contents.as_str() == "File not Found" {
                (
                    "HTTP/1.1 404 Not Found",
                    "text/plain",
                    "Not Found.".to_string(),
                )
            } else {
                (
                    "HTTP/1.1 200 Ok",
                    "text/javascript; charset=utf-8",
                    contents,
                )
            }
        }
        // any css page request
        "GET /css" => {
            let contents = file_retrieve("css", file_name.as_str());
            if contents.as_str() == "File not Found" {
                (
                    "HTTP/1.1 404 Not Found",
                    "text/plain",
                    "Not Found.".to_string(),
                )
            } else {
                ("HTTP/1.1 200 Ok", "text/css", contents)
            }
        }
        // requesting to turn city state code into lat lon
        "POST /locationData HTTP/1.1" => {
            let location: structs::UserEnteredLocation =
                serde_json::from_str(body_content).expect("invalid location json file");
            let response = match api_service::geocoding(
                api_keys.geocoding.as_str(),
                location.city.as_str().to_lowercase().trim(),
                location.state.as_str().to_lowercase().trim(),
            ) {
                Ok((lat, lon)) => {
                    let wp_name = format!("{}, {}", location.city, location.state);
                    let wp = structs::Waypoint {
                        id: 0,
                        name: wp_name,
                        latitude: lat as f64,
                        longitude: lon as f64,
                    };
                    (
                        "HTTP/1.1 200 Ok",
                        "application/json",
                        serde_json::to_string(&wp).expect("Invalid lat and lon struct"),
                    )
                }
                Err(_) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from("Invalid location"),
                ),
            };

            response
        }
        // requesting routes from a list of waypoints
        "POST /directions HTTP/1.1" => {
            let waypoints: Vec<structs::Waypoint> =
                serde_json::from_str(body_content).expect("invalid waypoint json file");
            let waypoints: Vec<(f32, f32)> = waypoints
                .iter()
                .map(|s| (s.latitude as f32, s.longitude as f32))
                .collect();

            let response = match api_service::directions(api_keys.mapbox.as_str(), waypoints) {
                Ok(routes) => (
                    "HTTP/1.1 200 Ok",
                    "application/json",
                    serde_json::to_string(&routes).expect("Invalid route object"),
                ),
                Err(_) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from("Unable to route"),
                ),
            };

            response
        }
        // requesting a map with routes on it
        "POST /mapWithRoutes HTTP/1.1" => {
            println!("generating map");
            let routes: Vec<structs::RouteToMap> =
                serde_json::from_str(body_content).expect("Invalid routes with polyline json");
            let response =
                match api_service::static_images_with_routes(routes, api_keys.mapbox.as_str()) {
                    Ok(image_str) => (
                        "HTTP/1.1 200 Ok",
                        "application/json",
                        serde_json::to_string(&structs::Base64Image {
                            image_type: "png".to_string(),
                            image: image_str,
                        })
                        .expect("Invalid  struct base64Image"),
                    ),
                    Err(_) => (
                        "HTTP/1.1 400 Bad Request",
                        "text/plain",
                        String::from("Unable to map"),
                    ),
                };
            println!("Map generated");
            response
        }
        // requesting a map centered around the users location
        "POST /mapWithUserLoc HTTP/1.1" => {
            println!("generating map");
            let loc: structs::UserLocation =
                serde_json::from_str(body_content).expect("Invalid routes with polyline json");
            let loc = (loc.latitude as f32, loc.longitude as f32);
            let response =
                match api_service::static_images_with_user_loc(loc, api_keys.mapbox.as_str()) {
                    Ok(image_str) => (
                        "HTTP/1.1 200 Ok",
                        "application/json",
                        serde_json::to_string(&structs::Base64Image {
                            image_type: "png".to_string(),
                            image: image_str,
                        })
                        .expect("Invalid  struct base64Image"),
                    ),
                    Err(_) => (
                        "HTTP/1.1 400 Bad Request",
                        "text/plain",
                        String::from("Unable to map user location"),
                    ),
                };
            println!("Map generated");
            response
        }

        // TODO: test the following matches with db
        /*
        // login handle
        "POST /login HTTP/1.1" => {
            let credentials: structs::User =
                serde_json::from_str(body_content).expect("Invalid login credentials json");
            let response = match user_model::login(&mut conn, credentials) {
                Ok(uuid) => (
                    "HTTP/1.1 200 Ok",
                    "application/json",
                    String::from(format!("{{\"uuid\":{}}}", uuid)),
                ),
                Err(LoginFailed {
                    username: err_string,
                }) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from(format!("{err_string} failed to login")),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred during login"),
                ),
            };
            response
        }

        // create account handle
        "POST /createAccount HTTP/1.1" => {
            let credentials: structs::User = serde_json::from_str(body_content)
                .expect("Invalid create account credentials json");
            let response = match user_model::create_account(&mut conn, credentials) {
                Ok(uuid) => (
                    "HTTP/1.1 200 Ok",
                    "application/json",
                    String::from(format!("{{\"uuid\":{}}}", uuid)),
                ),
                Err(UserExists {
                    username: err_string,
                }) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from(format!("{err_string} already exists.")),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred during account creation"),
                ),
            };
            response
        }

        // change password handle
        "POST /changePassword HTTP/1.1" => {
            let credentials: structs::ChangePassword = serde_json::from_str(body_content)
                .expect("Invalid change password credentials json");
            let response = match user_model::change_pass(&mut conn, credentials) {
                Ok(uid) => (
                    "HTTP/1.1 200 Ok",
                    "text/plain",
                    String::from("Password changed successfully"),
                ),
                Err(IncorrectPassword) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    "Incorrect current password".to_string(),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred during password change"),
                ),
            };
            response
        }

        // add favorite handle
        "POST /addFavorite HTTP/1.1" => {
            let favorite: structs::AddFavorite =
                serde_json::from_str(body_content).expect("Invalid add favorite json");
            let response = match favorite_model::add_fav(&mut conn, favorite) {
                Ok(()) => (
                    "HTTP/1.1 200 Ok",
                    "text/plain",
                    String::from("Favorite added successfully"),
                ),
                Err(MaxRoutesExceeded) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from("Maximum number of favorite routes exceeded"),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred while adding favorite"),
                ),
            };
            response
        }

        // delete favorite handle
        "POST /deleteFavorite HTTP/1.1" => {
            let favorite: structs::DeleteFavorite =
                serde_json::from_str(body_content).expect("Invalid delete favorite json");
            let response = match favorite_model::delete_fav(&mut conn, favorite) {
                Ok(()) => (
                    "HTTP/1.1 200 Ok",
                    "text/plain",
                    String::from("Favorite deleted successfully"),
                ),
                Err(DeleteUnsuccessful) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from("Unable to delete favorite route"),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred while deleting favorite"),
                ),
            };
            response
        }

        // retrieve favorites handle
        "POST /retrieveFavorites HTTP/1.1" => {
            let request: structs::RetrieveFavorites =
                serde_json::from_str(body_content).expect("Invalid retrieve favorites json");
            let response = match favorite_model::get_favorites(&mut conn, request) {
                Ok(favorites) => (
                    "HTTP/1.1 200 Ok",
                    "application/json",
                    serde_json::to_string(&structs::FavoritesList { favorites })
                        .expect("Failed to serialize favorites"),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred while retrieving favorites"),
                ),
            };
            response
        }

        // retrieve favorite handle
        "POST /retrieveFavorite HTTP/1.1" => {
            let request: structs::Favorite =
                serde_json::from_str(body_content).expect("Invalid retrieve favorite json");
            let response = match favorite_model::get_favorite(&mut conn, request) {
                Ok(route) => (
                    "HTTP/1.1 200 Ok",
                    "application/json",
                    serde_json::to_string(&route).expect("Failed to serialize route"),
                ),
                Err(RouteNotFound) => (
                    "HTTP/1.1 400 Bad Request",
                    "text/plain",
                    String::from("Favorite route not found"),
                ),
                Err(_) => (
                    "HTTP/1.1 500 Internal Server Error",
                    "text/plain",
                    String::from("An error occurred while retrieving favorite route"),
                ),
            };
            response
        }
        */
        _ => (
            "HTTP/1.1 404 Not Found",
            "text/plain",
            "Not Found.".to_string(),
        ),
    };
    println!("{status_code}, {content_type}, {body}");

    (String::from(status_code), String::from(content_type), body)
}

fn file_retrieve(file_type: &str, filename: &str) -> String {
    println!("Retrieving: {file_type}/{filename}");
    let path = format!("{file_type}/{filename}");
    let contents = match fs::read_to_string(path) {
        Ok(str) => str,
        Err(_) => String::from("File not Found"),
    };

    contents
}
