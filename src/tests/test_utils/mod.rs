// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2025 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::domain::{
    location::Location,
    niseci::{
        AnagraficaNISECI, AreaNISECI, CampionamentoNISECI, ComunitaNISECI, IdroEcoRegioneNISECI,
        RecordNISECI, RiferimentoNISECI, SpecieNISECI, TipoComunitaNISECI,
    },
};

pub fn create_dummy_anagrafica() -> AnagraficaNISECI {
    return AnagraficaNISECI {
        comunita: ComunitaNISECI {
            tipo: TipoComunitaNISECI::Redatta,
            fonte: None,
            numero_protocollo: None,
        },
        codice_stazione: "foo".to_string(),
        date_string: "foo".to_string(),
        area: AreaNISECI::Alpina,
        corpo_idrico: "foo".to_string(),
        bacino_appartenenza: "foo".to_string(),
        idro_eco_regione: IdroEcoRegioneNISECI::Toscana,
        posizione: Location {
            regione: "foo".to_string(),
            provincia: "foo".to_string(),
        },
        lunghezza_media_stazione: 0.0,
        larghezza_media_stazione: 0.0,
    };
}

pub fn create_dummy_riferimento() -> RiferimentoNISECI {
    let importante_1 = SpecieNISECI {
        id: 1.to_string(),
        specie_attesa: true,
        nome: "Ciaccio ciaccensis".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let importante_2 = SpecieNISECI {
        id: 2.to_string(),
        specie_attesa: true,
        nome: "Ciaccio sbribbrensis".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let importante_3 = SpecieNISECI {
        id: 3.to_string(),
        specie_attesa: true,
        nome: "Ciaccio cozzensis".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let normale_1 = SpecieNISECI {
        id: 4.to_string(),
        specie_attesa: true,
        nome: "Normus sempliciottum".to_string(),
        tipo_autoctono: 1,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let normale_2 = SpecieNISECI {
        id: 5.to_string(),
        specie_attesa: true,
        nome: "Normus qualunquis".to_string(),
        tipo_autoctono: 1,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let alloctono_1 = SpecieNISECI {
        id: 6.to_string(),
        specie_attesa: true,
        nome: "Disturbus infognatus".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let alloctono_2 = SpecieNISECI {
        id: 7.to_string(),
        specie_attesa: true,
        nome: "Disturbus sotterfugius".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let inatteso_1 = SpecieNISECI {
        id: 7.to_string(),
        specie_attesa: false,
        nome: "Sorprendo sorprendentes".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let inatteso_2 = SpecieNISECI {
        id: 8.to_string(),
        specie_attesa: false,
        nome: "Sorprendo improvvisus".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
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
        elenco_specie: elenco_specie,
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
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let importante_2 = SpecieNISECI {
        id: 2.to_string(),
        specie_attesa: true,
        nome: "Ciaccio sbribbrensis".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let importante_3 = SpecieNISECI {
        id: 3.to_string(),
        specie_attesa: true,
        nome: "Ciaccio cozzensis".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let normale_1 = SpecieNISECI {
        id: 4.to_string(),
        specie_attesa: true,
        nome: "Normus sempliciottum".to_string(),
        tipo_autoctono: 1,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let normale_2 = SpecieNISECI {
        id: 5.to_string(),
        specie_attesa: true,
        nome: "Normus qualunquis".to_string(),
        tipo_autoctono: 1,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let alloctono_1 = SpecieNISECI {
        id: 6.to_string(),
        specie_attesa: true,
        nome: "Disturbus infognatus".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let alloctono_2 = SpecieNISECI {
        id: 7.to_string(),
        specie_attesa: true,
        nome: "Disturbus sotterfugius".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let inatteso_1 = SpecieNISECI {
        id: 7.to_string(),
        specie_attesa: false,
        nome: "Sorprendo sorprendentes".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };
    let inatteso_2 = SpecieNISECI {
        id: 8.to_string(),
        specie_attesa: false,
        nome: "Sorprendo improvvisus".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 1,
        cl_soglia2: 2,
        cl_soglia3: 3,
        cl_soglia4: 4,
        ad_juv_soglia1: 1.0,
        ad_juv_soglia2: 2.0,
        ad_juv_soglia3: 3.0,
        ad_juv_soglia4: 4.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };

    let record_1 = RecordNISECI {
        specie: importante_1,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_2 = RecordNISECI {
        specie: importante_2,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_3 = RecordNISECI {
        specie: importante_3,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_4 = RecordNISECI {
        specie: normale_2,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_5 = RecordNISECI {
        specie: normale_1,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_6 = RecordNISECI {
        specie: inatteso_1,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_7 = RecordNISECI {
        specie: inatteso_2,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_8 = RecordNISECI {
        specie: alloctono_1,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
    };
    let record_9 = RecordNISECI {
        specie: alloctono_2,
        lunghezza: 5,
        peso: 5,
        passaggio_cattura: 1,
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
        campionamento: campionamento,
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
        campionamento: chopped,
    }
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
/// aggiungo in C2 anche una specie alloctono per testare la conta delle specie autoctone trovate
/// x2a qua vale 1.0 e x2b 1.0
pub fn create_massive_campionamento_ciacci() -> CampionamentoNISECI {
    let mut c = create_massive_campionamento_ciacci_solo_autoctoni_1();

    // pesce alloctono in C2
    let trocchio = RecordNISECI {
        specie: get_trocchio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(trocchio);

    c
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl4
///   -> 5 ciacci cl1
/// aggiungo in C2 anche una specie alloctono per testare la conta delle specie autoctone trovate
/// x2a qua vale 0.5 e x2b 1.0
pub fn create_massive_campionamento_ciacci_2() -> CampionamentoNISECI {
    let mut c = create_massive_campionamento_ciacci_solo_autoctoni_2();

    // pesce alloctono in C2
    let trocchio = RecordNISECI {
        specie: get_trocchio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(trocchio);

    c
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl4
///   -> 5 ciacci cl1
/// non sono presenti specie alloctone
pub fn create_massive_campionamento_ciacci_solo_autoctoni_2() -> CampionamentoNISECI {
    let mut ciaccio = get_ciaccio();
    ciaccio.dens_soglia1 = 3.0;
    ciaccio.dens_soglia2 = 5.0;

    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 ciacci cl5 in c1
    let ciaccio_cl5_c1 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 13,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl5_c1.clone());
    }

    // 10 ciacci cl4 in c1
    let ciaccio_cl4_c1 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 10,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl4_c1.clone());
    }

    // 10 ciacci cl3 in c1
    let ciaccio_cl3_c1 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 7,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl3_c1.clone());
    }

    // 10 ciacci cl4 in c2
    let ciaccio_cl4_c2 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 10,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl4_c2.clone());
    }

    // 5 ciacci cl1 in c2
    let ciaccio_cl1_c2 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 2,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..5 {
        campionamento.push(ciaccio_cl1_c2.clone());
    }

    CampionamentoNISECI {
        campionamento: campionamento,
    }
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
/// non sono presenti specie alloctone
pub fn create_massive_campionamento_ciacci_solo_autoctoni_1() -> CampionamentoNISECI {
    let mut ciaccio = get_ciaccio();
    ciaccio.dens_soglia1 = 3.0;
    ciaccio.dens_soglia2 = 5.0;

    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 ciacci cl5 in c1
    let ciaccio_cl5_c1 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 13,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl5_c1.clone());
    }

    // 10 ciacci cl4 in c1
    let ciaccio_cl4_c1 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 10,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl4_c1.clone());
    }

    // 10 ciacci cl3 in c1
    let ciaccio_cl3_c1 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 7,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl3_c1.clone());
    }

    // 10 ciacci cl2 in c2
    let ciaccio_cl2_c2 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(ciaccio_cl2_c2.clone());
    }

    // 5 ciacci cl1 in c2
    let ciaccio_cl1_c2 = RecordNISECI {
        specie: ciaccio.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..5 {
        campionamento.push(ciaccio_cl1_c2.clone());
    }

    CampionamentoNISECI {
        campionamento: campionamento,
    }
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 trocchi cl5
///   -> 10 trocchi cl4
///   -> 10 trocchi cl3
/// -> C2
///   -> 10 trocchi cl2
///   -> 5 trocchi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_1_strutt() -> CampionamentoNISECI {
    let mut trocchio = get_trocchio();
    trocchio.dens_soglia1 = 3.0;
    trocchio.dens_soglia2 = 5.0;

    create_campionamento_strutturato_data_una_specie(trocchio)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 bronzi cl5
///   -> 10 bronzi cl4
///   -> 10 bronzi cl3
/// -> C2
///   -> 10 bronzi cl2
///   -> 5 bronzi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_2_strutt() -> CampionamentoNISECI {
    let mut bronzo = get_bronzo();
    bronzo.dens_soglia1 = 3.0;
    bronzo.dens_soglia2 = 5.0;

    create_campionamento_strutturato_data_una_specie(bronzo)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 tappi cl5
///   -> 10 tappi cl4
///   -> 10 tappi cl3
/// -> C2
///   -> 10 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_3_strutt() -> CampionamentoNISECI {
    let mut tappo = get_tappo();
    tappo.dens_soglia1 = 3.0;
    tappo.dens_soglia2 = 5.0;

    create_campionamento_strutturato_data_una_specie(tappo)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 5 tappi cl2
pub fn create_massive_campionamento_solo_tipo_alloctono_3_destrutt() -> CampionamentoNISECI {
    let mut tappo = get_tappo();
    tappo.dens_soglia1 = 3.0;
    tappo.dens_soglia2 = 5.0;

    create_campionamento_destrutturato_data_una_specie(tappo)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 5 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_3_mediam_strutt() -> CampionamentoNISECI {
    let mut tappo = get_tappo();
    tappo.dens_soglia1 = 3.0;
    tappo.dens_soglia2 = 5.0;

    create_campionamento_mediam_strutturato_data_una_specie(tappo)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 10 trocchi cl5
///   -> 10 trocchi cl4
///   -> 10 trocchi cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 10 trocchi cl2
///   -> 5 trocchi cl1
pub fn create_massive_campionamento_ciacci_con_trocchi_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_trocchi = create_massive_campionamento_solo_tipo_alloctono_1_strutt();

    c_trocchi.campionamento.append(&mut c_ciacci.campionamento);

    c_trocchi
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 10 bronzi cl5
///   -> 10 bronzi cl4
///   -> 10 bronzi cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 10 bronzi cl2
///   -> 5 bronzi cl1
pub fn create_massive_campionamento_ciacci_con_bronzi_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_bronzi = create_massive_campionamento_solo_tipo_alloctono_2_strutt();

    c_bronzi.campionamento.append(&mut c_ciacci.campionamento);

    c_bronzi
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 10 tappi cl5
///   -> 10 tappi cl4
///   -> 10 tappi cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 10 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_ciacci_con_tappi_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_tappi = create_massive_campionamento_solo_tipo_alloctono_3_strutt();

    c_tappi.campionamento.append(&mut c_ciacci.campionamento);

    c_tappi
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 5 tappi cl2
pub fn create_massive_campionamento_ciacci_con_tappi_destrutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_tappi = create_massive_campionamento_solo_tipo_alloctono_3_destrutt();

    c_tappi.campionamento.append(&mut c_ciacci.campionamento);

    c_tappi
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 5 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_ciacci_con_tappi_mediam_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_tappi = create_massive_campionamento_solo_tipo_alloctono_3_mediam_strutt();

    c_tappi.campionamento.append(&mut c_ciacci.campionamento);

    c_tappi
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 esemplari cl5
///   -> 10 esemplari cl4
///   -> 10 esemplari cl3
/// -> C2
///   -> 10 esemplari cl2
///   -> 5 esemplari cl1
fn create_campionamento_strutturato_data_una_specie(specie: SpecieNISECI) -> CampionamentoNISECI {
    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 esemplari cl5 in c1
    let cl5_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 13,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl5_c1.clone());
    }

    // 10 esemplari cl4 in c1
    let cl4_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 10,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl4_c1.clone());
    }

    // 10 esemplari cl3 in c1
    let cl3_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 7,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl3_c1.clone());
    }

    // 10 esemplari cl2 in c2
    let cl2_c2 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl2_c2.clone());
    }

    // 5 esemplari cl1 in c2
    let cl1_c2 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..5 {
        campionamento.push(cl1_c2.clone());
    }

    CampionamentoNISECI {
        campionamento: campionamento,
    }
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 esemplari cl5
///   -> 20 esemplari cl4
/// -> C2
///   -> 5 esemplari cl2
fn create_campionamento_destrutturato_data_una_specie(specie: SpecieNISECI) -> CampionamentoNISECI {
    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 esemplari cl5 in c1
    let cl5_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 13,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl5_c1.clone());
    }

    // 10 esemplari cl4 in c1
    let cl4_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 10,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..20 {
        campionamento.push(cl4_c1.clone());
    }

    // 10 esemplari cl2 in c2
    let cl2_c2 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl2_c2.clone());
    }

    CampionamentoNISECI {
        campionamento: campionamento,
    }
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 esemplari cl5
///   -> 20 esemplari cl4
/// -> C2
///   -> 5 esemplari cl2
///   -> 5 esemplari cl1
fn create_campionamento_mediam_strutturato_data_una_specie(
    specie: SpecieNISECI,
) -> CampionamentoNISECI {
    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 esemplari cl5 in c1
    let cl5_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 13,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl5_c1.clone());
    }

    // 10 esemplari cl4 in c1
    let cl4_c1 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 10,
        passaggio_cattura: 1,
        peso: 10,
    };
    for _ in 0..20 {
        campionamento.push(cl4_c1.clone());
    }

    // 10 esemplari cl2 in c2
    let cl2_c2 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..10 {
        campionamento.push(cl2_c2.clone());
    }

    // 5 esemplari cl1 in c2
    let cl1_c2 = RecordNISECI {
        specie: specie.clone(),
        lunghezza: 4,
        passaggio_cattura: 2,
        peso: 10,
    };
    for _ in 0..5 {
        campionamento.push(cl1_c2.clone());
    }

    CampionamentoNISECI {
        campionamento: campionamento,
    }
}

/// id == 1
pub fn get_ciaccio() -> SpecieNISECI {
    SpecieNISECI {
        id: 1.to_string(),
        specie_attesa: true,
        nome: "Ciaccio ciaccensis".to_string(),
        tipo_autoctono: 2,
        tipo_alloctono: 0,
        cl_soglia1: 3,
        cl_soglia2: 6,
        cl_soglia3: 9,
        cl_soglia4: 12,
        ad_juv_soglia1: 0.5,
        ad_juv_soglia2: 0.67,
        ad_juv_soglia3: 1.5,
        ad_juv_soglia4: 2.0,
        dens_soglia1: 5.0,
        dens_soglia2: 10.0,
    }
}

/// id == 2
pub fn get_trocchio() -> SpecieNISECI {
    SpecieNISECI {
        id: 2.to_string(),
        specie_attesa: true,
        nome: "Trocchio trocchiensis".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 1,
        cl_soglia1: 3,
        cl_soglia2: 6,
        cl_soglia3: 9,
        cl_soglia4: 12,
        ad_juv_soglia1: 0.5,
        ad_juv_soglia2: 0.67,
        ad_juv_soglia3: 1.5,
        ad_juv_soglia4: 2.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    }
}

/// id == 3
pub fn get_bronzo() -> SpecieNISECI {
    SpecieNISECI {
        id: 3.to_string(),
        specie_attesa: true,
        nome: "Bronzo bronzensis".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 2,
        cl_soglia1: 3,
        cl_soglia2: 6,
        cl_soglia3: 9,
        cl_soglia4: 12,
        ad_juv_soglia1: 0.5,
        ad_juv_soglia2: 0.67,
        ad_juv_soglia3: 1.5,
        ad_juv_soglia4: 2.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    }
}

/// id == 4
pub fn get_tappo() -> SpecieNISECI {
    SpecieNISECI {
        id: 4.to_string(),
        specie_attesa: true,
        nome: "Tappo sugheribus".to_string(),
        tipo_autoctono: 0,
        tipo_alloctono: 3,
        cl_soglia1: 3,
        cl_soglia2: 6,
        cl_soglia3: 9,
        cl_soglia4: 12,
        ad_juv_soglia1: 0.5,
        ad_juv_soglia2: 0.67,
        ad_juv_soglia3: 1.5,
        ad_juv_soglia4: 2.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    }
}
