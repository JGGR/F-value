// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2025 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use core::f32;

#[derive(Debug, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

#[derive(Debug)]
pub enum LinearRegressionError {
    SameValues,
}

fn gradient_descent(m_now: f32, b_now: f32, points: &[Point<f32>], step: f32) -> (f32, f32) {
    let mut m_gradient: f32 = 0.0;
    let mut b_gradient: f32 = 0.0;

    let n = points.len();

    for point in points {
        let x = point.x;
        let y = point.y;

        m_gradient += -(2.0 / (n as f32)) * x * (y - (m_now * x + b_now));
        b_gradient += -(2.0 / (n as f32)) * (y - (m_now * x + b_now));
    }

    let m = m_now - m_gradient * step;
    let b = b_now - b_gradient * step;

    (m, b)
}

pub fn gradient_descent_iterate(
    points: &[Point<i32>],
) -> Result<(f32, f32), LinearRegressionError> {
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

pub fn calculate_quantita_with_regression(
    campionamenti: &[Point<i32>],
) -> Result<u32, String> {
    // trova m e b della retta
    let (m, b) = match gradient_descent_iterate(campionamenti) {
        Ok((m, b)) => (m, b),
        Err(error) => {
            match error {
                LinearRegressionError::SameValues => return Ok(get_sum(campionamenti)), // come da accordi ritorno la somma
            }
        }
    };

    let epsilon: f32 = 1e-6;
    if m.abs() < epsilon {
        // In questo caso, come da accordi, ritorniamo la somma dei campionamenti
        return Ok(get_sum(campionamenti));
    }

    if m > 0.0 {
        return Ok(get_sum(campionamenti));
    }

    // l'incorcio della retta con l'asse x ci da la quantita stimata
    let quantita_stimata = (-(b / m)) as i32;
    if quantita_stimata < 0 {
        return Err(format!("quantita stimata negativa {}", quantita_stimata));
    }
    Ok(quantita_stimata as u32)
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

fn normalize_points(points: &[Point<i32>]) -> Result<Vec<Point<f32>>, LinearRegressionError> {
    let max_x = points.iter().map(|p| p.x).max().unwrap() as f32;
    let min_x = points.iter().map(|p| p.x).min().unwrap() as f32;

    let max_y = points.iter().map(|p| p.y).max().unwrap() as f32;
    let min_y = points.iter().map(|p| p.y).min().unwrap() as f32;

    let epsilon: f32 = 1e-6;
    if (max_y - min_y).abs() < epsilon {
        return Err(LinearRegressionError::SameValues);
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

fn get_sum(points: &[Point<i32>]) -> u32 {
    let sum: i32 = points.iter().map(|point| point.y).sum();

    // le condizioni imposte prima di chiamare la fn garantiscono che sum >= 0
    // (il numero di esemplari in una cattura è >= 0)
    sum as u32
}
