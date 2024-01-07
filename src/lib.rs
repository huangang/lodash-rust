#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn add(a: f64, b: f64) -> f64 {
  a + b
}

#[napi]
pub fn subtract(a: f64, b: f64) -> f64 {
  a - b
}

#[napi]
pub fn multiply(a: f64, b: f64) -> f64 {
  a * b
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

#[napi]
pub fn round(value: f64, decimal_places: u32) -> f64 {
  let multiplier = 10_f64.powi(decimal_places as i32);
  (value * multiplier).round() / multiplier
}

#[napi]
pub fn sum(nums: Vec<f64>) -> f64 {
  nums.iter().sum()
}

#[napi]
pub fn max(nums: Vec<f64>) -> Option<f64> {
  match nums.iter().max_by(|a, b| a.partial_cmp(b).unwrap()) {
    Some(&max_value) => Some(max_value),
    None => None,
  }
}

#[napi]
pub fn min(nums: Vec<f64>) -> Option<f64> {
  match nums.iter().min_by(|a, b| a.partial_cmp(b).unwrap()) {
    Some(&max_value) => Some(max_value),
    None => None,
  }
}

#[napi]
pub fn mean(nums: Vec<f64>) -> Option<f64> {
  let sum: f64 = nums.iter().sum();
  let length = nums.len() as f64;
  if length > 0.0 {
    Some(sum / length)
  } else {
    None
  }
}
