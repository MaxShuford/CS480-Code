mod api_service;
mod error;
mod routes;
pub mod structs;

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

    let match_key = format!("{} {}", method, prefix); // ex: "GET /html"
    Some((match_key, rest.to_string()))
}

// === API Handlers ===
fn handle_login() {
    println!("POST /login");
    // TODO: parse {username, password}, return {uuid}
}

fn handle_create_account() {
    println!("POST /createAccount");
    // TODO: parse {username, password, confirmPassword}, return {error_code}
}

fn handle_change_password() {
    println!("POST /changePassword");
    // TODO: parse {uuid, oldPassword, newPassword, confirmPassword}, return {error_code}
}

fn handle_add_favorite() {
    println!("POST /addFavorite");
    // TODO: parse {uuid, route}, return {error_code}
}

fn handle_delete_favorite() {
    println!("POST /deleteFavorite");
    // TODO: parse {uuid, route_id}, return {error_code}
}

fn handle_retrieve_favorites() {
    println!("POST /retrieveFavorites");
    // TODO: parse {uuid}, return {route_id: [], names: []}
}

fn handle_retrieve_favorite() {
    println!("POST /retrieveFavorite");
    // TODO: parse {uuid, route_id}, return {route: Route}
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
        routes::POST_LOGIN => handle_login(),
        routes::POST_CREATE_ACCOUNT => handle_create_account(),
        routes::POST_CHANGE_PASSWORD => handle_change_password(),
        routes::POST_ADD_FAVORITE => handle_add_favorite(),
        routes::POST_DELETE_FAVORITE => handle_delete_favorite(),
        routes::POST_RETRIEVE_FAVORITES => handle_retrieve_favorites(),
        routes::POST_RETRIEVE_FAVORITE => handle_retrieve_favorite(),

        // Static assets (JS, CSS)
        _ if method == "GET" && (path.starts_with("/js/") || path.starts_with("/css/")) => {
            serve_static_file(path)
        }

        // Fallback
        _ => handle_404(),
    }
}

fn main() {
    route_request("GET", "/html/Login.html");
    route_request("POST", "/login");
    route_request("GET", "/html/SelectWaypoints.html");
    route_request("GET", "/js/main.js");
    route_request("GET", "/css/header.css");
    route_request("POST", "/retrieveFavorites");
    route_request("GET", "/nonexistent");

    let raw = "GET /html/Login.html HTTP/1.1";
    if let Some(file) = extract_file_name(raw) {
        println!("Extracted file name: {}", file);
    }
}
