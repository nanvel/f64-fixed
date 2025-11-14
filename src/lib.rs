pub trait ToFixedString {
    fn to_fixed_string(&self, places: isize) -> String;
}

impl ToFixedString for f64 {
    fn to_fixed_string(&self, places: isize) -> String {
        let align_left = places > 0;
        let places = places.abs().max(6) as usize;
        let sign = if self.is_sign_negative() { "-" } else { "" };
        let value = self.abs();

        let (mut mantissa, exp) = if *self == 0.0 {
            ("0.0".to_string(), "".to_string())
        } else {
            let exp = self.abs().log10().floor() as i32 / 3 * 3;
            if exp > 3 || exp < -3 || value < 0.001 {
                (format!("{:.6}", value / 10f64.powi(exp)), format!("{:+}", exp))
            } else {
                (format!("{:.6}", value), "".to_string())
            }
        };

        let mantissa_places = places.saturating_sub(sign.len() + exp.len());
        if let Some(pos) = mantissa.find('.') {
            if mantissa_places == pos {
                mantissa.truncate(pos);
            } else if mantissa_places > pos {
                mantissa.truncate(mantissa_places);
                mantissa = mantissa.trim_end_matches('0').trim_end_matches('.').to_string();
            }
        }

        if align_left {
            format!("{}{}{}{}", sign, mantissa, exp, " ".repeat(mantissa_places.saturating_sub(mantissa.len())))
        } else {
            format!("{}{}{}{}", " ".repeat(mantissa_places.saturating_sub(mantissa.len())), sign, mantissa, exp)
        }
    }
}

pub fn to_fixed_string(value: f64, places: isize) -> String {
    value.to_fixed_string(places)
}
