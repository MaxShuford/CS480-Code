// Route constants – all METHOD/path strings 

// === Page requests (GET) ===
// Log In
pub const GET_LOGIN_PAGE: &str = "GET/Login.html";
// Create Account
pub const GET_CREATE_ACCOUNT_PAGE: &str = "GET/CreateAccount.html";
// Change Password
pub const GET_CHANGE_PASSWORD_PAGE: &str = "GET/ChangePassword.html";
// Select Waypoints 
pub const GET_SELECT_WAYPOINTS_PAGE: &str = "GET/SelectWaypoints.html";
// Select Route
pub const GET_SELECT_ROUTE_PAGE: &str = "GET/SelectRoute.html";
// View Route
pub const GET_VIEW_ROUTE_PAGE: &str = "GET/ViewRoute.html";

// === API endpoints (POST) ===
// Log In
pub const POST_LOGIN: &str = "POST/login";
// Create Account
pub const POST_CREATE_ACCOUNT: &str = "POST/createAccount";
// Change Password
pub const POST_CHANGE_PASSWORD: &str = "POST/changePassword";
// Add to Favorites
pub const POST_ADD_FAVORITE: &str = "POST/addFavorite";
// Remove from Favorites
pub const POST_DELETE_FAVORITE: &str = "POST/deleteFavorite";
// Retrieve Favorites list
pub const POST_RETRIEVE_FAVORITES: &str = "POST/retrieveFavorites";
// Retrieve a single Favorite
pub const POST_RETRIEVE_FAVORITE: &str = "POST/retrieveFavorite";