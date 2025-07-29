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
    domain::niseci::RecordNISECI,
    engines::niseci::x3::calculate_x3,
    tests::test_utils::{
        create_massive_campionamento_ciacci_2,
        create_massive_campionamento_ciacci_con_bronzi_strutt,
        create_massive_campionamento_ciacci_con_tappi_destrutt,
        create_massive_campionamento_ciacci_con_tappi_mediam_strutt,
        create_massive_campionamento_ciacci_con_tappi_strutt,
        create_massive_campionamento_ciacci_con_trocchi_strutt,
        create_massive_campionamento_ciacci_solo_autoctoni_1, get_ciaccio,
    },
};

#[test]
fn calculate_x3_assenza_specie_aliene() {
    let c = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 1.0);
}

#[test]
fn calculate_x3_un_trocchio() {
    let c = create_massive_campionamento_ciacci_2();
    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.75);
}

#[test]
fn calculate_x3_alieni_magg_uguale_autoctoni() {
    let c = create_massive_campionamento_ciacci_con_trocchi_strutt();
    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.0);
}

#[test]
fn calculate_x3_alieni_tipo_1_strutt() {
    let mut c = create_massive_campionamento_ciacci_con_trocchi_strutt();

    let ciaccio = RecordNISECI {
        specie: get_ciaccio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(ciaccio);

    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.0);
}

#[test]
fn calculate_x3_alieni_tipo_2_strutt() {
    let mut c = create_massive_campionamento_ciacci_con_bronzi_strutt();

    let ciaccio = RecordNISECI {
        specie: get_ciaccio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(ciaccio);

    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.25);
}

#[test]
fn calculate_x3_alieni_tipo_3_strutt() {
    let mut c = create_massive_campionamento_ciacci_con_tappi_strutt();

    let ciaccio = RecordNISECI {
        specie: get_ciaccio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(ciaccio);

    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.375);
}

#[test]
fn calculate_x3_alieni_tipo_3_destrutt() {
    let mut c = create_massive_campionamento_ciacci_con_tappi_destrutt();

    let ciaccio = RecordNISECI {
        specie: get_ciaccio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(ciaccio);

    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.875);
}

#[test]
fn calculate_x3_alieni_tipo_3_mediam_strutt() {
    let mut c = create_massive_campionamento_ciacci_con_tappi_mediam_strutt();

    let ciaccio = RecordNISECI {
        specie: get_ciaccio(),
        passaggio_cattura: 2,
        lunghezza: 2,
        peso: 2,
    };
    c.campionamento.push(ciaccio);

    let x3 = calculate_x3(&c);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.625);
}
