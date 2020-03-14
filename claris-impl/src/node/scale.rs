use self::super::super::ext::YamlExt;
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
