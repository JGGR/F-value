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

pub fn calc_dbent(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut sbent = 0.0;
    let mut densita_biomassa;
    let mut specie_sbent;
    let mut bbent = 0.0;
    let area = anagrafica.larghezza_media_transetto * anagrafica.lunghezza_media_transetto;
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
                specie_sbent = specie.specie.gruppo_trofico.microbentivori;
                specie_sbent += specie.specie.gruppo_trofico.macrobentivori;
                densita_biomassa = (specie.peso / area) * 100.0;
                bbent += densita_biomassa * specie_sbent;
                sbent += specie_sbent;
            }
            _ => {}
        }
    }

    let epsilon: f32 = 1e-6;
    if sbent.abs() < epsilon {
        return 0.0;
    }

    if (sbent - 0.2).abs() < epsilon {
        return 0.01;
    }

    let dbent = (((sbent - 1.0) / bbent.ln()) + 1.0).ln();
    (1000.0 * dbent).round() / 1000.0
}

#[cfg(test)]
mod dbent_private_tests {
    use super::*;
    use crate::domain::hfbi::{
        AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI, GruppoTrofHFBI, HabitatHFBI, RecordHFBI,
        SpecieHFBI, StagioneHFBI, TipoLagunaCostieraHFBI,
    };
    use crate::domain::location::Location;

    const EPSILON: f32 = 1e-6;

    // Helper function to create AnagraficaHFBI
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

    // CORRECTED helper to create a species record
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
                // The "..Default::default()" has been removed and all fields are now set explicitly
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
    fn test_dbent_empty_input() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        let result = calc_dbent(&campione, &anagrafica);
        assert!(
            (result - 0.0).abs() < EPSILON,
            "Expected 0.0 for empty input, got {}",
            result
        );
    }

    #[test]
    fn test_dbent_sbent_is_near_zero() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                GruppoEcoHFBI::ResidentiDiEstuario,
                0.0,
                0.0,
                100.0,
            )],
        };
        let result = calc_dbent(&campione, &anagrafica);
        assert!(
            (result - 0.0).abs() < EPSILON,
            "Expected 0.0 for sbent near zero, got {}",
            result
        );
    }

    #[test]
    fn test_dbent_sbent_is_near_point_two() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record(GruppoEcoHFBI::Diadromi, 0.15, 0.05, 100.0),
                create_specie_record(GruppoEcoHFBI::MigratoriMarini, 0.0, 0.0, 50.0),
            ],
        };
        let result = calc_dbent(&campione, &anagrafica);
        assert!(
            (result - 0.01).abs() < EPSILON,
            "Expected 0.01 for sbent near 0.2, got {}",
            result
        );
    }

    #[test]
    fn test_dbent_standard_calculation() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record(GruppoEcoHFBI::MigratoriMarini, 0.8, 0.7, 200.0),
                create_specie_record(GruppoEcoHFBI::ResidentiDiEstuario, 0.5, 0.0, 150.0),
                create_specie_record(GruppoEcoHFBI::OccasionaliMarini, 1.0, 1.0, 50.0),
            ],
        };

        let sbent = 2.0;
        let bbent = 75.0_f32.ln();
        let expected = (1000.0 * (((sbent - 1.0) / bbent) + 1.0).ln()).round() / 1000.0;
        let result = calc_dbent(&campione, &anagrafica);

        assert!(
            (result - expected).abs() < EPSILON,
            "Standard calculation failed. Expected: {}, Got: {}",
            expected,
            result
        );
    }
}
