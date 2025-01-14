use crate::model::niseci::{CampionamentoNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI};



pub fn create_dummy_riferimento() -> RiferimentoNISECI {
  let importante_1 = SpecieNISECI {
    id: 1.to_string(),
    specie_attesa: true,
    nome: "Ciaccio ciaccensis".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0,
  };
  let importante_2 = SpecieNISECI {
    id: 2.to_string(),
    specie_attesa: true,
    nome: "Ciaccio sbribbrensis".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0,
  };
  let importante_3 = SpecieNISECI {
    id: 3.to_string(),
    specie_attesa: true,
    nome: "Ciaccio cozzensis".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0,
  };
  let normale_1 = SpecieNISECI {
    id: 4.to_string(),
    specie_attesa: true,
    nome: "Normus sempliciottum".to_string(),
    tipo_autoctono: 1,
    tipo_alloctono: 0
  };
  let normale_2 = SpecieNISECI {
    id: 5.to_string(),
    specie_attesa: true,
    nome: "Normus qualunquis".to_string(),
    tipo_autoctono: 1,
    tipo_alloctono: 0
  };
  let alloctono_1 = SpecieNISECI {
    id: 6.to_string(),
    specie_attesa: true,
    nome: "Disturbus infognatus".to_string(),
    tipo_autoctono: 0,
    tipo_alloctono: 1
  };
  let alloctono_2 = SpecieNISECI {
    id: 7.to_string(),
    specie_attesa: true,
    nome: "Disturbus sotterfugius".to_string(),
    tipo_autoctono: 0,
    tipo_alloctono: 1
  };
  let inatteso_1 = SpecieNISECI {
    id: 7.to_string(),
    specie_attesa: false,
    nome: "Sorprendo sorprendentes".to_string(),
    tipo_autoctono: 0,
    tipo_alloctono: 1
  };
  let inatteso_2 = SpecieNISECI {
    id: 8.to_string(),
    specie_attesa: false,
    nome: "Sorprendo improvvisus".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0
  };

  let mut elenco_specie = Vec::with_capacity(9);
  elenco_specie.push(importante_1);
  elenco_specie.push(importante_2);
  elenco_specie.push(importante_3);
  elenco_specie.push(normale_1);
  elenco_specie.push(normale_2);
  elenco_specie.push(alloctono_1);
  elenco_specie.push(alloctono_2);
  elenco_specie.push(inatteso_1);
  elenco_specie.push(inatteso_2);

  RiferimentoNISECI {
    elenco_specie: elenco_specie
  }
}


/// campionamento che ha dentro tutte le specie autoctone attese
/// in @create_dummy_riferimento.
/// Nel campionamento per futuri test si può modificare tutto
/// tranne le specie dei recordCsv,
/// che servono in @test_calculate_x1
pub fn create_dummy_campionamento_full() -> CampionamentoNISECI {
  let importante_1 = SpecieNISECI {
    id: 1.to_string(),
    specie_attesa: true,
    nome: "Ciaccio ciaccensis".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0,
  };
  let importante_2 = SpecieNISECI {
    id: 2.to_string(),
    specie_attesa: true,
    nome: "Ciaccio sbribbrensis".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0,
  };
  let importante_3 = SpecieNISECI {
    id: 3.to_string(),
    specie_attesa: true,
    nome: "Ciaccio cozzensis".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0,
  };
  let normale_1 = SpecieNISECI {
    id: 4.to_string(),
    specie_attesa: true,
    nome: "Normus sempliciottum".to_string(),
    tipo_autoctono: 1,
    tipo_alloctono: 0
  };
  let normale_2 = SpecieNISECI {
    id: 5.to_string(),
    specie_attesa: true,
    nome: "Normus qualunquis".to_string(),
    tipo_autoctono: 1,
    tipo_alloctono: 0
  };
  let alloctono_1 = SpecieNISECI {
    id: 6.to_string(),
    specie_attesa: true,
    nome: "Disturbus infognatus".to_string(),
    tipo_autoctono: 0,
    tipo_alloctono: 1
  };
  let alloctono_2 = SpecieNISECI {
    id: 7.to_string(),
    specie_attesa: true,
    nome: "Disturbus sotterfugius".to_string(),
    tipo_autoctono: 0,
    tipo_alloctono: 1
  };
  let inatteso_1 = SpecieNISECI {
    id: 7.to_string(),
    specie_attesa: false,
    nome: "Sorprendo sorprendentes".to_string(),
    tipo_autoctono: 0,
    tipo_alloctono: 1
  };
  let inatteso_2 = SpecieNISECI {
    id: 8.to_string(),
    specie_attesa: false,
    nome: "Sorprendo improvvisus".to_string(),
    tipo_autoctono: 2,
    tipo_alloctono: 0
  };

  let record_1 = RecordNISECI {
    specie: importante_1,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_2 = RecordNISECI {
    specie: importante_2,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_3 = RecordNISECI {
    specie: importante_3,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_4 = RecordNISECI {
    specie: normale_2,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_5 = RecordNISECI {
    specie: normale_1,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_6 = RecordNISECI {
    specie: inatteso_1,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_7 = RecordNISECI {
    specie: inatteso_2,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_8 = RecordNISECI {
    specie: alloctono_1,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };
  let record_9 = RecordNISECI {
    specie: alloctono_2,
    lunghezza: 5,
    peso: 5,
    passaggio_cattura: 1
  };

  let mut campionamento = Vec::with_capacity(9);
  campionamento.push(record_1);
  campionamento.push(record_2);
  campionamento.push(record_3);
  campionamento.push(record_4);
  campionamento.push(record_5);
  campionamento.push(record_6);
  campionamento.push(record_7);
  campionamento.push(record_8);
  campionamento.push(record_9);

  CampionamentoNISECI {
    campionamento: campionamento
  }
}

/// campionamento che non contiene tutte le specie
/// create in @create_dummy_riferimento
pub fn create_dummy_campionamento_chopped() -> CampionamentoNISECI {

  // uso il full campionamento e vado poi a togliere alcuni record
  let campionamento = create_dummy_campionamento_full();

  let mut chopped = campionamento.campionamento.clone();
  chopped.remove(1);

  CampionamentoNISECI {
    campionamento: chopped
  }
}
