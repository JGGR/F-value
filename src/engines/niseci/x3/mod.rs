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

use std::collections::hash_map::Entry;

pub struct MetricheX3 {
    criterio_a: f32,
    criterio_b: f32
}

impl MetricheX3 {
    pub fn new(criterio_a: f32, criterio_b: f32) -> Self {
        Self {
            criterio_a: criterio_a,
            criterio_b: criterio_b
        }
    }
    pub fn get_criterio_a(&self) -> f32 {
        return self.criterio_a;
    }
    pub fn get_criterio_b(&self) -> f32 {
        return self.criterio_b;
    }
}

use crate::model::niseci::{CampionamentoNISECI, ClassiEtaAlieniNISECI, ClassiEtaSpecieNISECI, InfoPopolazioniAlieneNISECI};

pub fn calculate_x3(c: &CampionamentoNISECI) -> Result<(f32, MetricheX3), Vec<String>> {

  let alieni_indigeni = c.get_numero_pesci_alieni_e_indigeni();

  // condizione 1
  if alieni_indigeni.alieni == 0 {
    return Ok((1.0, MetricheX3::new(-1.0, -1.0)));
  }

  // condizione 2
  if alieni_indigeni.alieni >= alieni_indigeni.indigeni {
    return Ok((0.0, MetricheX3::new(-1.0, -1.0)));
  }


  // mi serve ora capire se ci sono specie aliene con popolazioni strutturate o meno
  // il calcolo è simile a quello usato per calcolare x2_a
  // solo che questa volta lo faccio sulle specie aliene
  // e suddivido in base al tipo di specie aliena
  let classi_eta = calculate_classi_eta_alieni(c);

  // ora ho ottenuto le classi di eta per ogni specie aliena trovata

  let info_pop_aliene = match InfoPopolazioniAlieneNISECI::get_info_pop_aliene(&classi_eta) {
    Ok(info) => info,
    Err(errors) => return Err(errors)
  };

  // condizione 3
  let epsilon: f32 = 1e-6;
  if (info_pop_aliene.tipo_1.popolazione_piu_strutt - 1.0).abs() < epsilon {
    return Ok((0.0, MetricheX3::new(-1.0, -1.0)));
  }


  // se le condizioni precendenti non si sono verificate
  // allora uso la formula x3 = 0.5 * (a + b)

  let a = calculate_a(&info_pop_aliene);
  let b = calculate_b(&info_pop_aliene);

  let x3 = 0.5 * (a + b);

  Ok((x3, MetricheX3::new(a, b)))
}

fn calculate_classi_eta_alieni(c: &CampionamentoNISECI) -> ClassiEtaAlieniNISECI {

  let mut classi_eta = ClassiEtaAlieniNISECI::new();

  // riempo l'hashmap con solo le specie alloctone campionate
  for cattura in &c.campionamento {
    if cattura.specie.tipo_alloctono == 1 {
      match classi_eta.map_tipo_1.entry(cattura.specie.id.clone()) {
        Entry::Occupied(mut entry) => {
          entry.get_mut().update_classi_eta(&cattura);
        },
        Entry::Vacant(entry) => {
          entry.insert(ClassiEtaSpecieNISECI::new_cl_prevalorizzata(&cattura));
        }
      };
    } else if cattura.specie.tipo_alloctono == 2 {
      match classi_eta.map_tipo_2.entry(cattura.specie.id.clone()) {
        Entry::Occupied(mut entry) => {
          entry.get_mut().update_classi_eta(&cattura);
        },
        Entry::Vacant(entry) => {
          entry.insert(ClassiEtaSpecieNISECI::new_cl_prevalorizzata(&cattura));
        }
      };
    } else if cattura.specie.tipo_alloctono == 3 {
      match classi_eta.map_tipo_3.entry(cattura.specie.id.clone()) {
        Entry::Occupied(mut entry) => {
          entry.get_mut().update_classi_eta(&cattura);
        },
        Entry::Vacant(entry) => {
          entry.insert(ClassiEtaSpecieNISECI::new_cl_prevalorizzata(&cattura));
        }
      };
    }
    classi_eta.tot_specie_autoctone = c.get_tot_specie_autoctone();
  }

  classi_eta.tot_specie_aliene = classi_eta.map_tipo_1.len() + classi_eta.map_tipo_2.len() + classi_eta.map_tipo_3.len();

  classi_eta
}


fn calculate_a(info: &InfoPopolazioniAlieneNISECI) -> f32 {

  if info.tipo_1.tot_species > 0 && info.tipo_1.popolazione_piu_strutt < 1.0 {
    return 0.5;
  }
  if info.tipo_2.tot_species != 0 && info.tipo_2.tot_species >= info.tot_specie_autoctone {
    return 0.5;
  }
  if info.tipo_2.tot_species != 0 && info.tipo_2.tot_species < info.tot_specie_autoctone {
    return 0.75;
  }
  if info.tipo_3.tot_species >= info.tot_specie_autoctone {
    return 0.75;
  }
  if info.tipo_3.tot_species != 0 && info.tipo_3.tot_species < info.tot_specie_autoctone {
    return 0.85;
  }

  1.0
}

fn calculate_b(info: &InfoPopolazioniAlieneNISECI) -> f32 {

  let specie_mediamente_strutt = info.get_species_mediamente_strutt();
  let species_destrutt = info.get_species_destrutt();

  let i2 = 0.5 * (specie_mediamente_strutt as f32 / info.tot_specie_aliene as f32);
  let i3 = species_destrutt as f32 / info.tot_specie_aliene as f32;

  i2 + i3
}


// tutte le fn private le mettiamo qua
#[cfg(test)]
pub mod tests {
  use super::*;

  #[test]
  fn calculate_b_tutte_destrutt() {
    let mut info_aliene = InfoPopolazioniAlieneNISECI::new();
    info_aliene.tipo_3.species_destrutt = 10;
    info_aliene.tipo_2.species_destrutt = 20;
    info_aliene.tipo_3.species_strutt = 20;
    info_aliene.tot_specie_aliene = 50;

    let b = calculate_b(&info_aliene);

    assert_eq!(b, 0.6);


    info_aliene.tipo_3.species_strutt = 0;
    info_aliene.tot_specie_aliene = 30;

    let b = calculate_b(&info_aliene);

    assert_eq!(b, 1.0);

  }

  #[test]
  fn calculate_b_tutte_mediam_strutt() {
    let mut info_aliene = InfoPopolazioniAlieneNISECI::new();
    info_aliene.tipo_3.species_mediamente_strutt = 10;
    info_aliene.tipo_2.species_mediamente_strutt = 20;
    info_aliene.tipo_3.species_strutt = 20;
    info_aliene.tot_specie_aliene = 50;

    let b = calculate_b(&info_aliene);

    assert_eq!(b, 0.3);


    info_aliene.tipo_3.species_strutt = 0;
    info_aliene.tot_specie_aliene = 30;

    let b = calculate_b(&info_aliene);

    assert_eq!(b, 0.5);
  }

  #[test]
  fn calculate_b_miscellaneous() {
    let mut info_aliene = InfoPopolazioniAlieneNISECI::new();
    info_aliene.tipo_3.species_mediamente_strutt = 10;
    info_aliene.tipo_2.species_mediamente_strutt = 20;
    info_aliene.tipo_3.species_destrutt = 10;
    info_aliene.tipo_2.species_destrutt = 20;
    info_aliene.tipo_3.species_strutt = 20;
    info_aliene.tot_specie_aliene = 80;

    let b = calculate_b(&info_aliene);

    assert_eq!(b, 0.5625);
  }

  /// if info.tipo_1.tot_species > 0 && info.tipo_1.popolazione_piu_strutt < 1.0 {
  ///   return 0.5;
  /// }
  #[test]
  fn calculate_a_tipo_1_presente_ma_no_strutt() {
    let mut info = InfoPopolazioniAlieneNISECI::new();
    info.tipo_1.tot_species = 2;
    info.tipo_1.species_mediamente_strutt = 2;
    info.tipo_1.popolazione_piu_strutt = 0.5;

    let a = calculate_a(&info);
    assert_eq!(a, 0.5);

    info.tipo_1.species_destrutt = 1;
    info.tipo_1.tot_species= 3;

    let a = calculate_a(&info);
    assert_eq!(a, 0.5);

  }

  /// if info.tipo_2.tot_species >= info.tot_specie_autoctone {
  ///   return 0.5;
  /// }
  #[test]
  fn calculate_a_tipo_2_magg_autoctone() {
    let mut info = InfoPopolazioniAlieneNISECI::new();
    info.tipo_2.tot_species = 3;
    info.tot_specie_autoctone = 2;

    let a = calculate_a(&info);
    assert_eq!(a, 0.5);
  }

  /// if info.tipo_2.tot_species != 0 && info.tipo_2.tot_species < info.tot_specie_autoctone {
  ///   return 0.75;
  /// }
  #[test]
  fn calculate_a_tipo_2_min_autoctone() {
    let mut info = InfoPopolazioniAlieneNISECI::new();
    info.tipo_2.tot_species = 2;
    info.tot_specie_autoctone = 3;

    let a = calculate_a(&info);
    assert_eq!(a, 0.75);
  }

  /// if info.tipo_3.tot_species >= info.tot_specie_autoctone {
  ///   return 0.75;
  /// }
  #[test]
  fn calculate_a_tipo_3_magg_autoctone() {
    let mut info = InfoPopolazioniAlieneNISECI::new();
    info.tipo_3.tot_species = 3;
    info.tot_specie_autoctone = 2;

    let a = calculate_a(&info);
    assert_eq!(a, 0.75);
  }

  /// if info.tipo_3.tot_species != 0 && info.tipo_3.tot_species < info.tot_specie_autoctone {
  ///   return 0.85;
  /// }
  #[test]
  fn calculate_a_tipo_3_min_autoctone() {
    let mut info = InfoPopolazioniAlieneNISECI::new();
    info.tipo_3.tot_species = 2;
    info.tot_specie_autoctone = 3;

    let a = calculate_a(&info);
    assert_eq!(a, 0.85);
  }

  #[test]
  fn calculate_a_progressive_scaling() {
    let mut info = InfoPopolazioniAlieneNISECI::new();

    info.tipo_3.tot_species = 2;
    info.tot_specie_autoctone = 3;
    let a = calculate_a(&info);
    assert_eq!(a, 0.85);

    info.tipo_2.tot_species = 2;
    let a = calculate_a(&info);
    assert_eq!(a, 0.75);

    info.tipo_1.tot_species = 2;
    info.tipo_1.species_mediamente_strutt = 2;
    info.tipo_1.popolazione_piu_strutt = 0.5;
    let a = calculate_a(&info);
    assert_eq!(a, 0.5);

    info.tipo_1.species_destrutt = 1;
    info.tipo_1.tot_species= 3;
    let a = calculate_a(&info);
    assert_eq!(a, 0.5);
  }

}
