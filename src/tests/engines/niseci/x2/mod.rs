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

use crate::{
    domain::{
        location::Location,
        niseci::{
            AnagraficaNISECI, AreaNISECI, ClassiEtaSpecieNISECI, ComunitaNISECI,
            IdroEcoRegioneNISECI, TipoComunitaNISECI,
        },
    },
    engines::niseci::x2::calculate_x2,
    tests::test_utils::{
        create_massive_campionamento_ciacci, create_massive_campionamento_ciacci_2, get_ciaccio,
    },
};

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

    let (x2_a_criterio_b, _ad_juv) = classe.get_x2_a_criterio_b();
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

    let (x2_a_criterio_b, _ad_juv) = classe.get_x2_a_criterio_b();
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

    let (x2_a_criterio_b, _ad_juv) = classe.get_x2_a_criterio_b();
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

    let (x2_a_criterio_b, _ad_juv) = classe.get_x2_a_criterio_b();
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

    let (x2_a_criterio_b, _ad_juv) = classe.get_x2_a_criterio_b();
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

    let (x2_a_criterio_b, _ad_juv) = classe.get_x2_a_criterio_b();
    assert_eq!(3, x2_a_criterio_b)
}

#[test]
fn calculate_x2_test_1() {
    let campionamento = create_massive_campionamento_ciacci();

    let comunita = ComunitaNISECI {
        fonte: Some("hey".to_string()),
        numero_protocollo: None,
        tipo: TipoComunitaNISECI::Dm260_2010,
    };

    let anagrafica = AnagraficaNISECI {
        bacino_appartenenza: "dummy".to_string(),
        area: AreaNISECI::Alpina,
        codice_stazione: "Foo".to_string(),
        date_string: "Never".to_string(),
        comunita: comunita,
        idro_eco_regione: IdroEcoRegioneNISECI::AlpiCentroOrientali,
        larghezza_media_stazione: 1.0,
        lunghezza_media_stazione: 10.0,
        corpo_idrico: "canaletta".to_string(),
        posizione: Location {
            regione: "sardninaia".to_string(),
            provincia: "oristano".to_string(),
        },
    };

    let x2 = calculate_x2(&campionamento, &anagrafica, true);

    assert!(x2.is_ok());

    let (x2, _criteri_x2) = x2.unwrap();
    assert_eq!(Some(1.0), x2);

    // secondo test con valori alternativi

    let campionamento = create_massive_campionamento_ciacci_2();

    let comunita = ComunitaNISECI {
        fonte: Some("hey".to_string()),
        numero_protocollo: None,
        tipo: TipoComunitaNISECI::Dm260_2010,
    };

    let anagrafica = AnagraficaNISECI {
        bacino_appartenenza: "dummy".to_string(),
        area: AreaNISECI::Alpina,
        codice_stazione: "Foo".to_string(),
        date_string: "Never".to_string(),
        comunita: comunita,
        idro_eco_regione: IdroEcoRegioneNISECI::AlpiCentroOrientali,
        larghezza_media_stazione: 1.0,
        lunghezza_media_stazione: 10.0,
        corpo_idrico: "canaletta".to_string(),
        posizione: Location {
            regione: "sardninaia".to_string(),
            provincia: "oristano".to_string(),
        },
    };

    let x2 = calculate_x2(&campionamento, &anagrafica, true);

    assert!(x2.is_ok());
    let epsilon: f32 = 1e-6;
    let (x2, _criteri_x2) = x2.unwrap();
    assert!(x2.is_some());
    let x2_val = x2.unwrap();
    assert!((0.7 - x2_val).abs() < epsilon);
}
