pub mod helper;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Color {
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
}
impl Color {

    pub fn from_rgbf(red: f64, green: f64, blue: f64) -> Result<Self, String> {

        let r = red / 255.0;
        let g = green / 255.0;
        let b = blue/ 255.0;

        // Find maximum and minimum RGB values
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Calculate Value component (maximum RGB value)
        let v = max;

        // Calculate Saturation
        let s = if max != 0.0 {
            delta / max
        } else {
            0.0
        };

        // Calculate Hue
        let mut h = 0.0;
        if delta != 0.0 {
            if max == r {
                h = (g - b) / delta;
            } else if max == g {
                h = 2.0 + (b - r) / delta;
            } else {
                h = 4.0 + (r - g) / delta;
            }
            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
        }

        Ok(Color { hue:h, saturation:s, value:v })
    }
    pub fn from_rgb(red: u32, green: u32, blue: u32) -> Result<Self, String> {
        let r = red as f64 / 255.0;
        let g = green as f64 / 255.0;
        let b = blue as f64 / 255.0;

        // Find maximum and minimum RGB values
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        // Calculate Value component (maximum RGB value)
        let v = max;

        // Calculate Saturation
        let s = if max != 0.0 {
            delta / max
        } else {
            0.0
        };

        // Calculate Hue
        let mut h = 0.0;
        if delta != 0.0 {
            if max == r {
                h = (g - b) / delta;
            } else if max == g {
                h = 2.0 + (b - r) / delta;
            } else {
                h = 4.0 + (r - g) / delta;
            }
            h *= 60.0;
            if h < 0.0 {
                h += 360.0;
            }
        }

        Ok(Color { hue:h, saturation:s, value:v })
    }
    pub fn from_hex(hex: &str) -> Result<Self, String> {
        let trimmed = hex
            .strip_prefix("#")
            .unwrap_or(hex)
            .chars()
            .collect::<Vec<_>>();
        let chunks = &trimmed
            .chunks(match trimmed.len() {
                3 => 1,
                6 => 2,
                _ => return Err("Improper hex length".to_owned()),
            })
            .map(|x| x.iter().collect::<String>())
            .map(|x| {
                u32::from_str_radix(x.as_str(), 16).expect("Unable to parse hexidecimal value")
            })
            .collect::<Vec<_>>();

        Ok(Color::from_rgb(chunks[0], chunks[1], chunks[2])?)
    }

    pub fn to_rgb(&self) -> [f64; 3] {
        fn is_between(value: f64, min: f64, max: f64) -> bool {
            min <= value && value < max
        }
        let c = self.value * self.saturation;
        let h = self.hue / 60.0;
        let x = c * (1.0 - ((h % 2.0) - 1.0).abs());
        let m = self.value - c;

        let (r, g, b): (f64, f64, f64) = if is_between(h, 0.0, 1.0) {
            (c, x, 0.0)
        } else if is_between(h, 1.0, 2.0) {
            (x, c, 0.0)
        } else if is_between(h, 2.0, 3.0) {
            (0.0, c, x)
        } else if is_between(h, 3.0, 4.0) {
            (0.0, x, c)
        } else if is_between(h, 4.0, 5.0) {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        [
            ((r + m) * 255.0),
            ((g + m) * 255.0),
            ((b + m) * 255.0),
        ]
    }

    pub fn to_hex(&self) -> String {
        format!(
            "{}",
            self.to_rgb()
                .iter()
                .map(|x| format!("{:02x}", x.round() as u32))
                .collect::<String>()
        )
    }
}

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Color::from_hex(s)?)
    }
}
