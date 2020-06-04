use std::any::type_name;
use std::{fmt, str::FromStr};
use std::num::ParseFloatError;
use crate::*;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

impl<T> fmt::Display for Complex<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if (type_of(&self.re) == "f64") || (type_of(&self.re) == "f32"){
            let real = self.re.to_string();
            let imag = self.im.to_string();

            if &imag[0..1] != "-" {
                return write!(f, "{} + {}i", &real, &imag);
            } else {
                return write!(f, "{} - {}i", &real, &imag[1..]);
            }
        } else if (type_of(&self.re) == type_name::<Complex<f64>>()) || 
            (type_of(&self.re) == type_name::<Complex<f32>>())
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

/*
impl FromStr for Complex<T> {
    where T: FromStr
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // suppose "4+2i" or "4+i2" or "2i+4" or "i2+4"
        let coords: Vec<&str> = s.split(|c| {c == '+' || c == '-'}).collect();



        //if coords.len() == 2 {
        let x = (coords[0]).parse::<T>()?;
        let y = (coords[1] ).parse::<T>()?;

        Ok(Complex<T>{ re: x, im: y})
    }
}
*/