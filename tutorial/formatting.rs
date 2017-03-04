use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // lattitude
    lat: f32,
    // longitude
    long: f32
}

impl Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.long >= 0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}{} {:.3}{}", self.name, self.lat.abs(),
               lat_c, self.long.abs(), lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Display for Color {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Activity
        write!(f, "RGB ({}, {}, {}) {}{}{}", self.red, self.green, self.blue,
                format!("0X{:X}", self.red),
                format!("{:X}", self.green),
                format!("{:X}", self.blue)
        )
    }

}

fn main() {
    for city in [
        City {name: "Helsinki", lat: 53.343534, long: -5.4654476},
        City {name: "Espoo", lat: 34.56444765, long: -3.767567}
    ].iter() {
        println!("{}", city);
    }

    for color in [
        Color {red: 128, green: 255, blue: 90},
        Color {red: 0, green: 3, blue: 254}
    ].iter() {
        println!("{}", color);
    }

}