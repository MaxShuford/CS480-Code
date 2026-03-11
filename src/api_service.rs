use curl::easy::Easy;
use regex::Regex;
use serde::Deserialize;
use serde_json;
use std::sync::{Arc, Mutex};

// TODO: Real errors instead of expect then change return type to result

fn api_call(api_call_str: &str) -> String {
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

    String::from_utf8(result.clone()).expect("Did not receive valid UTF-8")
}

pub fn geocoding(api_key: &str, city_name: &str, state_code: &str) -> (f32, f32) {
    // TODO: Error checking on api key, city name, and state code lengths (must be < 1) this might
    // be done in controller?

    // constuct
    let api_call_str = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={city_name},{state_code},us&limit=1&appid={api_key}"
    );

    let result_str = self::api_call(api_call_str.as_str());

    if result_str.len() == 2 {
        println!("Invalid location")
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

    (lat, lon)
}

pub fn directions(api_key: &str, locations: Vec<(f32, f32)>) -> DirectionOptions {
    // construct api call
    let mut loc_str = format!("{},{}", locations[0].1, locations[0].0);
    for i in 1..(locations.len()) {
        loc_str.push_str(&format!(";{},{}", locations[i].1, locations[i].0));
    }
    let api_call_str = format!(
        "https://api.mapbox.com/directions/v5/mapbox/driving/{loc_str}?notifications=none&alternatives=true&steps=true&access_token={api_key}"
    );

    // execute directions api call
    let result_str = api_call(api_call_str.as_str());

    // parsing result json
    let result: APIResult =
        serde_json::from_str(result_str.as_str()).expect("failed parseing code");

    // construct usable route data from heavily nested json
    let mut routes: Vec<RouteWithDirections> = Vec::with_capacity(result.routes.len());
    for route in result.routes {
        let waypoints = locations.clone();
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
    DirectionOptions {
        code: result.code,
        routes: routes,
    }
}

pub struct DirectionOptions {
    pub code: String,
    pub routes: Vec<RouteWithDirections>,
}

pub struct RouteWithDirections {
    pub waypoints: Vec<(f32, f32)>,
    pub directions: Vec<String>,
    pub geometry: String,
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
