//! Simple significant figure-based rounding.
use decimal::d128;

/// Rounded addition.
/// TODO: Correct rounding algorithm.
pub fn add(l: d128, r: d128) -> d128 {
    (l + r).quantize(l.min(r))
}

/// Rounded subtraction.
/// TODO: Correct rounding algorithm.
pub fn sub(l: d128, r: d128) -> d128 {
    (l - r).quantize(l.min(r))
}

/// Rounded multiplication
pub fn mul(l: d128, r: d128) -> d128 {
    (l * r).quantize(l.min(r))
}

/// Rounded division.
pub fn div(l: d128, r: d128) -> d128 {
    (l / r).quantize(l.min(r))
}

/// Rounded remainder division.
pub fn rem(l: d128, r: d128) -> d128 {
    (l % r).quantize(l.min(r))
}
