use std::collections::{btree_map::{OccupiedEntry, VacantEntry}, hash_map::Entry, HashMap};

use crate::{model::niseci::{CampionamentoNISECI, ClassiEta, ClassiEtaSpecieNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI}, RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI};




pub fn calculate_x2(riferimento: &RiferimentoNISECI, campionamento: &CampionamentoNISECI) -> f32 {
  let x2_a = calculate_sommatoria_x2_a(riferimento, campionamento);
  // let x2_b = calculate_sommatoria_x2_b(riferimento, campionamento);

  0.0
}

fn calculate_sommatoria_x2_a(r: &RiferimentoNISECI, c: &CampionamentoNISECI) -> f32 {

  // ad ogni specie associo le loro classi che andrò poi a riempire
  // ho controllato i campionamenti di andrea e trovto massimo 9 specie diverse
  // per sicurezza prealloco memoria per 10 classi di eta
  let mut classi_eta_map: HashMap<String, ClassiEtaSpecieNISECI> = HashMap::with_capacity(10);

  // riempo l'hashmap con solo le specie autoctone campionate
  for c in &c.campionamento {
    if c.specie.tipo_autoctono == 1 || c.specie.tipo_autoctono == 2 {
      match classi_eta_map.entry(c.specie.id.clone()) {
        Entry::Occupied(mut entry) => {
          entry.get_mut().update_classi_eta(&c);
        },
        Entry::Vacant(entry) => {
          entry.insert(ClassiEtaSpecieNISECI::new_cl_prevalorizzata(&c));
        }
      };
    } 
  }

  // ora la mappa è riempita e tutte le classi sono state riempite
  // si procede quindi al calcolo di x2 a per ogni specie campionata autoctona
  // e si va a fare la sommatoria dei parametri trovati

  let mut sommatoria_x2_a = 0.0;
  for (key, classe) in &classi_eta_map {
    sommatoria_x2_a += calculate_x2_a(classe);
  }

  sommatoria_x2_a
}

fn calculate_sommatoria_x2_b() {

} 

fn update_classi_eta(cl: &mut ClassiEtaSpecieNISECI, record: &RecordNISECI) -> () {
  if record.lunghezza < record.specie.cl_soglia1 {
    cl.cl1 += 1;
  } else if record.lunghezza < record.specie.cl_soglia2 {
    cl.cl2 += 1;
  } else if record.lunghezza < record.specie.cl_soglia3 {
    cl.cl3 += 1;
  } else if record.lunghezza < record.specie.cl_soglia4 {
    cl.cl4 += 1;
  } else {
    cl.cl5 += 1;
  }
}

fn calculate_x2_a(classe: &ClassiEtaSpecieNISECI) -> f32 {
  let criterio_a: u8 = classe.get_x2_a_criterio_a();
  let criterio_b: u8 = classe.get_x2_a_criterio_b();

  if criterio_a == 1 && criterio_b == 3 {
    return 0.5;
  }
  if criterio_a == 1 {
    return 1.0;
  }
  if criterio_a == 2 && criterio_b == 3 {
    return 0.0;
  }
  if criterio_a == 2 {
    return 0.5
  }
  if criterio_a == 3 {
    return 0.0;
  }
  return 0.0;
}


