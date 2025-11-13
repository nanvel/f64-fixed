# `f64-fixed` converts `f64` to a fixed size `String`

Use cases:
- Display arbitrary numbers on a device with limited number of indicators
- Display a number in a fixed-font where alignment matters or space is limited

## Usage

```rust
use f64_fixed::{to_fixed_string, ToFixedString};

assert_eq!(12345.0_f64.to_fixed_string(6), "12345 ");

assert_eq!(to_fixed_string(1., 6), "1     ");
assert_eq!(to_fixed_string(1., -6), "     1");

assert_eq!(to_fixed_string(12345., 6), "12345 ");
assert_eq!(to_fixed_string(123456., 6), "123456");
assert_eq!(to_fixed_string(1234567., 6), "1.23+6");
```
