use std::fmt;

#[derive(Debug)]
struct Complex {
    real: i32,
    imaginary: i32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_debug() {
        let c = Complex {
            real: 3,
            imaginary: 4,
        };
        assert_eq!(
            format!("{:?}", c),
            "Complex { real: 3, imaginary: 4 }".to_string()
        );
    }

    #[test]
    fn test_fmt_display() {
        let c = Complex {
            real: 3,
            imaginary: 4,
        };
        assert_eq!(c.to_string(), "3 + 4i".to_string());
    }
}
