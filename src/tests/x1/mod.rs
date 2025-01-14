use crate::engines::niseci::x1::calculate_x1;
use crate::tests::test_utils::{create_dummy_campionamento_chopped, create_dummy_campionamento_full, create_dummy_riferimento};



/// in questo test il campionamento ha
/// tutte le specie attese dal riferimento
#[test]
fn test_calculate_x1_full_campionamento() {
  let riferimento = create_dummy_riferimento();
  let campionamento = create_dummy_campionamento_full();

  assert_eq!(calculate_x1(&campionamento, &riferimento), 1.0);
}


/// in questo test il campionamento non
/// non possiede tutte le specie del riferimento
#[test]
fn test_calculate_x1_mixed_campionamento() {
  let riferimento = create_dummy_riferimento();
  let campionamento = create_dummy_campionamento_chopped();
  let x1_calcolato = calculate_x1(&campionamento, &riferimento);
  let x1_atteso = 10.0 / 13.0;
  assert!((x1_calcolato - x1_atteso).abs() <= f32::EPSILON);
}



