mod api_service;
mod error;
mod routes;
pub mod structs;

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
    Some((match_key, rest.to_string()))
}

// === API Handlers ===
/*
fn handle_login(request_body: &str) -> String {
    println!("POST /login");

    let login: LoginRequest = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    let response = match DB::login(login) {
        Ok(uuid) => format!("uuid: {}", uuid),
        Err(e) => format!("Err processing request: {}", e),
    };

    response
}

fn handle_create_account(request_body: &str) -> String {
    println!("POST /createAccount");

    let create: User = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    let response = match DB::create_acct(create) {
        Ok(code) => format!("Success: {}", code),
        Err(e) => format!("Err processing request: {}", e),
    };

    response
}

fn handle_change_password(request_body: &str) -> String {
    println!("POST /changePassword");

    let hcp: ChangePassword = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    let response = match DB::change_password(hcp) {
        Ok(code) => formats!("Success: {}", code),
        Err(e) => format!("Err processing request: {}", e),
    };

    response
    // TODO: parse {uuid, oldPassword, newPassword, confirmPassword}, return {error_code}
}

fn handle_add_favorite(request_body: &str) -> String {
    println!("POST /addFavorite");

    let addfave: AddFavorite = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    let response = match DB::add_favorite(addfave) {
        Ok(code) => formats!("Success: {}", code),
        Err(e) => format!("Err processing request: {}", e),
    };

    response
    // TODO: parse {uuid, route}, return {error_code}
}

fn handle_delete_favorite(request_body: &str) -> String {
    println!("POST /deleteFavorite");

    let delfave: DeleteFavorite = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    let response = match DB::delete_favorite(delfave) {
        Ok(code) => formats!("Success: {}", code),
        Err(e) => format!("Err processing request: {}", e),
    };

    response
    // TODO: parse {uuid, route_id}, return {error_code}
}

fn handle_retrieve_favorites(request_body: &str) -> String {
    println!("POST /retrieveFavorites");

    let retfave: RetrieveFavorites = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    match DB::retrieve_favorite(retfave) {
        Ok(favorites) => {
            let route_ids: Vec<i32> = favorites.iter().map(|f| f.route_id).collect();
            let names: Vec<String> = favorites.iter().map(|f| f.name.clone()).collect();

            format!("{{\"route_id\": {:?}, \"names\": {:?}}}", route_ids, names)
        }
        Err(e) => format!("Err processing request: {}", e),
    }
    // TODO: parse {uuid}, return {route_id: [], names: []}
}

fn handle_retrieve_favorite(request_body: &str) -> String {
    println!("POST /retrieveFavorite");

    let retfave: RetrieveFavorites = match serde_json::from_str(request_body) {
        Ok(data) => data,
        Err(e) => {
            return format!("Error parsing request: {}", e);
        }
    };

    let response = match DB::retrieve_favorite(retfave) {
        Ok(route) => format!("Success: {:?}", route), // return route
        Err(e) => format!("Err processing request: {}", e),
    };
    // TODO: parse {uuid, route_id}, return {route: Route}
}
*/

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
    // TODO: read api keys from a config file

    // listen to local hosted
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_stream(stream);
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

fn handle_stream(mut stream: TcpStream) {
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
        handle_request(request_line.as_str(), body_content.as_str());

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

fn handle_request(request_line: &str, body_content: &str) -> (String, String, String) {
    let (request_code, file_name) = match extract_file_name(request_line) {
        Some((file_type, file_name)) => (format!("GET /{file_type}"), file_name),
        None => (String::from(request_line), String::from("")),
    };

    let (status_code, content_type, body) = match request_code.as_str() {
        GET_LANDING_PAGE => {
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
        GET_HTML_PAGE => {
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
        GET_JS_PAGE => {
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
        GET_CSS_PAGE => {
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
        // TODO: geocoding api handle
        // TODO: directions api handle
        // TODO: static map with routes handle
        // TODO: static map with user location handle
        // TODO: login handle
        // TODO: create account handle
        // TODO: change password handle
        // TODO: add favorite handle
        // TODO: delete favorite handle
        // TODO: retrieve favorites handle
        // TODO: retrieve favorite handle
        _ => (
            "HTTP/1.1 404 Not Found",
            "text/plain",
            "Not Found.".to_string(),
        ),
    };

    (String::from(status_code), String::from(content_type), body)
}

fn file_retrieve(file_type: &str, filename: &str) -> String {
    let path = format!("{file_type}/{filename}");
    let contents = match fs::read_to_string(path) {
        Ok(str) => str,
        Err(_) => String::from("File not Found"),
    };

    contents
}
