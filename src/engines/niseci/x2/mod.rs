use std::collections::{hash_map::Entry, HashMap};

use crate::model::niseci::{AnagraficaNISECI, CampionamentoNISECI, ClassiEtaSpecieNISECI, EsemplariPerCattura, RecordNISECI, RiferimentoNISECI};

use super::linear_regression::{calculate_quantita_stimata, Point};




pub fn calculate_x2(riferimento: &RiferimentoNISECI, campionamento: &CampionamentoNISECI, anagrafica: &AnagraficaNISECI) -> f32 {
  let x2_a = calculate_sommatoria_x2_a(riferimento, campionamento);
  let x2_b = calculate_sommatoria_x2_b(riferimento, campionamento, anagrafica);

  0.0
}

fn calculate_sommatoria_x2_a(r: &RiferimentoNISECI, c: &CampionamentoNISECI) -> f32 {

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
  for (_key, classe) in &classi_eta_map {
    sommatoria_x2_a += calculate_x2_a(classe);
  }

  sommatoria_x2_a
}

fn calculate_sommatoria_x2_b(r: &RiferimentoNISECI, c: &CampionamentoNISECI, anagrafica: &AnagraficaNISECI) -> Result<f32, Vec<String>> {
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

fn calculate_x2_b(e: &EsemplariPerCattura, superficie: &f32) -> Result<f32, String> {

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

fn get_quantita_stimata(passaggi: &HashMap<u8, u32>) -> Result<i32, String> {
  if passaggi.len() == 1 {
    return Ok(passaggi.get(passaggi.keys().min().unwrap()).unwrap().clone() as i32); // brutto anch qua
  }
  if passaggi.len() == 2 && passaggi.contains_key(&1) && passaggi.contains_key(&2) {
    let c1 = passaggi.get(&1).unwrap().clone();
    let c2 = passaggi.get(&2).unwrap().clone();

    return Ok(calculate_passaggi_ripetuti(c1, c2));
  }
  return calculate_q_stimata_regression(passaggi);
}

fn calculate_passaggi_ripetuti(c1: u32, c2: u32) -> i32 {
  let z = 1.0 - (c2 as f32 / c1 as f32);
  let c = c1 + c2;
  
  (c as f32 / (1.0 - z.powf(2.0))) as i32
}

fn calculate_q_stimata_regression(passaggi: &HashMap<u8, u32>) -> Result<i32, String> {
  let ultimo_passaggio = passaggi.keys().max().unwrap().clone(); // brutta roba

  // dalla mappa non riesco a capire se ci siano o meno sei passaggi in cui non è stato trovato pesce
  // quindi mi creo un vettore che rappresenta i pesci trovati per ogni passaggio in ordine di passaggio
  let mut esemplari_per_passaggio = vec![0 as i32; ultimo_passaggio as usize];
  for (key, value) in passaggi {
    esemplari_per_passaggio.insert(key.clone() as usize, value.clone() as i32);
  }

  // ora creo i punti con x == esemplari catturati fino a quel passaggio 
  // e y == esmplari catturati in quel passaggio
  let mut current_tot = 0;
  let points: Vec<Point<i32>> = esemplari_per_passaggio
    .iter()
    .map(|esemplari: &i32| {
      current_tot += esemplari;
      return Point::new(current_tot, esemplari.clone());
    })
    .collect();

  return calculate_quantita_stimata(points.as_slice());

}

