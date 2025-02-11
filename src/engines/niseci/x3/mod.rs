use std::collections::{hash_map::Entry, HashMap};

use crate::model::niseci::{CampionamentoNISECI, ClassiEtaAlieniNISECI, ClassiEtaSpecieNISECI, InfoPopolazioniAlieneNISECI, InfoPopolazioniNISECI};




pub fn calculate_x3(c: &CampionamentoNISECI) -> Result<f32, Vec<String>> {

  let alieni_indigeni = c.get_numero_pesci_alieni_e_indigeni();
  
  // condizione 1
  if alieni_indigeni.alieni == 0 {
    return Ok(1.0);
  }

  // condizione 2
  if alieni_indigeni.alieni >= alieni_indigeni.indigeni {
    return Ok(0.0);
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
  if info_pop_aliene.tipo_1.popolazione_piu_strutt == 1.0 {
    return Ok(0.0);
  }

  // se le condizioni precendenti non si sono verificate
  // allora uso la formula x3 = 0.5 * (a + b)

  let a = calculate_a(&info_pop_aliene);
  let b = calculate_b(&info_pop_aliene);

  let x3 = 0.5 * (a + b);

  Ok(x3)
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
    } else if cattura.specie.tipo_autoctono == 1 || cattura.specie.tipo_autoctono == 2 {
      classi_eta.tot_specie_autoctone += 1;
    }
  }

  classi_eta.tot_specie_aliene = classi_eta.map_tipo_1.len() + classi_eta.map_tipo_2.len() + classi_eta.map_tipo_3.len();

  classi_eta
}


fn calculate_a(info: &InfoPopolazioniAlieneNISECI) -> f32 {

  if info.tipo_1.tot_species > 0 && info.tipo_1.popolazione_piu_strutt < 1.0 {
    return 0.5;
  }
  if info.tipo_2.tot_species >= info.tot_specie_autoctone {
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
  let i3 = 0.5 * (species_destrutt as f32 / info.tot_specie_aliene as f32);

  i2 + i3
}

