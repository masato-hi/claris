use crate::ext::YamlExt;
use yaml_rust::yaml::Hash;

#[derive(Debug, Clone, Copy)]
pub struct Scale {
    pub x: f64,
    pub y: f64,
}

impl Scale {
    const DEFAULT_SCALE: f64 = 1.0;

    pub fn parse(src: &Hash) -> Scale {
        let x = src.f64_val("x").unwrap_or(Self::DEFAULT_SCALE);
        let y = src.f64_val("y").unwrap_or(Self::DEFAULT_SCALE);

        Scale { x, y }
    }

    pub fn default() -> Scale {
        Scale {
            x: Self::DEFAULT_SCALE,
            y: Self::DEFAULT_SCALE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Scale;
    use crate::parse_yaml;
    use float_cmp::approx_eq;
    use yaml_rust::Yaml;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match src {
                Yaml::Hash(x) => Scale::parse(&x),
                _ => panic!("invalid yaml"),
            }
        }};
    }

    #[test]
    fn x_is_blank() {
        let s = "---
y: 3.5
        ";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 1.0));
    }

    #[test]
    fn x_is_not_blank() {
        let s = "---
x: 2.5
y: 3.5
        ";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.x, 2.5));
    }
    #[test]
    fn y_is_blank() {
        let s = "---
x: 2.5
        ";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.y, 1.0));
    }

    #[test]
    fn y_is_not_blank() {
        let s = "---
x: 2.5
y: 3.5
        ";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.y, 3.5));
    }
}
