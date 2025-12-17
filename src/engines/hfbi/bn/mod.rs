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

use crate::domain::hfbi::CampionamentoHFBI;

pub fn calc_bn(campione: &CampionamentoHFBI) -> f32 {
    let mut b = 0.0;
    let mut n = 0.0;
    for specie in &campione.campionamento {
        b += specie.peso;
        n += specie.numero_individui as f32;
    }
    let bn = ((b / n) + 1.0).ln();
    (1000.0 * bn).round() / 1000.0
}

#[cfg(test)]
mod bn_private_tests {
    use super::*;
    use crate::domain::hfbi::{
        CampionamentoHFBI, GruppoEcoHFBI, GruppoTrofHFBI, RecordHFBI, SpecieHFBI,
    };

    // Epsilon for floating-point comparisons.
    const EPSILON: f32 = 1e-6;

    // Helper to create a species record. We only need the `peso` for these tests.
    fn create_dummy_record(peso: f32) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Dummy",
                codice_specie: "DM",
                autoctono: true,
                gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario, // This field is not used by calc_bn
                gruppo_trofico: GruppoTrofHFBI {
                    // This field is not used by calc_bn
                    microbentivori: 0.0,
                    macrobentivori: 0.0,
                    iperbentivori: 0.0,
                    erbivori: 0.0,
                    detritivori: 0.0,
                    planctivori: 0.0,
                    onnivori: 0.0,
                },
            },
            numero_individui: 1,
            peso,
        }
    }

    #[test]
    fn test_calc_bn_empty_campionamento() {
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        let result = calc_bn(&campione);
        // For an empty input, b=0 and n=0. The division 0.0 / 0.0 results in NaN (Not a Number).
        assert!(
            result.is_nan(),
            "Expected NaN for empty input, but got {}",
            result
        );
    }

    #[test]
    fn test_calc_bn_single_specie() {
        let campione = CampionamentoHFBI {
            campionamento: vec![create_dummy_record(150.0)],
        };
        let result = calc_bn(&campione);
        // b = 150.0, n = 1.0
        // expected = ((150.0 / 1.0) + 1.0).ln() = 151.0.ln()
        let expected = (1000.0 * 151.0_f32.ln()).round() / 1000.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed single specie test. Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_bn_multiple_species() {
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_dummy_record(100.0),
                create_dummy_record(200.0),
                create_dummy_record(50.0),
            ],
        };
        let result = calc_bn(&campione);
        // b = 100 + 200 + 50 = 350.0
        // n = 3.0
        // expected = ((350.0 / 3.0) + 1.0).ln()
        let expected = (1000.0 * (350.0_f32 / 3.0 + 1.0).ln()).round() / 1000.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed multiple species test. Expected: {}, Got: {}",
            expected,
            result
        );
    }
}
