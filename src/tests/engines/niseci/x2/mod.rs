use std::collections::HashMap;

use crate::{engines::niseci::x2::{calculate_passaggi_ripetuti, calculate_q_stimata_regression, calculate_x2, calculate_x2_a, calculate_x2_b, get_quantita_stimata}, model::{location::Location, niseci::{AnagraficaNISECI, ClassiEtaSpecieNISECI, ComunitaNISECI, EsemplariPerCattura, IdroEcoRegioneNISECI, TipoComunitaNISECI}}, tests::test_utils::{create_massive_campionamento_ciacci_1, create_massive_campionamento_ciacci_2, get_ciaccio}};




#[test]
fn calcola_q_stimata_regression() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 70);
  passaggi.insert(2, 60);
  passaggi.insert(3, 20);
  passaggi.insert(4, 10);

  let q_stimata = calculate_q_stimata_regression(&passaggi);

  assert!(q_stimata.is_ok());
  assert_eq!(190, q_stimata.unwrap());
}

#[test]
fn calcola_q_stimata_regression_err() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 50);
  passaggi.insert(2, 75);
  passaggi.insert(3, 100);

  let q_stimata = calculate_q_stimata_regression(&passaggi);

  assert!(q_stimata.is_err());
}

#[test]
fn calcola_passaggi_ripetuti() {

  let q_stimata_1 = calculate_passaggi_ripetuti(30, 12);
  
  assert!(q_stimata_1.is_ok());
  assert_eq!(q_stimata_1.unwrap(), 50);

  let q_stimata_2 = calculate_passaggi_ripetuti(30, 15);
  
  assert!(q_stimata_2.is_ok());
  assert_eq!(q_stimata_2.unwrap(), 60);
}

#[test]
fn calcola_passaggi_ripetuti_err_negative() {

  let q_stimata = calculate_passaggi_ripetuti(15, 30);

  assert!(q_stimata.is_err());
}

#[test]
fn calcola_passaggi_ripetuti_err_same_values() {

  let q_stimata = calculate_passaggi_ripetuti(30, 30);

  assert!(q_stimata.is_err());
}

#[test]
fn get_quantita_stimata_regression() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 70);
  passaggi.insert(2, 60);
  passaggi.insert(3, 20);
  passaggi.insert(4, 10);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_ok());
  assert_eq!(190, q_stimata.unwrap());
}

#[test]
fn get_quantita_stimata_passaggi_ripetuti() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 30);
  passaggi.insert(2, 12);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_ok());
  assert_eq!(50, q_stimata.unwrap());
  
  let mut passaggi2: HashMap<u8, u32> = HashMap::new();
  passaggi2.insert(1, 30);
  passaggi2.insert(2, 15);
  
  let q_stimata2 = get_quantita_stimata(&passaggi2);
  
  assert!(q_stimata2.is_ok());
  assert_eq!(60, q_stimata2.unwrap());
}

#[test]
fn get_quantita_stimata_err_passaggi_ripetuti_negative() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 15);
  passaggi.insert(2, 30);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_err());
}

#[test]
fn get_quantita_stimata_err_progression() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 50);
  passaggi.insert(2, 75);
  passaggi.insert(3, 100);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_err());
}

#[test]
fn calculate_x2_b_buona() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 30);
  passaggi.insert(2, 15);

  let specie = get_ciaccio();

  let esemplari_per_cattura = EsemplariPerCattura {
    specie: specie,
    mappa: passaggi
  };

  let x2_b = calculate_x2_b(&esemplari_per_cattura, &2.0);

  assert!(x2_b.is_ok());
  assert_eq!(x2_b.unwrap(), 1.0)

}

#[test]
fn calculate_x2_b_test_intermedia() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 30);
  passaggi.insert(2, 15);

  let mut specie = get_ciaccio();
  specie.dens_soglia1 = 20.0;
  specie.dens_soglia2 = 30.0;

  let esemplari_per_cattura = EsemplariPerCattura {
    specie: specie,
    mappa: passaggi
  };

  let x2_b = calculate_x2_b(&esemplari_per_cattura, &2.0);

  assert!(x2_b.is_ok());
  assert_eq!(x2_b.unwrap(), 0.5)

}

#[test]
fn calculate_x2_b_test_scarsa() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 30);
  passaggi.insert(2, 15);

  let mut specie = get_ciaccio();
  specie.dens_soglia1 = 30.0;
  specie.dens_soglia2 = 40.0;

  let esemplari_per_cattura = EsemplariPerCattura {
    specie: specie,
    mappa: passaggi
  };

  let x2_b = calculate_x2_b(&esemplari_per_cattura, &2.0);

  assert!(x2_b.is_ok());
  assert_eq!(x2_b.unwrap(), 0.0)

}

#[test]
fn calculate_x2_a_criterio_a_5_classi_valorizzate() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 1,
    cl2: 1,
    cl3: 1,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_a = classe.get_x2_a_criterio_a();
  assert_eq!(1, x2_a_criterio_a)

}

#[test]
fn calculate_x2_a_criterio_a_3_classi_valorizzate() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 1,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_a = classe.get_x2_a_criterio_a();
  assert_eq!(2, x2_a_criterio_a)
}

#[test]
fn calculate_x2_a_criterio_a_2_classi_valorizzate() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 0,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_a = classe.get_x2_a_criterio_a();
  assert_eq!(3, x2_a_criterio_a)

}

#[test]
fn calculate_x2_a_criterio_b_zero_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 0,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_b = classe.get_x2_a_criterio_b();
  assert_eq!(3, x2_a_criterio_b)

}

#[test]
fn calculate_x2_a_criterio_b_1_bilanciato() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 1,
    cl3: 1,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_b = classe.get_x2_a_criterio_b();
  assert_eq!(1, x2_a_criterio_b)

}

#[test]
fn calculate_x2_a_criterio_b_2_medio_sbilanciato_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 1,
    cl3: 1,
    cl4: 2,
    cl5: 2,
  };

  let x2_a_criterio_b = classe.get_x2_a_criterio_b();
  assert_eq!(2, x2_a_criterio_b)

}

#[test]
fn calculate_x2_a_criterio_b_2_medio_sbilanciato_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 2,
    cl3: 2,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_b = classe.get_x2_a_criterio_b();
  assert_eq!(2, x2_a_criterio_b)

}

#[test]
fn calculate_x2_a_criterio_b_3_molto_sbilanciato_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 1,
    cl3: 1,
    cl4: 3,
    cl5: 3,
  };

  let x2_a_criterio_b = classe.get_x2_a_criterio_b();
  assert_eq!(3, x2_a_criterio_b)

}

#[test]
fn calculate_x2_a_criterio_b_3_molto_sbilanciato_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 3,
    cl3: 3,
    cl4: 1,
    cl5: 1,
  };

  let x2_a_criterio_b = classe.get_x2_a_criterio_b();
  assert_eq!(3, x2_a_criterio_b)

}

#[test]
fn calculate_x2_a_test_ca_1_cb_3_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 3,
    cl3: 3,
    cl4: 1,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.5, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_1_cb_3_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 1,
    cl3: 1,
    cl4: 3,
    cl5: 3,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.5, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_1_cb_2_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 1,
    cl3: 1,
    cl4: 2,
    cl5: 2,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(1.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_1_cb_2_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 2,
    cl3: 2,
    cl4: 1,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(1.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_1_cb_1() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 1,
    cl3: 1,
    cl4: 1,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(1.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_2_cb_1() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 2,
    cl4: 1,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.5, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_2_cb_2_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 2,
    cl4: 2,
    cl5: 2,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.5, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_2_cb_2_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 2,
    cl3: 2,
    cl4: 2,
    cl5: 0,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.5, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_2_cb_3_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 2,
    cl4: 3,
    cl5: 3,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_2_cb_3_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 6,
    cl4: 1,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_3_cb_3_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 1,
    cl4: 0,
    cl5: 6,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_3_cb_3_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 6,
    cl4: 0,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_3_cb_1() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 1,
    cl4: 0,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_3_cb_2_giovani() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 2,
    cl4: 0,
    cl5: 1,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_3_cb_2_adulti() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 0,
    cl2: 0,
    cl3: 1,
    cl4: 0,
    cl5: 2,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.0, x2_a.unwrap());

}

#[test]
fn calculate_x2_a_test_ca_1_cb_3_adulti_cl1_valorizzato() {
  let classe = ClassiEtaSpecieNISECI {
    specie: get_ciaccio(),
    cl1: 5,
    cl2: 0,
    cl3: 10,
    cl4: 20,
    cl5: 10,
  };

  let x2_a = calculate_x2_a(&classe);

  assert!(x2_a.is_ok());
  assert_eq!(0.5, x2_a.unwrap());

}


#[test]
fn calculate_x2_test_1() {
  let campionamento = create_massive_campionamento_ciacci_1();

  let comunita = ComunitaNISECI {
    fonte: Some("hey".to_string()),
    numero_protocollo: None,
    tipo: TipoComunitaNISECI::Dm260_2010
  };

  let anagrafica = AnagraficaNISECI {
    bacino_appartenenza: "dummy".to_string(),
    codice_stazione: 1,
    comunita: comunita,
    idro_eco_regione: IdroEcoRegioneNISECI::AlpiCentroOrientali,
    larghezza_media_stazione: 1.0,
    lunghezza_media_stazione: 10.0,
    nome_fiume: "canaletta".to_string(),
    posizione: Location {
      regione: "sardninaia".to_string(),
      provincia: "oristano".to_string()
    }
  };


  let x2 = calculate_x2(&campionamento, &anagrafica);

  assert!(x2.is_ok());
  assert_eq!(1.0, x2.unwrap());

  // secondo test con valori alternativi

  let campionamento = create_massive_campionamento_ciacci_2();
  
  let comunita = ComunitaNISECI {
    fonte: Some("hey".to_string()),
    numero_protocollo: None,
    tipo: TipoComunitaNISECI::Dm260_2010
  };
  
  let anagrafica = AnagraficaNISECI {
    bacino_appartenenza: "dummy".to_string(),
    codice_stazione: 1,
    comunita: comunita,
    idro_eco_regione: IdroEcoRegioneNISECI::AlpiCentroOrientali,
    larghezza_media_stazione: 1.0,
    lunghezza_media_stazione: 10.0,
    nome_fiume: "canaletta".to_string(),
    posizione: Location {
      regione: "sardninaia".to_string(),
      provincia: "oristano".to_string()
    }
  };
  
  
  let x2 = calculate_x2(&campionamento, &anagrafica);
  
  assert!(x2.is_ok());
  let epsilon: f32 = 1e-6;
  assert!((0.7 - x2.unwrap()).abs() < epsilon);
}

