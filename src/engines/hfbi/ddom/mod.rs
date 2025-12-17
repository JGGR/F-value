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

use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI};

pub fn calc_ddom(campionamento: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let (s90, b90): (u32, f32) = calc_s90_b90(campionamento, anagrafica);

    let ddom = (((s90 as f32 - 1.0) / b90) + 1.0).ln();
    (1000.0 * ddom).round() / 1000.0
}

fn calc_s90_b90(campionamento: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> (u32, f32) {
    let mut biomassa_tot = 0.0;
    for cattura in &campionamento.campionamento {
        biomassa_tot += cattura.peso;
    }

    let biomassa_90 = biomassa_tot * 0.9;

    let mut n_specie_90: u32 = 0;
    let mut biomassa_tmp: f32 = 0.0;
    for cattura in &campionamento.campionamento {
        biomassa_tmp += cattura.peso;
        n_specie_90 += 1;
        if biomassa_tmp > biomassa_90 {
            break;
        }
    }

    let area: f32 = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;
    let b90: f32 = ((biomassa_90 / area) * 100.0 + 1.0).ln();

    (n_specie_90, b90)
}

#[cfg(test)]
mod ddom_private_tests {
    use super::*;
    use crate::domain::hfbi::{
        AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI, GruppoTrofHFBI, HabitatHFBI, RecordHFBI,
        SpecieHFBI, StagioneHFBI, TipoLagunaCostieraHFBI,
    };
    use crate::domain::location::Location;

    const EPSILON: f32 = 1e-6;

    // Helper to create AnagraficaHFBI for tests
    fn create_test_anagrafica(lunghezza: f32, larghezza: f32) -> AnagraficaHFBI {
        AnagraficaHFBI {
            codice_stazione: "TestStazione".to_string(),
            corpo_idrico: "TestCorpoIdrico".to_string(),
            posizione: Location {
                regione: "Test".to_string(),
                provincia: "Test".to_string(),
            },
            date_string: "01/01/2025".to_string(),
            tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
            stagione: StagioneHFBI::Primavera,
            habitat_vegetato: HabitatHFBI::NonVegetato,
            lunghezza_media_transetto: lunghezza,
            larghezza_media_transetto: larghezza,
        }
    }

    // Helper to create a dummy RecordHFBI, as only the peso is relevant here
    fn create_dummy_record(peso: f32) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Dummy",
                codice_specie: "DM",
                autoctono: true,
                gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
                gruppo_trofico: GruppoTrofHFBI {
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

    // --- Tests for the private helper function: calc_s90_b90 ---

    #[test]
    fn test_s90_b90_empty_input() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        let (s90, b90) = calc_s90_b90(&campione, &anagrafica);

        assert_eq!(s90, 0);
        // b90 = ln((0 / 500) * 100 + 1) = ln(1) = 0
        assert!((b90 - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_s90_b90_single_species() {
        let anagrafica = create_test_anagrafica(10.0, 10.0); // area = 100
        let campione = CampionamentoHFBI {
            campionamento: vec![create_dummy_record(200.0)],
        };
        let (s90, b90) = calc_s90_b90(&campione, &anagrafica);

        // n_specie_90 is 1 because the loop runs once and breaks.
        assert_eq!(s90, 1);
        // biomassa_90 = 200 * 0.9 = 180
        // b90 = ln((180 / 100) * 100 + 1) = ln(181)
        let expected_b90 = 181.0_f32.ln();
        assert!((b90 - expected_b90).abs() < EPSILON);
    }

    #[test]
    fn test_s90_b90_zero_area() {
        let anagrafica = create_test_anagrafica(10.0, 0.0); // area = 0
        let campione = CampionamentoHFBI {
            campionamento: vec![create_dummy_record(100.0)],
        };
        let (s90, b90) = calc_s90_b90(&campione, &anagrafica);

        assert_eq!(s90, 1);
        // Division by zero area results in infinity
        assert!(b90.is_infinite());
    }

    // --- Tests for the public function: calc_ddom ---

    #[test]
    fn test_ddom_empty_input() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        let result = calc_ddom(&campione, &anagrafica);
        // s90=0, b90=0. Formula is ln(((0-1)/0)+1) = ln(-inf) = NaN
        assert!(result.is_nan());
    }

    #[test]
    fn test_ddom_single_species() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![create_dummy_record(100.0)],
        };
        let result = calc_ddom(&campione, &anagrafica);
        // s90=1. Formula is ln(((1-1)/b90)+1) = ln(1) = 0
        assert!((result - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_ddom_zero_area() {
        let anagrafica = create_test_anagrafica(10.0, 0.0); // area = 0
        let campione = CampionamentoHFBI {
            campionamento: vec![create_dummy_record(100.0), create_dummy_record(50.0)],
        };
        let result = calc_ddom(&campione, &anagrafica);
        // b90 is infinity. Formula is ln(((s90-1)/inf)+1) = ln(0+1) = ln(1) = 0
        assert!((result - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_ddom_standard_case() {
        let anagrafica = create_test_anagrafica(10.0, 10.0); // area = 100
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_dummy_record(100.0),
                create_dummy_record(50.0),
                create_dummy_record(30.0),
                create_dummy_record(20.0), // 90% threshold (180) is crossed here
            ],
        };
        // From calc_s90_b90:
        // biomassa_tot = 200, biomassa_90 = 180.
        // Loop adds weights: 100, 150, 180, 200. It breaks after the 4th species.
        // s90 = 4
        // b90 = ln((180 / 100) * 100 + 1) = ln(181)
        let s90 = 4.0_f32;
        let b90 = 181.0_f32.ln();

        let expected_result = (1000.0 * (((s90 - 1.0) / b90) + 1.0).ln()).round() / 1000.0;
        let actual_result = calc_ddom(&campione, &anagrafica);

        assert!((actual_result - expected_result).abs() < EPSILON);
    }
}
