use crate::{engines::niseci::x3::calculate_x3, model::niseci::RecordNISECI, tests::test_utils::{create_massive_campionamento_ciacci_1, create_massive_campionamento_ciacci_2, create_massive_campionamento_ciacci_con_bronzi_strutt, create_massive_campionamento_ciacci_con_tappi_destrutt, create_massive_campionamento_ciacci_con_tappi_mediam_strutt, create_massive_campionamento_ciacci_con_tappi_strutt, create_massive_campionamento_ciacci_con_trocchi_strutt, create_massive_campionamento_ciacci_solo_autoctoni_1, get_ciaccio}};




#[test]
fn calculate_x3_assenza_specie_aliene() {
  let c = create_massive_campionamento_ciacci_solo_autoctoni_1();
  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 1.0);
}

#[test]
fn calculate_x3_un_trocchio() {
  let c = create_massive_campionamento_ciacci_2();
  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.75);
}

#[test]
fn calculate_x3_alieni_magg_uguale_autoctoni() {
  let c = create_massive_campionamento_ciacci_con_trocchi_strutt();
  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.0);
}

#[test]
fn calculate_x3_alieni_tipo_1_strutt() {
  let mut c = create_massive_campionamento_ciacci_con_trocchi_strutt();

    let ciaccio = RecordNISECI {
      specie: get_ciaccio(),
      passaggio_cattura: 2,
      lunghezza: 2,
      peso: 2
    };
    c.campionamento.push(ciaccio);

  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.0);
}

#[test]
fn calculate_x3_alieni_tipo_2_strutt() {
  let mut c = create_massive_campionamento_ciacci_con_bronzi_strutt();

    let ciaccio = RecordNISECI {
      specie: get_ciaccio(),
      passaggio_cattura: 2,
      lunghezza: 2,
      peso: 2
    };
    c.campionamento.push(ciaccio);

  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.25);
}


#[test]
fn calculate_x3_alieni_tipo_3_strutt() {
  let mut c = create_massive_campionamento_ciacci_con_tappi_strutt();

  let ciaccio = RecordNISECI {
    specie: get_ciaccio(),
    passaggio_cattura: 2,
    lunghezza: 2,
    peso: 2
  };
  c.campionamento.push(ciaccio);

  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.375);
}

#[test]
fn calculate_x3_alieni_tipo_3_destrutt() {
  let mut c = create_massive_campionamento_ciacci_con_tappi_destrutt();

  let ciaccio = RecordNISECI {
    specie: get_ciaccio(),
    passaggio_cattura: 2,
    lunghezza: 2,
    peso: 2
  };
  c.campionamento.push(ciaccio);

  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.875);
}

#[test]
fn calculate_x3_alieni_tipo_3_meidam_strutt() {
  let mut c = create_massive_campionamento_ciacci_con_tappi_mediam_strutt();

  let ciaccio = RecordNISECI {
    specie: get_ciaccio(),
    passaggio_cattura: 2,
    lunghezza: 2,
    peso: 2
  };
  c.campionamento.push(ciaccio);

  let x3 = calculate_x3(&c);

  assert!(x3.is_ok());
  assert_eq!(x3.unwrap(), 0.625);
}


