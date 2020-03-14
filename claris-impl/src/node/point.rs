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

                    Some(Point { x: x, y: y })
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

            Some(Point { x: x, y: y })
        } else {
            None
        }
    }
}
