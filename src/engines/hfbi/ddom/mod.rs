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

use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI};

pub(crate) fn calc_ddom(campionamento: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {

  let (s90, b90): (u32, f32) = calc_s90_b90(campionamento, anagrafica);

  (((s90 as f32 - 1.0) / b90) + 1.0).ln()
}


fn calc_s90_b90(campionamento: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> (u32, f32) {

  let mut biomassa_tot = 0;
  for cattura in &campionamento.campionamento {
    biomassa_tot += cattura.peso;
  }

  let biomassa_90 = (biomassa_tot as f32 * 0.9) as u32;

  let mut n_specie_90: u32 = 0;
  let mut biomassa_tmp: u32 = 0;
  for cattura in  &campionamento.campionamento {
    biomassa_tmp += cattura.peso;
    n_specie_90 += 1;
    if biomassa_tmp > biomassa_90 {
      break;
    }
  }

  let area: f32 = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;
  let b90: f32 = ((biomassa_90 as f32 / area) * 100.0 +1.0).ln();

  (n_specie_90, b90)

}


#[cfg(test)]
mod ddom_private_tests {
    use crate::domain::hfbi::{CampionamentoHFBI, RecordHFBI, RIFERIMENTO_HFBI};



  // fn calc_b90_test1 {
  //   let record: Vec<RecordHFBI> = Vec::with_capacity(3);
  //   record.push(RecordHFBI { specie: RIFERIMENTO_HFBI[0], numero_individui: , peso: () });
  // }
}




