use std::process;
use std::iter;
use std::{fmt::Write, num::ParseIntError};
use clap::Parser;
use clap::ArgEnum;
use regex::Regex;


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
#[clap(arg_enum)]
enum Output {
    Hex,
    Rgb,
    Percent,
}
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn to_hex(&self) -> String{
        let mut res = "#".to_string();

        res.push_str(&encode_hex(&[self.r, self.g, self.b, self.a]));
        return res;
    }

    fn from_hex(input: &str) -> Color{
        let hex = str::replace(input, "#", "");
        let hex = hex.trim();
            let width =  hex.chars().count();
            match width {
                3 => {
                    let parts = decode_hex(&pad_hex(&hex)).expect("Failed hex decode");
                    return Color {
                        r: parts[0],
                        g: parts[1],
                        b: parts[2],
                        a: 255,
                    }
                }
                4 => {
                    let parts = decode_hex(&pad_hex(&hex)).expect("Failed hex decode");
                    return Color {
                        r: parts[0],
                        g: parts[1],
                        b: parts[2],
                        a: parts[3],
                    }
                }
                6 => {
                    let parts = decode_hex(&hex).expect("Failed hex decode");
                    return Color {
                        r: parts[0],
                        g: parts[1],
                        b: parts[2],
                        a: 255,
                    }
                },
                8 => {
                    let parts = decode_hex(&hex).expect("Failed hex decode");
                    return Color {
                        r: parts[0],
                        g: parts[1],
                        b: parts[2],
                        a: parts[3],
                    }
                },
                _ => {
                    // Todo Raise exception
                    println!("Must provide valid hex length, was: {}", width);
                    process::exit(1);
                }
            }
    }

    fn to_rgba(&self) -> String{
        let mut res = "rgba(".to_string();
        res.push_str(&self.r.to_string());
        res.push_str(", ");
        res.push_str(&self.g.to_string());
        res.push_str(", ");
        res.push_str(&self.b.to_string());
        res.push_str(", ");
        let a = u8_to_decimal(self.a);
        res.push_str(&a.to_string());
        res.push(')');
        return res;
    }

    fn from_rgb(input: &str) -> Color{
        let remover_re = Regex::new(r"rgba?\(").unwrap();
        let rgb = remover_re.replace(input, "").replace(")","");
        let rgb = rgb.trim();
        let rgb: Vec<&str> = rgb.split(",").collect();
        let len = rgb.len();
        let r = if len > 0 {rgb[0].trim().parse().expect("Invalid r number")} else { 0 };
        let g = if len > 1 {rgb[1].trim().parse().expect("Invalid g number")} else { 0 };
        let b = if len > 2 {rgb[2].trim().parse().expect("Invalid b number")} else { 0 };
        let a = if len > 3 {decimal_to_u8(rgb[3].trim().parse::<f64>().expect("Invalid a number"))} else { 255 };
        return Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    fn to_percent(&self) -> String{
        let mut res = "% ".to_string();
        let r = u8_to_decimal(self.r);
        let g = u8_to_decimal(self.g);
        let b = u8_to_decimal(self.b);
        let a = u8_to_decimal(self.a);
        res.push_str(&r.to_string());
        res.push_str(", ");
        res.push_str(&g.to_string());
        res.push_str(", ");
        res.push_str(&b.to_string());
        res.push_str(", ");
        res.push_str(&a.to_string());
        return res;
    }

    fn from_percent(input: &str) -> Color {
        let percent = str::replace(input, "%", "");
        let percent = percent.trim();
        let percent: Vec<&str> = percent.split(",").collect();
        let len = percent.len();
        let r = if len > 3 {decimal_to_u8(percent[0].trim().parse::<f64>().expect("Invalid a number"))} else { 0 };
        let g = if len > 3 {decimal_to_u8(percent[1].trim().parse::<f64>().expect("Invalid a number"))} else { 0 };
        let b = if len > 3 {decimal_to_u8(percent[2].trim().parse::<f64>().expect("Invalid a number"))} else { 0 };
        let a = if len > 3 {decimal_to_u8(percent[3].trim().parse::<f64>().expect("Invalid a number"))} else { 255 };

        return Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }
}

fn u8_to_decimal(i: u8) -> f64 {
    let n = f64::from(i);
    let n = n / 255.0;
    let n = (n * 1000.0).round() / 1000.0;
    return n;
}

fn decimal_to_u8(i: f64) -> u8 {
    return (i * 255.0).round() as u8;
    
}

#[derive(Parser, Debug)]
#[clap(author = "Author: John", version = "1.0.0", about = "Transforms colors into other formats", long_about = None)]
struct Args {
    #[clap(short, long, arg_enum, help = "The transformed output format.", default_value_t = Output::Hex)]
    output: Output,

    #[clap(help = "The input, can be a hex \"#123456\", \"rgb(1,2,3)\" or \"%0.1,0.2,0.3\"")]
    input: String,
}


fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

fn pad_hex(s: &str) -> String {
    return s
        .chars()
        .flat_map(|c| iter::repeat(c).take(2))
        .collect::<String>();
}



fn parse_color(input: &str) -> Color{

    let hex_matcher = Regex::new(r"^#[0-9a-fA-F]{3,8}$").unwrap();
    let rgb_matcher = Regex::new(r"^rgba?\(\d{1,3},\s?\d{1,3},\s?\d{1,3}(,\s?(1|0?\.?\d*))?\)$").unwrap();
    let percent_matcher = Regex::new(r"^%\s?0?\.?\d+,\s?0?\.?\d+,\s?0?\.?\d+(,\s?0?\.?\d+)?$").unwrap();

    if hex_matcher.is_match(input){
        return Color::from_hex(&input);

    } else if rgb_matcher.is_match(input){
        return Color::from_rgb(&input);

    } else if percent_matcher.is_match(input){
        return Color::from_percent(&input);
    } else {
        // Todo Raise exception
        println!("Could not match an input type. Use either #000000, rgb(0,0,0) or %0,0,0");
        process::exit(1);

    }

}

fn transform_color(output: Output, color: Color) -> String {
    match output{
        Output::Hex => {
            return color.to_hex();
        },
        Output::Rgb => {
            return color.to_rgba();
        }
        Output::Percent => {
            return color.to_percent();
        }
    }
}


fn main() {
    

    let args = Args::parse();

    let output = args.output;
    let input = args.input;

    let color = parse_color(&input);
    let out = transform_color(output, color);
    println!("Input: {}", input);
    println!("Target: {:?}", output);
    println!("Output: {}", out);
}