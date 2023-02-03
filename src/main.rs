use std::fmt::Display;
use std::io::{self, BufRead};

use atty::Stream;
use color_eyre::eyre::eyre;
use color_eyre::Result;

fn main() -> Result<()> {
    if atty::is(Stream::Stdin) {
        return Err(eyre!("input is a terminal, please pipe data via stdin!"));
    }

    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let mut rgb_min_r = 255;
    let mut rgb_max_r = 0;
    let mut rgb_min_g = 255;
    let mut rgb_max_g = 0;
    let mut rgb_min_b = 255;
    let mut rgb_max_b = 0;

    let mut hsv_min_h = 360;
    let mut hsv_max_h = 0;
    let mut hsv_min_s = 255;
    let mut hsv_max_s = 0;
    let mut hsv_min_v = 255;
    let mut hsv_max_v = 0;

    let mut rgb_min_luminance = 1.0;
    let mut rgb_max_luminance = 0.0;
    let mut hsv_min_luminance = 1.0;
    let mut hsv_max_luminance = 0.0;

    for line in lines {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !line.starts_with('#') {
            eprintln!("warning: skipping line '{line}' because it doesn't start with `#`");
            continue;
        }
        let rgb: Rgb = line.into();
        let hsv: Hsv = {
            let tmp: Rgb = line.into();
            tmp.into()
        };

        println!(
            "{}: {} luminance_rgb({}) luminance_hsv({})",
            rgb,
            hsv,
            rgb.luminance(),
            hsv.luminance()
        );

        if rgb.r < rgb_min_r {
            rgb_min_r = rgb.r;
        }
        if rgb.r > rgb_max_r {
            rgb_max_r = rgb.r;
        }
        if rgb.g < rgb_min_g {
            rgb_min_g = rgb.g;
        }
        if rgb.g > rgb_max_g {
            rgb_max_g = rgb.g;
        }
        if rgb.b < rgb_min_b {
            rgb_min_b = rgb.b;
        }
        if rgb.b > rgb_max_b {
            rgb_max_b = rgb.b;
        }

        if hsv.h < hsv_min_h {
            hsv_min_h = hsv.h;
        }
        if hsv.h > hsv_max_h {
            hsv_max_h = hsv.h;
        }
        if hsv.s < hsv_min_s {
            hsv_min_s = hsv.s;
        }
        if hsv.s > hsv_max_s {
            hsv_max_s = hsv.s;
        }
        if hsv.v < hsv_min_v {
            hsv_min_v = hsv.v;
        }
        if hsv.v > hsv_max_v {
            hsv_max_v = hsv.v;
        }

        if rgb.luminance() < rgb_min_luminance {
            rgb_min_luminance = rgb.luminance();
        }
        if rgb.luminance() > rgb_max_luminance {
            rgb_max_luminance = rgb.luminance();
        }
        if hsv.luminance() < hsv_min_luminance {
            hsv_min_luminance = hsv.luminance();
        }
        if hsv.luminance() > hsv_max_luminance {
            hsv_max_luminance = hsv.luminance();
        }
    }

    println!(
        "rgb ranges: r({}..{}) g({}..{}) b({}..{})",
        rgb_min_r, rgb_max_r, rgb_min_g, rgb_max_g, rgb_min_b, rgb_max_b
    );
    println!(
        "hsv ranges: h({}..{}) s({}..{}) v({}..{})",
        hsv_min_h, hsv_max_h, hsv_min_s, hsv_max_s, hsv_min_v, hsv_max_v
    );
    println!(
        "luminance ranges: rgb({}..{}) hsv({}..{})",
        rgb_min_luminance, rgb_max_luminance, hsv_min_luminance, hsv_max_luminance
    );

    Ok(())
}

trait Luminance {
    fn luminance(&self) -> f32;
}

struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl From<Hsv> for Rgb {
    fn from(hsv: Hsv) -> Self {
        let h = hsv.h as f32;
        let s = hsv.s as f32 / 255.0;
        let v = hsv.v as f32 / 255.0;

        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;

        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        Rgb {
            r: ((r + m) * 255.0) as u8,
            g: ((g + m) * 255.0) as u8,
            b: ((b + m) * 255.0) as u8,
        }
    }
}

impl From<String> for Rgb {
    fn from(s: String) -> Self {
        let mut chars = s.chars();

        // Skip the leading '#'
        chars.next();

        let r = u8::from_str_radix(&chars.by_ref().take(2).collect::<String>(), 16).unwrap();
        let g = u8::from_str_radix(&chars.by_ref().take(2).collect::<String>(), 16).unwrap();
        let b = u8::from_str_radix(&chars.by_ref().take(2).collect::<String>(), 16).unwrap();

        Rgb { r, g, b }
    }
}

impl<'a> From<&'a str> for Rgb {
    fn from(s: &'a str) -> Self {
        let mut chars = s.chars();

        // Skip the leading '#'
        chars.next();

        let r = u8::from_str_radix(&chars.by_ref().take(2).collect::<String>(), 16).unwrap();
        let g = u8::from_str_radix(&chars.by_ref().take(2).collect::<String>(), 16).unwrap();
        let b = u8::from_str_radix(&chars.by_ref().take(2).collect::<String>(), 16).unwrap();

        Rgb { r, g, b }
    }
}

impl Luminance for Rgb {
    fn luminance(&self) -> f32 {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

struct Hsv {
    // Hue: 0-360
    h: u16,
    // Saturation: 0-255
    s: u8,
    // Value: 0-255
    v: u8,
}

impl From<Rgb> for Hsv {
    fn from(rgb: Rgb) -> Self {
        let r = rgb.r as f32 / 255.0;
        let g = rgb.g as f32 / 255.0;
        let b = rgb.b as f32 / 255.0;

        let max = r.max(g.max(b));
        let min = r.min(g.min(b));

        let h = if max == min {
            0.0
        } else if max == r {
            60.0 * ((g - b) / (max - min) + (if g < b { 6.0 } else { 0.0 }))
        } else if max == g {
            60.0 * ((b - r) / (max - min) + 2.0)
        } else {
            60.0 * ((r - g) / (max - min) + 4.0)
        };

        let s = if max == 0.0 { 0.0 } else { 1.0 - min / max };

        Hsv {
            h: h as u16,
            s: (s * 255.0) as u8,
            v: (max * 255.0) as u8,
        }
    }
}

impl Luminance for Hsv {
    fn luminance(&self) -> f32 {
        let s = self.s as f32 / 255.0;
        let v = self.v as f32 / 255.0;

        v * (1.0 - s / 2.0)
    }
}

impl Display for Hsv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsv({}, {}, {})", self.h, self.s, self.v)
    }
}
