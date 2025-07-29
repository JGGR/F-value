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
    domain::niseci::{CampionamentoNISECI, RecordNISECI, SpecieNISECI},
    engines::niseci::linear_regression::Point,
};

#[test]
fn test_calcolo_pesci_per_passaggio() {
    let specie_1 = SpecieNISECI {
        id: "1234".to_string(),
        nome: "Cervus elaphus".to_string(),
        tipo_autoctono: 1,
        tipo_alloctono: 0,
        specie_attesa: true,
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
        specie: specie_1.clone(),
        passaggio_cattura: 1,
        lunghezza: 100, // in millimetri
        peso: 100,      // in grammi
    };

    let record_2 = RecordNISECI {
        specie: specie_1.clone(),
        passaggio_cattura: 2,
        lunghezza: 100, // in millimetri
        peso: 100,      // in grammi
    };

    let record_3 = RecordNISECI {
        specie: specie_1.clone(),
        passaggio_cattura: 3,
        lunghezza: 100, // in millimetri
        peso: 100,      // in grammi
    };

    let mut c1 = vec![record_1; 20];
    let mut c2 = vec![record_2; 15];
    let mut c3 = vec![record_3; 10];

    c1.append(&mut c2);
    c1.append(&mut c3);
    let campionamento = CampionamentoNISECI { campionamento: c1 };

    let pesci_per_passaggio = campionamento.fishes_for_every_passage();

    assert_eq!(pesci_per_passaggio[0], Point::new(20, 20));
    assert_eq!(pesci_per_passaggio[1], Point::new(35, 15));
    assert_eq!(pesci_per_passaggio[2], Point::new(45, 10));
}
