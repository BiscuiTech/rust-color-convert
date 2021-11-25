use colored::*;
use hex;
use std::env;
use std::f64;
use std::u8;
fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    let arg = &args[1];
    let hex = match arg.starts_with("#") {
        true => HexColor::from_hex(&arg[1..]),
        false => HexColor::from_hex(arg),
    };
    println!("This color is composed of: {:?}", hex);
    println!(
        "{}",
        "                      ".on_truecolor(hex.blue, hex.green, hex.red)
    );
    println!("HEX | #{}", hex.to_string());
    println!("RGB | rgb{:?}", hex.to_rgb());
    println!("HSL | hsl{:?}", hex.to_hsl());
}

#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}
impl HexColor {
    fn new(red: u8, green: u8, blue: u8) -> HexColor {
        HexColor { red, green, blue }
    }

    fn red(&self) -> u8 {
        self.red
    }

    fn green(&self) -> u8 {
        self.green
    }

    fn blue(&self) -> u8 {
        self.blue
    }

    fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    fn from_hex(hex: &str) -> HexColor {
        let decoded = hex::decode(hex).unwrap();
        let red = decoded[0];
        let green = decoded[1];
        let blue = decoded[2];
        HexColor::new(red, green, blue)
    }

    fn from_rgb(rgb: &str) -> HexColor {
        let red = rgb[1..3].parse::<u8>().unwrap();
        let green = rgb[3..5].parse::<u8>().unwrap();
        let blue = rgb[5..7].parse::<u8>().unwrap();
        HexColor::new(red, green, blue)
    }

    fn to_rgb(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    fn to_rgba(&self) -> (u8, u8, u8, u8) {
        (self.red, self.green, self.blue, u8::MAX)
    }

    fn from_rgba(rgba: &str) -> HexColor {
        let red = rgba[1..3].parse::<u8>().unwrap();
        let green = rgba[3..5].parse::<u8>().unwrap();
        let blue = rgba[5..7].parse::<u8>().unwrap();
        HexColor::new(red, green, blue)
    }

    fn from_hsl(hsl: &str) -> HexColor {
        let red = hsl[1..3].parse::<u8>().unwrap();
        let green = hsl[3..5].parse::<u8>().unwrap();
        let blue = hsl[5..7].parse::<u8>().unwrap();
        HexColor::new(red, green, blue)
    }

    fn from_hsla(hsla: &str) -> HexColor {
        let red = hsla[1..3].parse::<u8>().unwrap();
        let green = hsla[3..5].parse::<u8>().unwrap();
        let blue = hsla[5..7].parse::<u8>().unwrap();
        HexColor::new(red, green, blue)
    }

    fn to_hsl(&self) -> (u8, u8, u8) {
        let red = f64::from(self.red) / 255.0;
        let green = f64::from(self.green) / 255.0;
        let blue = f64::from(self.blue) / 255.0;
        let c_max = f64::max(f64::max(red, green), blue);
        let c_min = f64::min(f64::min(red, green), blue);
        let delta = c_max - c_min;
        let h = match delta {
            0.0 => 0.0,
            _ => {
                let mut h = match c_max {
                    red => (green - blue) / delta,
                    green => (blue - red) / delta + 2.0,
                    blue => (red - green) / delta + 4.0,
                    _ => 0.0,
                };
                h = h * 60.0;
                if h < 0.0 {
                    h += 360.0;
                }
                h
            }
        };
        let s = match delta {
            0.0 => 0.0,
            _ => (delta / c_max) * 100.0,
        };
        let l = (c_max + c_min) / 2.0 * 100.0;
        (h as u8, s as u8, l as u8)
    }

    // fn to_hsla(&self) -> (u8, u8, u8, u8) {
    //     let red = f64::from(self.red) / 255.0;
    //     let green = f64::from(self.green) / 255.0;
    //     let blue = f64::from(self.blue) / 255.0;
    //     let c_max = f64::max(f64::max(red, green), blue);
    //     let c_min = f64::min(f64::min(red, green), blue);
    //     let delta = c_max - c_min;
    //     let h = match delta {
    //         0.0 => 0.0,
    //         _ => {
    //             let mut h = match c_max {
    //                 red => (green - blue) / delta,
    //                 green => (blue - red) / delta + 2.0,
    //                 blue => (red - green) / delta + 4.0,
    //                 _ => 0.0,
    //             };
    //             h = h * 60.0;
    //             if h < 0.0 {
    //                 h += 360.0;
    //             }
    //             h
    //         }
    //     };
    //     let s = match delta {
    //         0.0 => 0.0,
    //         _ => (delta / c_max) * 100.0,
    //     };
    //     let l = (c_max + c_min) / 2.0 * 100.0;
    //     (h as u8, s as u8, l as u8)
    // }

    fn to_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}
