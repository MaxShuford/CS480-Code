use crate::error::AppResult;
use crate::error::Error::*;
use crate::structs::*;
use mysql::prelude::*;
use mysql::*;

//function to add favorite route to database
pub fn add_fav(conn: &mut PooledConn, input: AddFavorite) -> AppResult<i32> {
    // count how many routes the user has
    let num_routes: Option<u32> = conn.exec_first(
        "SELECT COUNT(*) FROM route WHERE user_id = :uid;",
        params! {
            "uid" => input.uuid
        },
    )?;

    let count = num_routes.unwrap_or(0);

    // limit to 5 favorites
    if count > 4 {
        return Err(MaxRoutesExceeded);
    }

    // insert new route
    conn.exec_drop(
        "INSERT INTO route (user_id, name) VALUES (:uid, :name);",
        params! {
            "uid" => input.uuid,
            "name" => "Favorite Route"
        },
    )?;

    // get new route id
    let route_id = conn.last_insert_id() as i32;

    println!("Created route {} for user {}", route_id, input.uuid);

    // insert waypoints
    for wp in &input.wp {
        println!("Inserting waypoint {:?}", wp);

        conn.exec_drop(
            "INSERT INTO location (location_id, route_id, name, latitude, longitude)
             VALUES (:lid, :rid, :name, :lat, :lon);",
            params! {
                "lid" => wp.id,
                "rid" => route_id,
                "name" => &wp.name,
                "lat" => wp.latitude,
                "lon" => wp.longitude
            },
        )?;
    }

    Ok(route_id)
}

//function to delete favorite from database
pub fn delete_fav(conn: &mut PooledConn, input: DeleteFavorite) -> AppResult<i32> {
    //delete waypoints
    conn.exec_drop(
        "DELETE FROM location 
        WHERE route_id = :rid;",
        params! {
            "rid" => &input.route_id
        },
    )?;

    //delete from route table
    conn.exec_drop(
        "DELETE FROM route 
        WHERE user_id = :uid
        AND route_id = :rid;",
        params! {
            "uid" => &input.uuid,
            "rid" => &input.route_id
        },
    )?;

    //check that the route was deleted
    if conn.affected_rows() > 0 {
        Ok(1)
    } else {
        Err(DeleteUnsuccessful)
    }
}

//function to get user's favorite routes
pub fn get_favorites(
    conn: &mut PooledConn,
    input: RetrieveFavorites,
) -> AppResult<Vec<FavoriteReturn>> {
    //query db for user's favorites
    let favs: Vec<FavoriteReturn> = conn.exec_map(
        //crazy select statement to get route id and start and end location names
        "SELECT route_id,
            (SELECT `name` FROM location 
            WHERE route_id = r.route_id 
            ORDER BY location_id ASC LIMIT 1) AS start_name,
            (SELECT `name` FROM location 
            WHERE route_id = r.route_id 
            ORDER BY location_id DESC LIMIT 1) AS end_name 
        FROM route r
        WHERE r.user_id = :uuid;",
        params! {
            "uuid" => &input.uuid
        },
        //map each row to a FavoriteReturn struct
        |(route_id, start_name, end_name): (i32, Option<String>, Option<String>)| {
            //unpack options, if none, set to "Unknown"
            let start = start_name.unwrap_or_else(|| String::from("Unknown"));
            let end = end_name.unwrap_or_else(|| String::from("Unknown"));

            //create FavoriteReturn struct
            FavoriteReturn {
                name: format!("{}-{}", start, end),
                route_id,
            }
        },
    )?;

    //return favorites
    Ok(favs)
}

//function to get a route from the user's favorites
pub fn get_favorite(conn: &mut PooledConn, input: Favorite) -> AppResult<Route> {
    //try to retrieve the route from the database
    let route_exists: Option<i32> = conn.exec_first(
        "SELECT route_id 
        FROM route 
        WHERE user_id = :uid 
        AND route_id = :rid;",
        params! {
            "uid" => &input.uuid,
            "rid" => &input.route_id
        },
    )?;

    //if the route doesn't exist, return an error
    if route_exists.is_none() {
        return Err(RouteNotFound);
    }

    //query db for waypoints of route
    let wp: Vec<Waypoint> = conn.exec_map(
        "SELECT location_id, name, latitude, longitude
        FROM location 
        WHERE route_id = :rid;",
        params! {
            "rid" => &input.route_id
        },
        //map each row to a Waypoint struct
        |(id, name, latitude, longitude)| Waypoint {
            id,
            name,
            latitude,
            longitude,
        },
    )?;

    //create route struct
    let route = Route {
        route_id: input.route_id,
        wp: wp,
    };

    //return route
    Ok(route)
}
