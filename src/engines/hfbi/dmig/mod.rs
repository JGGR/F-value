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

use std::collections::HashMap;

use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI, SpecieHFBI};

pub fn calc_dmig(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let bmig = calc_bmig(campione, anagrafica);

    let mut specie_map: HashMap<String, SpecieHFBI> = HashMap::with_capacity(10);
    // trovo il numero di specie trovate
    for cattura in &campione.campionamento {
        match cattura.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi | GruppoEcoHFBI::MigratoriMarini => {
                specie_map.insert(
                    cattura.specie.codice_specie.to_string(),
                    cattura.specie.clone(),
                );
            }
            _ => {}
        }
    }

    let smig = specie_map.len();

    if smig == 0 {
        return 0.0;
    }

    if smig == 1 {
        return 0.01;
    }

    let dmig = (((smig as f32 - 1.0) / bmig.ln()) + 1.0).ln();
    (1000.0 * dmig).round() / 1000.0
}

fn calc_bmig(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut biomig = 0.0;
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi | GruppoEcoHFBI::MigratoriMarini => biomig += specie.peso,
            _ => {}
        }
    }

    let area = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;

    (biomig / area) * 100.0
}

#[cfg(test)]
mod dmig_private_tests {
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

    // Helper to create a species record, specifying code, group, and peso
    fn create_specie_record(
        codice_specie: &'static str,
        gruppo_eco: GruppoEcoHFBI,
        peso: f32,
    ) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Test Specie",
                codice_specie: codice_specie,
                autoctono: true,
                gruppo_eco,
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

    // --- Tests for the private helper function: calc_bmig ---

    #[test]
    fn test_bmig_empty_input() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![],
        };
        // biomig = 0 -> ln(1) = 0
        assert!((calc_bmig(&campione, &anagrafica) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_bmig_zero_area() {
        let anagrafica = create_test_anagrafica(10.0, 0.0); // area = 0
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 100.0)],
        };
        // biomig > 0, area = 0 -> division by zero -> infinity
        assert!(calc_bmig(&campione, &anagrafica).is_infinite());
    }

    #[test]
    fn test_bmig_standard_case() {
        let anagrafica = create_test_anagrafica(20.0, 5.0); // area = 100
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 150.0), // biomig += 150
                create_specie_record("SP2", GruppoEcoHFBI::ResidentiDiEstuario, 100.0), // ignored
                create_specie_record("SP3", GruppoEcoHFBI::MigratoriMarini, 50.0), // biomig += 50
            ],
        };
        // biomig = 150 + 50 = 200
        // expected = ln((200 / 100) * 100 + 1) = ln(201)
        let expected = 200.0_f32;
        let result = calc_bmig(&campione, &anagrafica);
        assert!((result - expected).abs() < EPSILON);
    }

    // --- Tests for the public function: calc_dmig ---

    #[test]
    fn test_dmig_smig_is_zero() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![create_specie_record(
                "SP1",
                GruppoEcoHFBI::ResidentiDiEstuario,
                100.0,
            )],
        };
        // No migratory species, so smig = 0. Should return 0.0
        assert!((calc_dmig(&campione, &anagrafica) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_dmig_smig_is_one() {
        let anagrafica = create_test_anagrafica(100.0, 5.0);
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 100.0),
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 50.0), // Same species
            ],
        };
        // Only one unique migratory species, so smig = 1. Should return 0.01
        assert!((calc_dmig(&campione, &anagrafica) - 0.01).abs() < EPSILON);
    }

    #[test]
    fn test_dmig_bmig_is_infinity() {
        let anagrafica = create_test_anagrafica(10.0, 0.0); // area = 0
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 100.0),
                create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 50.0),
            ],
        };
        // bmig is infinity. Formula is ln(((smig-1)/inf)+1) = ln(1) = 0
        assert!((calc_dmig(&campione, &anagrafica) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_dmig_standard_case_smig_greater_than_one() {
        let anagrafica = create_test_anagrafica(20.0, 5.0); // area = 100
        let campione = CampionamentoHFBI {
            campionamento: vec![
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 150.0),
                create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 250.0),
                create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 50.0), // Duplicate species
                create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 300.0), // Ignored
            ],
        };

        // bmig calculation:
        // biomig = 150 + 250 + 50 = 450
        // bmig = ln((450 / 100) * 100 + 1) = ln(451)
        let bmig = 451.0_f32.ln();

        // smig calculation:
        // Unique species are "SP1" and "SP2", so smig = 2
        let smig = 2.0_f32;

        let expected = (1000.0 * (((smig - 1.0) / bmig) + 1.0).ln()).round() / 1000.0;
        let result = calc_dmig(&campione, &anagrafica);

        assert!((result - expected).abs() < EPSILON);
    }
}
