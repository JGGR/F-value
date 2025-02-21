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

use crate::model::niseci::{CampionamentoNISECI, RiferimentoNISECI, AnagraficaNISECI};

use super::x1::calculate_x1;
use super::x2::calculate_x2;
use super::x3::calculate_x3;

const RQE_NISECI_MAGIC_ADDEND: f32 = 1.1283;
const RQE_NISECI_MAGIC_QUOTIENT: f32 = 1.0603;

pub fn calculate_niseci(campionamento: &CampionamentoNISECI, riferimento: &RiferimentoNISECI, anagrafica: &AnagraficaNISECI) -> Result<f32, Vec<String>> {
    let mut errors = Vec::new();
    let x1 = calculate_x1(campionamento, riferimento);

    let x2 = calculate_x2(campionamento, anagrafica);
    match x2 {
        Ok(_) => {},
        Err(x2_errors) => {
            for e in x2_errors {
                errors.push(format!("Errore durante calcolo x2: {}", e));
            }
            return Err(errors);
        }
    }
    let x2 = x2.expect("calc_niseci() returned earlier on Err match");

    let x3 = calculate_x3(campionamento);
    match x3 {
        Ok(_) => {},
        Err(x3_errors) => {
            for e in x3_errors {
                errors.push(format!("Errore durante calcolo x3: {}", e));
            }
            return Err(errors);
        }
    }
    let x3 = x3.expect("calc_niseci() returned earlier on Err match");

    let mut x1_x2_errors = Vec::new();
    if x1 < 0.0 {
        x1_x2_errors.push(format!("Errore risultato x1: valore negativo: {}", x1));
    }
    if x2 < 0.0 {
        x1_x2_errors.push(format!("Errore risultato x2: valore negativo: {}", x2));
    }
    if x1_x2_errors.len() != 0 {
        return Err(x1_x2_errors);
    }
    let niseci = (0.1 * x1.sqrt()) +
        (0.1 * x2.sqrt()) +
        (0.8 * (x1 * x2)) -
        ( (0.1 * (1.0 - x3)) *
          ((0.1 * x1.sqrt()) + (0.1 * x2.sqrt()) + (0.8 * (x1 * x2)))
        );
    return Ok(niseci);
}

pub fn calculate_rqe_niseci(niseci: f32) -> f32 {
    return (niseci.ln() +  RQE_NISECI_MAGIC_ADDEND ) / RQE_NISECI_MAGIC_QUOTIENT;
}
