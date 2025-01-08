use core::f32;
use std::process::exit;


#[derive(Debug, PartialEq)]
pub struct Point<T> {
  pub x: T,
  pub y: T
}

impl<T> Point<T> {
  pub fn new(x: T, y: T) -> Point<T> {
    Point {
      x, 
      y
    }
  }
}

fn gradient_descent(m_now: f32, b_now: f32, points: &[Point<f32>], step: f32) -> (f32, f32) {
    let mut m_gradient: f32 = 0.0;
    let mut b_gradient: f32 = 0.0;

    let n = points.len();

    for point in points {
      let x = point.x as f32;
      let y = point.y as f32;

      m_gradient += - (2.0 / (n as f32)) * x * (y - (m_now * x + b_now));
      b_gradient += - (2.0 / (n as f32)) * (y - (m_now * x + b_now));
    }

    let m = m_now - m_gradient * step;
    let b = b_now - b_gradient * step;

    (m, b)
  
}

pub fn gradient_descent_iterate(points: &[Point<i32>]) -> Result<(f32, f32), String> {
  
  let normalized_points = match normalize_points(points) {
    Ok(norm) => norm,
    Err(error) => {
      return Err(error); // faccio salire l'errore
    }
  };

  let mut m_norm: f32 = -1.0;
  let mut b_norm: f32 = 1.0;

  const STEP: f32 = 0.001;
  const ITERATIONS: i32 = 10000;
  
  for _i in 0..ITERATIONS {
    (m_norm, b_norm) = gradient_descent(m_norm, b_norm, &normalized_points, STEP);
  }

  let (m_final, b_final) = denormalize_retta(m_norm, b_norm, points);

  Ok((m_final, b_final))

}


pub fn calculate_quantita_stimata(campionamenti: &[Point<i32>]) -> Result<i32, String> {

  // trova m e b della retta
  let (m, b) = match gradient_descent_iterate(campionamenti) {
    Ok((m, b)) => (m, b),
    Err(error) => {
      return Err(error)
    }
  };

  if m.abs() < f32::EPSILON {
    println!("m è 0");
    return Err("Regression line is horizontal; no meaningful passaggi calculation possible.".to_string());
  }

  // l'incorcio della retta con l'asse x ci da la quantita stimata
  let quantita_stimata = (-1.0 * (b / m)) as i32;
  if quantita_stimata < 0 {
    return Err(format!("quantita stimata negativa {}", quantita_stimata));
  }
  return Ok(quantita_stimata);
}

/// La denormalizzazione riporta la retta normalizzata (rappresentata da m_norm e b_norm)
/// che vive nel piano normalizzato {[0, 1], [0, 1]},
/// nel suo spazio originale (piano cartesiano {R, R})
fn denormalize_retta(m_norm: f32, b_norm: f32, points: &[Point<i32>]) -> (f32, f32) {
  let max_x = points.iter().map(|p| p.x).max().unwrap() as f32;
  let min_x = points.iter().map(|p| p.x).min().unwrap() as f32;

  let max_y = points.iter().map(|p| p.y).max().unwrap() as f32;
  let min_y = points.iter().map(|p| p.y).min().unwrap() as f32;
  
  let m = m_norm * (max_y - min_y) / (max_x - min_x);
  let b = b_norm * (max_y - min_y) + min_y - m * min_x;
  (m, b)
}

fn normalize_points(points: &[Point<i32>]) -> Result<Vec<Point<f32>>, String> {
  let max_x = points.iter().map(|p| p.x).max().unwrap() as f32;
  let min_x = points.iter().map(|p| p.x).min().unwrap() as f32;

  let max_y = points.iter().map(|p| p.y).max().unwrap() as f32;
  let min_y = points.iter().map(|p| p.y).min().unwrap() as f32;

  if (max_y - min_y).abs() < f32::EPSILON {
    return Err("All y values are the same; cannot perform linear regression.".to_string());
  }

  let normalized_points = points
      .iter()
      .map(|p| {
          let x_norm = (p.x as f32 - min_x) / (max_x - min_x);
          let y_norm = (p.y as f32 - min_y) / (max_y - min_y);
          Point::new(x_norm, y_norm)
      })
      .collect();

  Ok(normalized_points)
}

