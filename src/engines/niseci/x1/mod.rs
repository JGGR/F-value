use std::collections::HashMap;

use crate::model::niseci::{CampionamentoNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI};

pub fn calculate_x1(campionamento: &CampionamentoNISECI, riferimento: &RiferimentoNISECI) -> f32 {
  // n_i è il numero di specie autoctone di maggiore importanza ecologico-funzionale campionate
  // n_a è il numero di altre specie autoctone campionate
  // m_i è il numero di specie autoctone di maggiore importanza ecologico-funzionale attese
  // m_a è il numero di altre specie autoctone attese

 // creo un set delle specie campionate
  let mut specie_campionate_map: HashMap<String, &RecordNISECI> = HashMap::new();
  for camp in &campionamento.campionamento {
    if camp.specie.specie_attesa {
      specie_campionate_map.entry(camp.specie.id.clone()).or_insert(camp);
    }
  }
  let set_specie_campionate: Vec<&RecordNISECI> = specie_campionate_map.into_values().collect();
  let mut n_i: f32 = 0.0;
  let mut n_a: f32 = 0.0;
  for spec in set_specie_campionate {
    if spec.specie.tipo_autoctono == 1 { // tipo_autoctono == 1 allora specie importante
      n_a += 1.0;
    } else if spec.specie.tipo_autoctono == 2 {
      n_i += 1.0;
    }
  }

  // ora trovo le specie attese dal riferimento
  // per evitare doppioni, anche se non dovrebbero esserci,
  // ricavo il set delle specie attese
  let mut specie_attese_map: HashMap<String, &SpecieNISECI> = HashMap::new();
  for specie in &riferimento.elenco_specie {
    if specie.specie_attesa {
      specie_attese_map.entry(specie.id.clone()).or_insert(specie);
    }
  }
  let set_specie_attese: Vec<&SpecieNISECI> = specie_attese_map.into_values().collect();
  let mut m_i: f32 = 0.0;
  let mut m_a: f32 = 0.0;
  for spec in set_specie_attese {
    if spec.tipo_autoctono == 1 { // tipo_autoctono == 1 allora specie importante
      m_a += 1.0;
    } else if spec.tipo_autoctono == 2 {
      m_i += 1.0;
    }
  }

  // this is the formula use in the NISECI docs
  return (1.2 * n_i + 0.8 * n_a ) / (1.2 * m_i + 0.8 * m_a )
}

