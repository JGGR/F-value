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

use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI};

pub fn calc_dhzp(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut shzp = 0.0;
    let bhzp = calc_bhzp(campione, anagrafica);
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
                shzp += specie.specie.gruppo_trofico.iperbentivori;
            }
            _ => {}
        }
    }

    let epsilon: f32 = 1e-6;
    if shzp.abs() < epsilon {
        return 0.0;
    }

    if (shzp - 0.2).abs() < epsilon {
        return 0.01;
    }

    let dhzp = (((shzp - 0.2) / bhzp.ln()) + 1.0).ln();
    (1000.0 * dhzp).round() / 1000.0
}

fn calc_bhzp(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut biohzp = 0.0;
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
                biohzp += specie.peso * (specie.specie.gruppo_trofico.iperbentivori)
            }
            _ => {}
        }
    }

    let area = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;

    (biohzp / area) * 100.0
}

#[cfg(test)]
mod dhzp_private_tests {
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

    // Helper to create a species record, specifying the hyperbentivore value
    fn create_specie_record(
        gruppo_eco: GruppoEcoHFBI,
        peso: f32,
        iperbentivori: f32,
    ) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Test Specie",
                codice_specie: "TS",
                autoctono: true,
                gruppo_eco,
                gruppo_trofico: GruppoTrofHFBI {
                    iperbentivori,
                    microbentivori: 0.0,
                    macrobentivori: 0.0,
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

    // --- Tests for the private helper function: calc_bhzp ---

    #[test]
    fn test_bhzp_empty_input() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        // biohzp = 0 -> ln(1) = 0
        assert!((calc_bhzp(&campione, &anagrafica) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_bhzp_zero_area() {
        let anagrafica = create_test_anagrafica(10.0, 0.0); // area = 0
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::ResidentiDiEstuario,
                100.0,
                1.0,
            )],
        };
        // biohzp > 0, area = 0 -> division by zero -> infinity
        assert!(calc_bhzp(&campione, &anagrafica).is_infinite());
    }

    #[test]
    fn test_bhzp_standard_case() {
        let anagrafica = create_test_anagrafica(20.0, 5.0); // area = 100
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record(GruppoEcoHFBI::ResidentiDiEstuario, 50.0, 0.5), // biohzp += 25
                create_specie_record(GruppoEcoHFBI::OccasionaliMarini, 100.0, 1.0),  // ignored
                create_specie_record(GruppoEcoHFBI::MigratoriMarini, 100.0, 0.75),   // biohzp += 75
            ],
        };
        // biohzp = 25 + 75 = 100
        // expected = ln((100 / 100) * 100 + 1) = ln(101)
        let expected = 100.0_f32;
        let result = calc_bhzp(&campione, &anagrafica);
        assert!((result - expected).abs() < EPSILON);
    }

    // --- Tests for the public function: calc_dhzp ---

    #[test]
    fn test_dhzp_shzp_is_zero() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            // Relevant species, but iperbentivori is 0, so shzp is 0
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::ResidentiDiEstuario,
                100.0,
                0.0,
            )],
        };
        // The special case for shzp near zero should trigger
        assert!((calc_dhzp(&campione, &anagrafica) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_dhzp_shzp_is_point_two() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record(GruppoEcoHFBI::Diadromi, 100.0, 0.1),
                create_specie_record(GruppoEcoHFBI::MigratoriMarini, 50.0, 0.1),
            ],
        };
        // shzp = 0.1 + 0.1 = 0.2. The special case should trigger.
        assert!((calc_dhzp(&campione, &anagrafica) - 0.01).abs() < EPSILON);
    }

    #[test]
    fn test_dhzp_bhzp_is_infinity() {
        let anagrafica = create_test_anagrafica(10.0, 0.0); // area = 0
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::ResidentiDiEstuario,
                100.0,
                0.5,
            )],
        };
        // bhzp is infinity. Formula is ln(((shzp-0.2)/inf)+1) = ln(1) = 0
        assert!((calc_dhzp(&campione, &anagrafica) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_dhzp_standard_calculation() {
        let anagrafica = create_test_anagrafica(10.0, 5.0); // area = 50
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record(GruppoEcoHFBI::ResidentiDiEstuario, 100.0, 0.5),
                create_specie_record(GruppoEcoHFBI::MigratoriMarini, 200.0, 1.0),
                create_specie_record(GruppoEcoHFBI::OccasionaliMarini, 50.0, 0.8), // ignored
            ],
        };
        // From calc_bhzp:
        // biohzp = (100 * 0.5) + (200 * 1.0) = 50 + 200 = 250
        // bhzp = ln((250 / 50) * 100 + 1) = ln(501)
        let bhzp = 501.0_f32.ln();

        // From calc_dhzp:
        // shzp = 0.5 + 1.0 = 1.5
        let shzp = 1.5_f32;

        let expected = (1000.0 * (((shzp - 0.2) / bhzp) + 1.0).ln()).round() / 1000.0;
        let result = calc_dhzp(&campione, &anagrafica);

        assert!((result - expected).abs() < EPSILON);
    }
}
