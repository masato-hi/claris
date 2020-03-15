use yaml_rust::Yaml;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait DefPoint {
    fn as_point(&self) -> Option<Point>;
}

impl DefPoint for Yaml {
    fn as_point(&self) -> Option<Point> {
        match &self {
            Yaml::Array(p) => {
                if p.len() == 2 {
                    let x = match p[0].as_i64() {
                        Some(x) => x as f64,
                        None => match p[0].as_f64() {
                            Some(x) => x,
                            None => return None,
                        },
                    };

                    let y = match p[1].as_i64() {
                        Some(x) => x as f64,
                        None => match p[1].as_f64() {
                            Some(x) => x,
                            None => return None,
                        },
                    };

                    Some(Point { x, y })
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl DefPoint for Vec<Yaml> {
    fn as_point(&self) -> Option<Point> {
        if self.len() == 2 {
            let x = match self[0].as_i64() {
                Some(x) => x as f64,
                None => match self[0].as_f64() {
                    Some(x) => x,
                    None => return None,
                },
            };

            let y = match self[1].as_i64() {
                Some(x) => x as f64,
                None => match self[1].as_f64() {
                    Some(x) => x,
                    None => return None,
                },
            };

            Some(Point { x, y })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefPoint;
    use crate::parse_yaml;
    use float_cmp::approx_eq;
    use yaml_rust::YamlLoader;

    #[test]
    fn yaml_integer_point() {
        let s = "---
- 10
- 20
";
        let src = parse_yaml!(s);
        let subject = src.as_point().unwrap();
        assert!(approx_eq!(f64, subject.x, 10.0));
        assert!(approx_eq!(f64, subject.y, 20.0));
    }

    #[test]
    fn yaml_float_point() {
        let s = "---
- 10.1
- 20.2
";
        let src = parse_yaml!(s);
        let subject = src.as_point().unwrap();
        assert!(approx_eq!(f64, subject.x, 10.1));
        assert!(approx_eq!(f64, subject.y, 20.2));
    }

    #[test]
    fn yaml_invalid_point_x() {
        let s = "---
- '10.1'
- 20.2
";
        let src = parse_yaml!(s);
        assert!(src.as_point().is_none());
    }

    #[test]
    fn yaml_invalid_point_y() {
        let s = "---
- 10.1
- '20.2'
";
        let src = parse_yaml!(s);
        assert!(src.as_point().is_none());
    }

    #[test]
    fn yaml_invalid_point() {
        let s = "---
- 10.1
- 20.2
- 33.3
";
        let src = parse_yaml!(s);
        assert!(src.as_point().is_none());
    }

    #[test]
    fn yaml_not_array() {
        let s = "---{}";
        let src = parse_yaml!(s);
        assert!(src.as_point().is_none());
    }

    #[test]
    fn vec_integer_point() {
        let s = "---
- 10
- 20
";
        let src = parse_yaml!(s);
        let subject = src.as_vec().unwrap().as_point().unwrap();
        assert!(approx_eq!(f64, subject.x, 10.0));
        assert!(approx_eq!(f64, subject.y, 20.0));
    }

    #[test]
    fn vec_float_point() {
        let s = "---
- 10.1
- 20.2
";
        let src = parse_yaml!(s);
        let subject = src.as_vec().unwrap().as_point().unwrap();
        assert!(approx_eq!(f64, subject.x, 10.1));
        assert!(approx_eq!(f64, subject.y, 20.2));
    }

    #[test]
    fn vec_invalid_point_x() {
        let s = "---
- '10.1'
- 20.2
";
        let src = parse_yaml!(s);
        assert!(src.as_vec().unwrap().as_point().is_none());
    }

    #[test]
    fn vec_invalid_point_y() {
        let s = "---
- 10.1
- '20.2'
";
        let src = parse_yaml!(s);
        assert!(src.as_vec().unwrap().as_point().is_none());
    }

    #[test]
    fn vec_invalid_point() {
        let s = "---
- 10.1
- 20.2
- 33.3
";
        let src = parse_yaml!(s);
        assert!(src.as_vec().unwrap().as_point().is_none());
    }
}
