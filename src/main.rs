mod routes;

// === Page Handlers ===
fn serve_login_page() {
    println!("Serving Login.html");
    // TODO: read file and return HTTP response
}

fn serve_create_account_page() {
    println!("Serving CreateAccount.html");
}

fn serve_change_password_page() {
    println!("Serving ChangePassword.html");
}

fn serve_select_waypoints_page() {
    println!("Serving SelectWaypoints.html");
}

fn serve_select_route_page() {
    println!("Serving SelectRoute.html");
}

fn serve_view_route_page() {
    println!("Serving ViewRoute.html");
}

// === API Handlers ===
fn handle_login() {
    println!("POST /login");
    // TODO: parse {username, password}, return {uuid}
}

fn handle_create_account() {
    println!("POST /createAccount");
    // TODO: parse {username, password, confirmPasword}, return {error_code}
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
    // Combine method and path into a single string for matching
    let route = format!("{}{}", method, path);

    match route.as_str() {
        // Page requests
        routes::GET_LOGIN_PAGE => serve_login_page(),
        routes::GET_CREATE_ACCOUNT_PAGE => serve_create_account_page(),
        routes::GET_CHANGE_PASSWORD_PAGE => serve_change_password_page(),
        routes::GET_SELECT_WAYPOINTS_PAGE => serve_select_waypoints_page(),
        routes::GET_SELECT_ROUTE_PAGE => serve_select_route_page(),
        routes::GET_VIEW_ROUTE_PAGE => serve_view_route_page(),

        // API endpoints
        routes::POST_LOGIN => handle_login(),
        routes::POST_CREATE_ACCOUNT => handle_create_account(),
        routes::POST_CHANGE_PASSWORD => handle_change_password(),
        routes::POST_ADD_FAVORITE => handle_add_favorite(),
        routes::POST_DELETE_FAVORITE => handle_delete_favorite(),
        routes::POST_RETRIEVE_FAVORITES => handle_retrieve_favorites(),
        routes::POST_RETRIEVE_FAVORITE => handle_retrieve_favorite(),

        // Fallback
        _ => handle_404(),
    }
}
fn main() {
    // Testing some routes
    route_request("GET", "/Login.html");
    route_request("POST", "/login");
    route_request("GET", "/SelectWaypoints.html");
    route_request("GET", "/SelectRoute.html");
    route_request("POST", "/retrieveFavorites");
    route_request("GET", "/nonexistent");
}