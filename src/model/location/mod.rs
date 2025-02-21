use std::fmt;

#[derive(Clone)]
pub struct Location {
  pub regione: String,
  pub provincia: String
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = format!("{{ regione: {{{}}}, provincia: {{{}}} }}",
        self.regione, self.provincia);
    write!(f, "{}", string_representation)
  }
}
