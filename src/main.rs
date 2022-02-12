use colour::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase", serialize = "snake_case"))]
struct Planet {
    name: String,
    hours_in_day: u32,
    earth_days_a_year: u32,
    no_of_moons: u32,
    temp_c: i32,
}

fn get_planet_data(url: &str) -> Result<Vec<Planet>, Box<dyn Error>> {
    let res = ureq::get(url).call()?.into_string()?;
    let planets: Vec<Planet> = serde_json::from_str(&res)?;
    Ok(planets)
}

fn render_planets(planets: &Vec<Planet>) {
    for planet in planets {
        dark_green!("{}\n", planet.name);
        key_value("Hours in a day", planet.hours_in_day);
        key_value("Earth days a year", planet.earth_days_a_year);
        key_value("Number of moons", planet.no_of_moons);
        key_value("Temperature (in Celsius)", planet.temp_c);
        println!("\n")
    }
}

fn key_value<T: std::fmt::Display>(k: &str, v: T) {
    cyan!("{}: ", k);
    dark_yellow!("{}\n", v);
}

fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://planet5.herokuapp.com/all";
    let planets = get_planet_data(url)?;
    render_planets(&planets);
    Ok(())
}
