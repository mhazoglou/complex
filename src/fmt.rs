use std::any::type_name;
use std::{fmt, str::FromStr};
use std::num::ParseFloatError;
use regex::Regex;
use crate::*;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (type_name::<T>() == "f64") || (type_name::<T>() == "f32") {
            let real = self.re.to_string();
            let imag = self.im.to_string();

            if &imag[0..1] != "-" {
                return write!(f, "{} + {}i", &real, &imag);
            } else {
                return write!(f, "{} - {}i", &real, &imag[1..]);
            }
        } else if (type_name::<T>() == type_name::<Complex<f64>>()) || 
            (type_name::<T>() == type_name::<Complex<f32>>())
        {
            let real = self.re.to_string();
            let imag = self.im.to_string();

            let real_split: Vec<&str> = real.split_whitespace().collect();
            let imag_split: Vec<&str> = imag.split_whitespace().collect();

            let len = imag_split[2].len();

            if &imag_split[0][0..1] != "-" {
                return write!(
                    f,
                    "{} {} {} + {}j {} {}k",
                    &real_split[0],
                    &real_split[1],
                    &real_split[2],
                    &imag_split[0],
                    &imag_split[1],
                    &imag_split[2][..(len - 1)]
                );
            } else {
                return write!(
                    f,
                    "{} {} {} - {}j {} {}k",
                    &real_split[0],
                    &real_split[1],
                    &real_split[2],
                    &imag_split[0][1..],
                    &imag_split[1],
                    &imag_split[2][..(len - 1)]
                );
            }
        } else {
            return write!(f, "({}, {})", &self.re, &self.im);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComplexParseError;

impl fmt::Display for ComplexParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid format for a complex or hypercomplex type.")
    }
}

impl<T> FromStr for Complex<T> 
where 
    T: FromStr + Fill<f64>
{
    type Err = ComplexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if (type_name::<T>() == "f32") || (type_name::<T>() == "f64") {
            let float_str = r"^([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)?";
            let float_imag_str = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[iI])?$";
            let pattern = float_str.to_string() + &float_imag_str;
            
            let re = Regex::new(&pattern[..]).unwrap();
            let caps = re.captures(s).unwrap();
            let textx = caps.get(1).map_or("", |m| m.as_str());
            let texty = caps.get(2).map_or("", |m| m.as_str());
            
            
            let x = textx.parse::<T>();
            let y = texty.parse::<T>();
            
            let result = match (x, y) {
                (Ok(re), Ok(im)) => Ok(Self { re: re, im: im}),
                (Ok(re), Err(_)) => Ok(Self { re: re, im: <T as Fill<f64>>::zero()}),
                (Err(_), Ok(im)) => Ok(Self { re: <T as Fill<f64>>::zero(), im: im}),
                (Err(_), Err(_)) => Err(ComplexParseError),
            };
            
            return result;
        } else if (type_name::<T>() == type_name::<Complex<f64>>()) || 
            (type_name::<T>() == type_name::<Complex<f32>>()) 
        {
            let float_str = r"^([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)?";
            let float_imag_str_i = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[iI])?";
            let float_imag_str_j = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[jJ])?";
            let float_imag_str_k = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[kK])?$";
            let pattern = float_str.to_string() + &float_imag_str_i +
                &float_imag_str_j + &float_imag_str_k;
            
            let re = Regex::new(&pattern[..]).unwrap();
            let caps = re.captures(s).unwrap();
            let textx = caps.get(1).map_or("", |m| m.as_str());
            let texty = caps.get(2).map_or("", |m| m.as_str());
            let textu = caps.get(3).map_or("", |m| m.as_str());
            let textv = caps.get(4).map_or("", |m| m.as_str());
            
            let mut zstring = String::new();
            if (textx != "") && (texty != "") {
                let temp_str = textx.to_string() + &texty + &"i";
                zstring.push_str(&temp_str[..]);
            } else if (textx != "") {
                let temp_str = textx.to_string();
                zstring.push_str(&temp_str[..]);
            } else if (texty != "") {
                let temp_str = texty.to_string() + &"i";
                zstring.push_str(&temp_str[..]);
            }
            
            let mut wstring = String::new();
            if (textu != "") && (textv != "") {
                let temp_str = textu.to_string() + &textv + &"i";
                wstring.push_str(&temp_str[..]);
            } else if (textu != "") {
                let temp_str = textu.to_string();
                wstring.push_str(&temp_str[..]);
            } else if (textv != "") {
                let temp_str = textv.to_string() + &"i";
                wstring.push_str(&temp_str[..]);
            }
            
            let result = match (zstring.parse::<T>(), wstring.parse::<T>()) {
                (Ok(re), Ok(im)) => Ok(Self { re: re, im: im}),
                (Ok(re), Err(_)) => Ok(Self { re: re, im: <T as Fill<f64>>::zero()}),
                (Err(_), Ok(im)) => Ok(Self { re: <T as Fill<f64>>::zero(), im: im}),
                (Err(_), Err(_)) => Err(ComplexParseError),
            };
            
            return result;
        } else {
            let pattern = r"^\(\s*(.+)\s*,\s*(.+)\s*\)$";
            
            let re = Regex::new(&pattern[..]).unwrap();
            let caps = re.captures(s).unwrap();
            let textx = caps.get(1).map_or("", |m| m.as_str());
            let texty = caps.get(2).map_or("", |m| m.as_str());
            
            let result = match (textx.parse::<T>(), texty.parse::<T>()) {
                (Ok(re), Ok(im)) => Ok(Self { re: re, im: im}),
                (Ok(re), Err(_)) => Ok(Self { re: re, im: <T as Fill<f64>>::zero()}),
                (Err(_), Ok(im)) => Ok(Self { re: <T as Fill<f64>>::zero(), im: im}),
                (Err(_), Err(_)) => Err(ComplexParseError),
            };
            
            return result;
        }
        
        unreachable!();
    }
}
/*
impl FromStr for Complex<f64> 
{
    type Err = ComplexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let float_str = r"^([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)?";
        let float_imag_str = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[iI])?$";
        let pattern = float_str.to_string() + &float_imag_str;
        
        let re = Regex::new(&pattern[..]).unwrap();
        let caps = re.captures(s).unwrap();
        let textx = caps.get(1).map_or("", |m| m.as_str());
        let texty = caps.get(2).map_or("", |m| m.as_str());
        
        let x = textx.parse::<f64>();
        let y = texty.parse::<f64>();
        
        match (x, y) {
            (Ok(re), Ok(im)) => Ok(Self { re: re, im: im}),
            (Ok(re), Err(_)) => Ok(Self { re: re, im: 0.0_f64}),
            (Err(_), Ok(im)) => Ok(Self { re: 0.0_f64, im: im}),
            (Err(_), Err(_)) => Err(ComplexParseError),
        }
    }
}

impl FromStr for Complex<Complex<f64>> 
// where
    // Complex<T>: FromStr
{
    type Err = ComplexParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //if type_name::<T>() == f64 {
            let float_str = r"^([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)?";
            let float_imag_str_i = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[iI])?";
            let float_imag_str_j = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[jJ])?";
            let float_imag_str_k = r"(?:([+-]?(?:\d+|\d*\.\d+|\d+\.\d*)(?:[eE][+-]?\d{1,4})?)[kK])?$";
            let pattern = float_str.to_string() + &float_imag_str_i +
                &float_imag_str_j + &float_imag_str_k;
            
            let re = Regex::new(&pattern[..]).unwrap();
            let caps = re.captures(s).unwrap();
            let textx = caps.get(1).map_or("", |m| m.as_str());
            let texty = caps.get(2).map_or("", |m| m.as_str());
            let textu = caps.get(3).map_or("", |m| m.as_str());
            let textv = caps.get(4).map_or("", |m| m.as_str());
            
            let x = textx.parse::<f64>();
            let y = texty.parse::<f64>();
            let u = textu.parse::<f64>();
            let v = textv.parse::<f64>();
            
            match (x, y, u, v) {
                (Ok(re), Ok(i), Ok(j), Ok(k)) => Ok(complex!(re, i, j, k)),
                (Err(_), Ok(i),  Ok(j), Ok(k)) => Ok(complex!(0.0_f64, i, j, k)),
                (Ok(re), Err(_), Ok(j), Ok(k)) => Ok(complex!(re, 0.0_f64, j, k)),
                (Ok(re), Ok(i), Err(_), Ok(k)) => Ok(complex!(re, i, 0.0_f64, k)),
                (Ok(re), Ok(i), Ok(j), Err(_)) => Ok(complex!(re, i, j, 0.0_f64)),
                (Err(_), Err(_), Ok(j), Ok(k)) => Ok(complex!(0.0_f64, 0.0_f64, j, k)),
                (Err(_), Ok(i), Err(_), Ok(k)) => Ok(complex!(0.0_f64, i, 0.0_f64, k)),
                (Err(_), Ok(i), Ok(j), Err(_)) => Ok(complex!(0.0_f64, i, j, 0.0_f64)),
                (Ok(re), Err(_), Err(_), Ok(k)) => Ok(complex!(re, 0.0_f64, 0.0_f64, k)),
                (Ok(re), Err(_), Ok(j), Err(_)) => Ok(complex!(re, 0.0_f64, j, 0.0_f64)),
                (Ok(re), Ok(i), Err(_), Err(_)) => Ok(complex!(re, i, 0.0_f64, 0.0_f64)),
                (Ok(re), Err(_), Err(_), Err(_)) => Ok(complex!(re, 0.0_f64, 0.0_f64, 0.0_f64)),
                (Err(_), Ok(i), Err(_), Err(_)) => Ok(complex!(0.0_f64, i, 0.0_f64, 0.0_f64)),
                (Err(_), Err(_), Ok(j), Err(_)) => Ok(complex!(0.0_f64, 0.0_f64, j, 0.0_f64)),
                (Err(_), Err(_), Err(_), Ok(k)) => Ok(complex!(0.0_f64, 0.0_f64, 0.0_f64, k)),
                (Err(_), Err(_), Err(_), Err(_)) => Err(ComplexParseError),
            }
            /*
            let mut zstring = String::new();
            if (textx != "") && (texty != "") {
                let temp_str = textx.to_string() + &texty + &"i";
                zstring.push_str(&temp_str[..]);
            } else if (textx != "") {
                let temp_str = textx.to_string();
                zstring.push_str(&temp_str[..]);
            } else if (texty != "") {
                let temp_str = texty.to_string() + &"i";
                zstring.push_str(&temp_str[..]);
            }
            
            let mut wstring = String::new();
            if (textu != "") && (textv != "") {
                let temp_str = textu.to_string() + &textv + &"i";
                zstring.push_str(&temp_str[..]);
            } else if (textu != "") {
                let temp_str = textu.to_string();
                zstring.push_str(&temp_str[..]);
            } else if (textv != "") {
                let temp_str = textv.to_string() + &"i";
                zstring.push_str(&temp_str[..]);
            }
            
            Ok(Self {re: zstring.parse::<Complex<f64>>()?, 
                     im: wstring.parse::<Complex<f64>>()?} )
            */
        //}
    }
}
*/