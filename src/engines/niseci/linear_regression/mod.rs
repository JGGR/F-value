use std::process::exit;


fn gradient_descent(m_now: f32, b_now: f32, points: &[i32], step: f32) -> (f32, f32) {
    let mut m_gradient: f32 = 0.0;
    let mut b_gradient: f32 = 0.0;

    let n = points.len();

    for i in 0..n {
      let x = (i + 1) as f32;
      let y = points[i] as f32;

      m_gradient += - (2.0 / (n as f32)) * x * (y - (m_now * x + b_now));
      b_gradient += - (2.0 / (n as f32)) * (y - (m_now * x + b_now));
    }

    let m = m_now - m_gradient * step;
    let b = b_now - b_gradient * step;

    (m, b)
  
}

pub fn gradient_descent_iterate(points: &[i32]) -> (i32, i32) {
  
  let mut m: f32 = -1.0;
  let mut b: f32 = 100.0;

  const STEP: f32 = 0.1;
  const ITERATIONS: i32 = 1000;
  
  for _i in 0..ITERATIONS {
    (m, b) = gradient_descent(m, b, points, STEP);
  } 
  return (m.round() as i32, b.round() as i32)
}


pub fn calculate_quantita_stimata(campionamenti: &[i32]) -> Result<i32, i32> {

  // trova m e b della retta
  let (m, b) = gradient_descent_iterate(campionamenti);

  // trova quanti passaggi bisgonerebbe fare per catturare tutti i pesci
  // troviamo quindi il primo passaggio in cui non si troverebbero più pesci
  let n_passaggi = -1 * (b / m);
  
  if n_passaggi < 0 {
    println!("Errato calcolo regressione lineare: n_passaggi stimato is {}, avendo m = {}, b = {}", n_passaggi, m, b);
    return Err(n_passaggi)
  }

  // ora dobbiamo sommare tutti i pesci dei passaggi previsti
  let mut pesci_totali = 0;
  for i in 1..n_passaggi {
    pesci_totali += m * i + b;
  }

  Ok(pesci_totali)
}
