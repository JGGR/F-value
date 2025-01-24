use std::collections::{btree_map::{OccupiedEntry, VacantEntry}, hash_map::Entry, HashMap};

use crate::{model::niseci::{CampionamentoNISECI, ClassiEta, ClassiEtaSpecieNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI}, RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI};




pub fn calculate_x2(riferimento: &RiferimentoNISECI, campionamento: &CampionamentoNISECI) -> f32 {
  let x2_a = calculate_x2_a(riferimento, campionamento);


  0.0
}

pub fn calculate_x2_a(r: &RiferimentoNISECI, c: &CampionamentoNISECI) -> i32 {

  // ad ogni specie associo le loro classi che andrò poi a riempire
  // ho controllato i campionamenti di andrea e trovto massimo 9 specie diverse
  // per sicurezza prealloco memoria per 10 classi di eta
  let mut classi_eta_map: HashMap<String, ClassiEtaSpecieNISECI> = HashMap::with_capacity(10);

  // riempo l'hashmap con solo le specie autoctone campionate
  for c in &c.campionamento {
    if c.specie.tipo_autoctono == 1 || c.specie.tipo_autoctono == 2 {
      match classi_eta_map.entry(c.specie.id.clone()) {
        Entry::Occupied(mut entry) => {
          update_classi_eta(entry.get_mut(), &c);
        },
        Entry::Vacant(entry) => {
          entry.insert(ClassiEtaSpecieNISECI::new_cl_prevalorizzata(ClassiEta::find_classe_eta(c)));
        }
      };
    } 
  }

  // ora la mappa è riempita e tutte le classi sono state riempite
  // si procede quindi al calcolo di x2 per ogni specie campionata autoctona
  // e si va a fare la sommatoria dei parametri trovati




  0
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
    cl.cl4 += 1;
  }
}

