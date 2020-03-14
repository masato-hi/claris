use self::super::super::ext::YamlExt;
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

    Stroke {
      width: width,
      cap: cap,
    }
  }

  pub fn default() -> Stroke {
    Stroke {
      width: Self::DEFAULT_WIDTH,
      cap: Self::DEFAULT_CAP,
    }
  }
}
