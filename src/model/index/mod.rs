use std::{fmt};

#[derive(Debug, Default, Clone)]
pub enum Indice {
  #[default]
  NISECI,
  HFBI
}

impl fmt::Display for Indice {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match *self {
      Indice::NISECI => "NISECI",
      Indice::HFBI => "HFBI"
    };
    write!(f, "{}", string_representation)
  }
}
