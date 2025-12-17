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

pub fn calc_bbent(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut biobent = 0.0;
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
                biobent += specie.peso * (specie.specie.gruppo_trofico.microbentivori)
                    + specie.peso * (specie.specie.gruppo_trofico.macrobentivori)
            }
            _ => {}
        }
    }

    let epsilon: f32 = 1e-6;
    if biobent.abs() < epsilon {
        return 0.0;
    }

    let area = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;

    let bbent = ((biobent / area) * 100.0 + 1.0).ln();

    (1000.0 * bbent).round() / 1000.0
}

#[cfg(test)]
mod bbent_private_tests {
    use super::*;
    // Make sure the paths to your domain modules are correct
    use crate::domain::hfbi::{
        AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI, GruppoTrofHFBI, HabitatHFBI, RecordHFBI,
        SpecieHFBI, StagioneHFBI, TipoLagunaCostieraHFBI,
    };
    use crate::domain::location::Location;

    // Epsilon for floating-point comparisons, as requested.
    const EPSILON: f32 = 1e-6;

    // Helper function to create AnagraficaHFBI
    fn create_test_anagrafica(lunghezza: f32, larghezza: f32) -> AnagraficaHFBI {
        AnagraficaHFBI {
            codice_stazione: "TestStazione".to_string(),
            corpo_idrico: "TestCorpoIdrico".to_string(),
            posizione: Location {
                regione: "TestRegione".to_string(),
                provincia: "TestProvincia".to_string(),
            },
            date_string: "01/01/2025".to_string(),
            tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
            stagione: StagioneHFBI::Primavera,
            habitat_vegetato: HabitatHFBI::NonVegetato,
            lunghezza_media_transetto: lunghezza,
            larghezza_media_transetto: larghezza,
        }
    }

    // Helper to create a species record
    fn create_specie_record(
        gruppo_eco: GruppoEcoHFBI,
        microb: f32,
        macrob: f32,
        peso: f32,
    ) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Test Specie",
                codice_specie: "TS",
                autoctono: true,
                gruppo_eco,
                gruppo_trofico: GruppoTrofHFBI {
                    microbentivori: microb,
                    macrobentivori: macrob,
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
    fn test_calc_bbent_empty_campionamento() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        let result = calc_bbent(&campione, &anagrafica);
        let expected = 0.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed empty campionamento test. Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_bbent_with_irrelevant_species() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::OccasionaliMarini,
                0.5,
                0.5,
                100.0,
            )],
        };
        let result = calc_bbent(&campione, &anagrafica);
        let expected = 0.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed irrelevant species test. Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_bbent_with_single_relevant_specie() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::ResidentiDiEstuario,
                0.4,
                0.6,
                200.0,
            )],
        };
        let result = calc_bbent(&campione, &anagrafica);
        let expected = (1000.0 * 41.0_f32.ln()).round() / 1000.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed single relevant specie test. Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_bbent_with_mixed_gruppoeco() {
        let anagrafica = create_test_anagrafica(80.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record(GruppoEcoHFBI::ResidentiDiEstuario, 0.5, 0.5, 150.0),
                create_specie_record(GruppoEcoHFBI::MigratoriMarini, 0.2, 0.3, 100.0),
                create_specie_record(GruppoEcoHFBI::OccasionaliDiAcqueDolci, 1.0, 0.0, 500.0),
                create_specie_record(GruppoEcoHFBI::Diadromi, 0.8, 0.2, 200.0),
            ],
        };
        let result = calc_bbent(&campione, &anagrafica);
        let expected = (1000.0 * 101.0_f32.ln()).round() / 1000.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed mixed species test. Expected: {}, Got: {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_bbent_division_by_zero_area() {
        let anagrafica = create_test_anagrafica(100.0, 0.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::ResidentiDiEstuario,
                1.0,
                0.0,
                100.0,
            )],
        };
        let result = calc_bbent(&campione, &anagrafica);
        assert!(
            result.is_infinite(),
            "Result should be infinity for zero area with positive biobent"
        );
    }

    #[test]
    fn test_calc_bbent_division_by_zero_area_and_zero_biobent() {
        let anagrafica = create_test_anagrafica(0.0, 10.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        let result = calc_bbent(&campione, &anagrafica);
        let expected = 0.0;
        assert!(
            (result - expected).abs() < EPSILON,
            "Failed zero area and zero biobent test. Expected: {}, Got: {}",
            expected,
            result
        );
    }
}
