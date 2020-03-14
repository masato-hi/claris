use std::fmt;

#[derive(Debug)]
pub enum NodeError {
  Required(String, String),
  InvalidColor(String),
  InvalidLayer,
  InvalidLayerCount,
  InvalidLayerDefine,
  UnknownLayer(String),
  InvalidVertex,
  InvalidPoint,
}

impl fmt::Display for NodeError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      NodeError::Required(x, y) => f.write_fmt(format_args!("'{}' is required '{}' option", x, y)),
      NodeError::InvalidColor(x) => f.write_fmt(format_args!("invalid color format '{}'", x)),
      NodeError::InvalidLayer => f.write_str("invalid layers"),
      NodeError::InvalidLayerCount => f.write_str("invalid layers count"),
      NodeError::InvalidLayerDefine => f.write_str("invalid layer define"),
      NodeError::UnknownLayer(x) => f.write_fmt(format_args!("unknown layer type '{}'", x)),
      NodeError::InvalidVertex => f.write_str("invalid vertex"),
      NodeError::InvalidPoint => f.write_str("invalid point"),
    }
  }
}
