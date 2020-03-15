use crate::ext::YamlExt;
use cairo::LineCap;
use yaml_rust::yaml::Hash;

#[derive(Debug, Clone, Copy)]
pub struct Stroke {
    pub width: f64,
    pub cap: LineCap,
}

impl Stroke {
    const DEFAULT_WIDTH: f64 = 1.0;
    const DEFAULT_CAP: LineCap = LineCap::Butt;

    pub fn parse(src: &Hash) -> Stroke {
        let width = src.f64_val("width").unwrap_or(Self::DEFAULT_WIDTH);

        let cap: LineCap = match src.str_val("cap") {
            Some(x) => match x {
                "butt" => LineCap::Butt,
                "round" => LineCap::Round,
                "square" => LineCap::Square,
                _ => Self::DEFAULT_CAP,
            },
            _ => Self::DEFAULT_CAP,
        };

        Stroke { width, cap }
    }

    pub fn default() -> Stroke {
        Stroke {
            width: Self::DEFAULT_WIDTH,
            cap: Self::DEFAULT_CAP,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Stroke;
    use crate::parse_yaml;
    use cairo::LineCap;
    use float_cmp::approx_eq;
    use yaml_rust::Yaml;
    use yaml_rust::YamlLoader;

    macro_rules! parse {
        ($x:expr) => {{
            let src = parse_yaml!($x);
            match src {
                Yaml::Hash(x) => Stroke::parse(&x),
                _ => panic!("invalid yaml"),
            }
        }};
    }

    #[test]
    fn width_is_blank() {
        let s = "---\n{}";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.width, 1.0));
    }

    #[test]
    fn width_is_not_blank() {
        let s = "---
width: 2.5
        ";
        let subject = parse!(s);
        assert!(approx_eq!(f64, subject.width, 2.5));
    }

    #[test]
    fn cap_is_blank() {
        let s = "---\n{}";
        let subject = parse!(s);
        assert_eq!(subject.cap, LineCap::Butt);
    }

    #[test]
    fn cap_is_butt() {
        let s = "---
cap: butt
        ";
        let subject = parse!(s);
        assert_eq!(subject.cap, LineCap::Butt);
    }

    #[test]
    fn cap_is_square() {
        let s = "---
cap: square
        ";
        let subject = parse!(s);
        assert_eq!(subject.cap, LineCap::Square);
    }

    #[test]
    fn cap_is_round() {
        let s = "---
cap: round
        ";
        let subject = parse!(s);
        assert_eq!(subject.cap, LineCap::Round);
    }

    #[test]
    fn cap_is_invalid() {
        let s = "---
cap: invalid
        ";
        let subject = parse!(s);
        assert_eq!(subject.cap, LineCap::Butt);
    }
}
