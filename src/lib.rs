#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn add(a: f64, b: f64) -> f64 {
  a + b
}

#[napi]
pub fn divide(a: f64, b: f64) -> f64 {
  a / b
}

#[napi]
pub fn ceil(num: f64, precision: u32) -> f64 {
  // div_ceil 
  let multiplier = 10_f64.powi(precision as i32);
  (num * multiplier).ceil() / multiplier
}

#[napi]
pub fn floor(num: f64, precision: u32) -> f64 {
  let multiplier = 10_f64.powi(precision as i32);
  (num * multiplier).floor() / multiplier
}