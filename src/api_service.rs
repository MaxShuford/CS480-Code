use base64::{Engine as _, engine::general_purpose};
use curl::easy::Easy;
use image::load_from_memory;
use regex::Regex;
use serde::Deserialize;
use serde_json;

use std::fs::File;
use std::sync::{Arc, Mutex};

use crate::error::{Error, Result};
use crate::structs::{DirectionOptions, RouteToMap, RouteWithDirections, Waypoint};

// TODO: Real errors instead of expect then change return type to result

fn api_call(api_call_str: &str) -> Result<Vec<u8>> {
    // Arc::Mutex allows us to clone out vec and use it in a closure for the write_function
    let out = Arc::new(Mutex::new(Vec::new()));
    let out_closure = out.clone();

    let mut easy = Easy::new();
    easy.url(api_call_str).expect("Unable to build easy url");

    // api call execution
    let mut transfer = easy.transfer();
    transfer
        .write_function(|data| {
            let mut out = out_closure
                .lock()
                .expect("Unable to lock output in closure");
            out.extend_from_slice(data);
            Ok(data.len())
        })
        .expect("Failed to write api response");
    transfer.perform().expect("easy crate error");

    let result = out.lock().expect("Unable to lock output");

    Ok(result.clone())
}

pub fn geocoding(api_key: &str, city_name: &str, state_code: &str) -> Result<(f32, f32)> {
    let city_name = city_name.replace(" ", "+");

    // constuct
    let api_call_str = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={city_name},{state_code},us&limit=1&appid={api_key}"
    );

    let result_str = self::api_call(api_call_str.as_str())?;
    let result_str = String::from_utf8(result_str).expect("Did not receive valid UTF-8");

    if result_str.len() == 2 {
        return Err(Error::InvalidLocation);
    }

    // String parsing for lat
    let lat_regex = Regex::new(r#""lat":(-?\d+\.\d+)"#).expect("invalid regex pattern");
    let lat: Vec<&str> = lat_regex
        .find(result_str.as_str())
        .expect("No latitude found")
        .as_str()
        .split(':')
        .collect();
    let lat = lat[1]
        .parse::<f32>()
        .expect("Failed to parse latitude as f32");

    // String parsing for lon
    let lon_regex = Regex::new(r#""lon":(-?\d+.\d+)"#).expect("invalid regex pattern");
    let lon: Vec<&str> = lon_regex
        .find(result_str.as_str())
        .expect("No longitude found")
        .as_str()
        .split(':')
        .collect();
    let lon = lon[1]
        .parse::<f32>()
        .expect("Failed to parse longitude as f32");

    Ok((lat, lon))
}

pub fn directions(api_key: &str, locations: Vec<(f32, f32)>) -> Result<DirectionOptions> {
    // construct api call
    let mut loc_str = format!("{},{}", locations[0].1, locations[0].0);
    for i in 1..(locations.len()) {
        loc_str.push_str(&format!(";{},{}", locations[i].1, locations[i].0));
    }
    let api_call_str = format!(
        "https://api.mapbox.com/directions/v5/mapbox/driving/{loc_str}?notifications=none&alternatives=true&steps=true&access_token={api_key}"
    );

    // execute directions api call
    let result_vec = api_call(api_call_str.as_str())?;
    let result_str = String::from_utf8(result_vec).expect("Did not receive valid UTF-8");

    // parsing result json
    let result: APIResult =
        serde_json::from_str(result_str.as_str()).expect("failed parseing code");

    // construct usable route data from heavily nested json
    let mut routes: Vec<RouteWithDirections> = Vec::with_capacity(result.routes.len());
    for route in result.routes {
        let mut waypoints = Vec::new();
        for i in 0..locations.len() {
            waypoints.push(Waypoint {
                id: i as i32,
                name: "".to_string(),
                latitude: locations[i].0 as f64,
                longitude: locations[i].1 as f64,
            })
        }

        let mut directions: Vec<String> = Vec::new();
        for leg in route.legs {
            for step in leg.steps {
                directions.push(step.maneuver.instruction);
            }
        }
        routes.push(RouteWithDirections {
            waypoints: waypoints,
            directions: directions,
            geometry: route.geometry,
        });
    }

    // return DirectionsResult
    Ok(DirectionOptions {
        code: result.code,
        routes: routes,
    })
}

pub fn static_images_with_routes(routes: Vec<RouteToMap>, api_key: &str) -> Result<String> {
    // build str for paths
    let mut geo_paths = String::new();
    for i in 0..routes.len() {
        let hex_color = match i {
            0 => "+00f",
            1 => "+800",
            2 => "+f00",
            _ => "",
        };
        let temp = format!("path{hex_color}({}),", routes[i].geometry);
        geo_paths.push_str(temp.as_str());
    }

    // build str for markers
    let mut markers = String::new();
    for i in 0..routes[0].route.wp.len() - 1 {
        let label = match (routes[0].route.wp[i].id + 1).to_string().chars().next() {
            Some(ch) => ch,
            None => '0',
        };
        let temp = format!(
            "pin-s-{}+ff0({},{}),",
            label, routes[0].route.wp[i].longitude, routes[0].route.wp[i].latitude
        );
        markers.push_str(temp.as_str());
    }
    let label = match (routes[0].route.wp[routes[0].route.wp.len() - 1].id + 1)
        .to_string()
        .chars()
        .next()
    {
        Some(ch) => ch,
        None => '0',
    };
    let temp = format!(
        "pin-s-{}+ff0({},{})",
        label,
        routes[0].route.wp[routes[0].route.wp.len() - 1].longitude,
        routes[0].route.wp[routes[0].route.wp.len() - 1].latitude
    );
    markers.push_str(temp.as_str());

    let api_call_str = format!(
        "https://api.mapbox.com/styles/v1/mapbox/streets-v12/static/{geo_paths}{markers}/auto/400x400?access_token={api_key}"
    );
    let api_result = api_call(api_call_str.as_str())?;

    // NOTE: TEMP
    let img = load_from_memory(&api_result).expect("Failed to load image from mem");
    let mut file = File::create("mapbox_result.png").expect("Failed to create file");
    img.write_to(&mut file, image::ImageFormat::Png)
        .expect("Failed to write img to file");
    // NOTE: END TEMP

    Ok(general_purpose::STANDARD.encode(&api_result))
}

pub fn static_images_with_user_loc(location: (f32, f32), api_key: &str) -> Result<String> {
    let lon = location.1;
    let lat = location.0;
    let api_call_str = format!(
        "https://api.mapbox.com/styles/v1/mapbox/streets-v12/static/{lon},{lat},2/400x400?access_token={api_key}"
    );

    let api_result = api_call(api_call_str.as_str())?;

    // NOTE: TEMP
    let img = load_from_memory(&api_result).expect("Failed to load image from mem");
    let mut file = File::create("mapbox_result.png").expect("Failed to create file");
    img.write_to(&mut file, image::ImageFormat::Png)
        .expect("Failed to write img to file");
    // NOTE: END TEMP

    Ok(general_purpose::STANDARD.encode(&api_result))
}

// structs for deserialing response from
#[derive(Deserialize)]
struct APIResult {
    code: String,
    routes: Vec<Route>,
}
#[derive(Deserialize)]
struct Route {
    geometry: String,
    legs: Vec<Leg>,
}
#[derive(Deserialize)]
struct Leg {
    steps: Vec<Step>,
}
#[derive(Deserialize)]
struct Step {
    maneuver: Instruction,
}
#[derive(Deserialize)]
struct Instruction {
    instruction: String,
}
