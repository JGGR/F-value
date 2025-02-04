use std::collections::HashMap;

use crate::{engines::niseci::x2::{calculate_passaggi_ripetuti, calculate_q_stimata_regression, calculate_x2_b, get_quantita_stimata}, model::niseci::{ClassiEtaSpecieNISECI, EsemplariPerCattura}, tests::test_utils::get_ciaccio};




#[test]
fn calculate_q_stimata_regression_test_1() {
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
fn calculate_q_stimata_regression_test_err() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 50);
  passaggi.insert(2, 75);
  passaggi.insert(3, 100);

  let q_stimata = calculate_q_stimata_regression(&passaggi);

  assert!(q_stimata.is_err());
}

#[test]
fn calculate_passaggi_ripetuti_test_1() {

  let q_stimata = calculate_passaggi_ripetuti(30, 12);

  assert!(q_stimata.is_ok());
  assert_eq!(q_stimata.unwrap(), 50);
}

#[test]
fn calculate_passaggi_ripetuti_test_2() {

  let q_stimata = calculate_passaggi_ripetuti(30, 15);

  assert!(q_stimata.is_ok());
  assert_eq!(q_stimata.unwrap(), 60);
}

#[test]
fn calculate_passaggi_ripetuti_test_negative() {

  let q_stimata = calculate_passaggi_ripetuti(15, 30);

  assert!(q_stimata.is_err());
}

#[test]
fn calculate_passaggi_ripetuti_test_same_values() {

  let q_stimata = calculate_passaggi_ripetuti(30, 30);

  assert!(q_stimata.is_err());
}

#[test]
fn get_quantita_stimata_test_1() {
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
fn get_quantita_stimata_test_3() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 30);
  passaggi.insert(2, 12);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_ok());
  assert_eq!(50, q_stimata.unwrap());
}

#[test]
fn get_quantita_stimata_test_4() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 30);
  passaggi.insert(2, 15);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_ok());
  assert_eq!(60, q_stimata.unwrap());
}

#[test]
fn get_quantita_stimata_test_negative() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 15);
  passaggi.insert(2, 30);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_err());
}

#[test]
fn get_quantita_stimata_test_err() {
  let mut passaggi: HashMap<u8, u32> = HashMap::new();
  passaggi.insert(1, 50);
  passaggi.insert(2, 75);
  passaggi.insert(3, 100);

  let q_stimata = get_quantita_stimata(&passaggi);

  assert!(q_stimata.is_err());
}

#[test]
fn calculate_x2_b_test_1_0() {
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
fn calculate_x2_b_test_0_5() {
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
fn calculate_x2_b_test_0_0() {
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
fn calculate_x2_a_criterio_a_1() {
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
fn calculate_x2_a_criterio_a_2() {
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
fn calculate_x2_a_criterio_a_3() {
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
fn calculate_x2_a_criterio_b_cl_zero() {
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
fn calculate_x2_a_criterio_b_1() {
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
fn calculate_x2_a_criterio_b_2_uno() {
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
fn calculate_x2_a_criterio_b_2_due() {
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
fn calculate_x2_a_criterio_b_3_uno() {
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
fn calculate_x2_a_criterio_b_3_due() {
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

