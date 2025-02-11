use std::collections::{hash_map::Entry, HashMap};

use crate::model::niseci::{AnagraficaNISECI, CampionamentoNISECI, ClassiEtaSpecieNISECI, EsemplariPerCattura, RecordNISECI};

use super::linear_regression::{calculate_quantita_with_regression, Point};




pub fn calculate_x2(campionamento: &CampionamentoNISECI, anagrafica: &AnagraficaNISECI) -> Result<f32, Vec<String>> {
  let x2_a = match calculate_sommatoria_x2_a(campionamento) {
    Ok(x2_a) => x2_a,
    Err(errors) => return Err(errors),
  };
  let x2_b = match calculate_sommatoria_x2_b(campionamento, anagrafica){
    Ok(result) => result,
    Err(errors) => return Err(errors),
  };

  let mut specie_campionate_set:HashMap<String, bool> = HashMap::new();
  for cattura in &campionamento.campionamento {
    if cattura.specie.tipo_autoctono == 1 || cattura.specie.tipo_autoctono == 2 {
      match specie_campionate_set.entry(cattura.specie.id.clone()) {
          Entry::Occupied(_) => {},
          Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(true);
          },
      }
    }
  }

  let tot_specie_attese_trovate = specie_campionate_set.len();

  let result = (0.6 * x2_a + 0.4 * x2_b) / tot_specie_attese_trovate as f32;   
  
  Ok(result)
}

fn calculate_sommatoria_x2_a(c: &CampionamentoNISECI) -> Result<f32, Vec<String>> {

  // ad ogni specie associo le loro classi che andrò poi a riempire
  // ho controllato i campionamenti di andrea e trovto massimo 9 specie diverse
  // per sicurezza prealloco memoria per 10 classi di eta
  let mut classi_eta_map: HashMap<String, ClassiEtaSpecieNISECI> = HashMap::with_capacity(10);

  // riempo l'hashmap con solo le specie autoctone campionate
  for cattura in &c.campionamento {
    if cattura.specie.tipo_autoctono == 1 || cattura.specie.tipo_autoctono == 2 {
      match classi_eta_map.entry(cattura.specie.id.clone()) {
        Entry::Occupied(mut entry) => {
          entry.get_mut().update_classi_eta(&cattura);
        },
        Entry::Vacant(entry) => {
          entry.insert(ClassiEtaSpecieNISECI::new_cl_prevalorizzata(&cattura));
        }
      };
    } 
  }

  // ora la mappa è riempita e tutte le classi sono state riempite
  // si procede quindi al calcolo di x2 a per ogni specie campionata autoctona
  // e si va a fare la sommatoria dei parametri trovati

  let mut sommatoria_x2_a = 0.0;
  let mut errors: Vec<String> = Vec::with_capacity(classi_eta_map.len()); // prenoto ora e poi restringo dopo
  for (_key, classe) in &classi_eta_map {
    match calculate_x2_a(classe) {
      Ok(x2_a) => sommatoria_x2_a += x2_a,
      Err(error) => errors.push(error),
    }
  }

  if errors.len() > 0 {
    errors.shrink_to_fit();
    return Err(errors);
  }

  Ok(sommatoria_x2_a)
}

fn calculate_sommatoria_x2_b(c: &CampionamentoNISECI, anagrafica: &AnagraficaNISECI) -> Result<f32, Vec<String>> {
  let superficie = anagrafica.get_larghezza_media() * anagrafica.get_lunghezza_media();
  
  let mut esemplari_per_cattura_map: HashMap<String, EsemplariPerCattura> = HashMap::with_capacity(10);

  for cattura in &c.campionamento {
    if cattura.specie.tipo_autoctono == 1 || cattura.specie.tipo_autoctono == 2 {
      match esemplari_per_cattura_map.entry(cattura.specie.id.clone()) {
          Entry::Occupied(mut occupied_entry) => {
            occupied_entry.get_mut().fill_passaggio(cattura.passaggio_cattura);
          },
          Entry::Vacant(vacant_entry) => {
            vacant_entry.insert(EsemplariPerCattura::new_prevalorized(cattura.passaggio_cattura, &cattura.specie));
          },
      }
    }
  }

  // ora che abbiamo riempito la mappa con tutte le catture, possiamo andare
  // a calcolar x2b per ogni specie
  let mut sommatoria_x2_b = 0.0;
  let mut errors: Vec<String> = Vec::with_capacity(esemplari_per_cattura_map.len()); // prenoto ora e poi restringo dopo
  for (_key, catture) in &esemplari_per_cattura_map {
    match calculate_x2_b(catture, &superficie) {
        Ok(x2_b) => sommatoria_x2_b += x2_b,
        Err(err_mess) => errors.push(err_mess),
    }
  }

  // controllo se ci sono errori, se sì allora ritorno gli errori
  if errors.len() > 0 {
    errors.shrink_to_fit(); // restringo
    return Err(errors);
  }

  Ok(sommatoria_x2_b) // finally
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

/// fn wrapper del calcolo della struttura di una popolazione
pub(crate) fn calculate_x2_a(classe: &ClassiEtaSpecieNISECI) -> Result<f32, String> {
  return classe.calculate_struttura_popolazione();
}

pub(crate) fn calculate_x2_b(e: &EsemplariPerCattura, superficie: &f32) -> Result<f32, String> {

  match get_quantita_stimata(&e.mappa) {
    Ok(q_stimata) => {
      // calcolo densita stimata
      let densita_stimata = q_stimata as f32 / superficie;

      // trovo ora x2_b
      if densita_stimata > e.specie.dens_soglia2 {
        return Ok(1.0);
      }
      if densita_stimata > e.specie.dens_soglia1 {
        return Ok(0.5);
      }
      return Ok(0.0);

    },
    Err(err_message) => return Err(err_message)
  };
}

pub(crate) fn get_quantita_stimata(passaggi: &HashMap<u8, u32>) -> Result<u32, String> {
  if passaggi.len() == 1 {
    return Ok(*passaggi.get(passaggi.keys().min().unwrap()).unwrap());
  }
  if passaggi.len() == 2 && passaggi.contains_key(&1) && passaggi.contains_key(&2) {
    let c1 = *passaggi.get(&1).unwrap();
    let c2 = *passaggi.get(&2).unwrap();

    return calculate_passaggi_ripetuti(c1, c2);
  }
  return calculate_q_stimata_regression(passaggi);
}

pub(crate) fn calculate_passaggi_ripetuti(c1: u32, c2: u32) -> Result<u32, String> {
  
  match c1 == c2 {
    true => return Err("Quantita stimata con metodo dei PASSAGGI RIPETUTI: stesso numero di esemplari per entrambi i passaggi".to_string()),
    false => {},
  }

  let c = c1 + c2;
  let divisore = c2 as f32 / c1 as f32;
  
  let result = (c as f32 / (1.0 - divisore.powf(2.0))).round() as i32;


  match result > 0 {
    true => return Ok(result as u32),
    false => return Err("Quantita stimata con metodo dei PASSAGGI RIPETUTI è negativa".to_string()),
  }


}

pub(crate) fn calculate_q_stimata_regression(passaggi: &HashMap<u8, u32>) -> Result<u32, String> {
  let ultimo_passaggio = *passaggi.keys().max().unwrap();

  // dalla mappa non riesco a capire se ci siano o meno dei passaggi in cui non è stato trovato pesce
  // quindi mi creo un vettore che rappresenta i pesci trovati per ogni passaggio in ordine di passaggio
  let mut esemplari_per_passaggio = vec![0 as u32; ultimo_passaggio as usize];
  
  for (key, value) in passaggi {
    esemplari_per_passaggio[(*key - 1) as usize] = *value;
  }

  // ora creo i punti con x == esemplari catturati fino a quel passaggio 
  // e y == esmplari catturati in quel passaggio
  let mut current_tot = 0;
  let points: Vec<Point<i32>> = esemplari_per_passaggio
    .iter()
    .map(|esemplari: &u32| {
      current_tot += esemplari;
      return Point::new(current_tot as i32, *esemplari as i32);
    })
    .collect();

  return calculate_quantita_with_regression(points.as_slice());

}

