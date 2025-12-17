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

use crate::csv::parser::parse_date;
use crate::csv::{RecordCsvAnagraficaHFBI, RecordCsvCampionamentoHFBI};
use crate::domain::hfbi::{
    AnagraficaHFBI, HabitatHFBI, RecordHFBI, StagioneHFBI, TipoLagunaCostieraHFBI, RIFERIMENTO_HFBI,
};
use crate::domain::location::Location;
use chrono::format::ParseErrorKind;
use std::fmt;

#[derive(Debug)]
pub enum RecordCsvCampionamentoHFBIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordCsvCampionamentoHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCsvCampionamentoHFBIError::ValoreInvalido { msg } => {
                format!("Errore record campionamento HFBI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

pub fn parse_recordcsv_campionamento_hfbi<T: RecordCsvCampionamentoHFBI>(
    records: Vec<T>,
) -> (Vec<RecordHFBI>, Vec<RecordCsvCampionamentoHFBIError>) {
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        if r.codice_specie().is_empty() {
            let err = RecordCsvCampionamentoHFBIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }
        let codice_specie = r.codice_specie();
        let mut opt_matched_specie = None;
        for s in &RIFERIMENTO_HFBI {
            // FIXME: this is O(n^2).
            if s.codice_specie == codice_specie {
                opt_matched_specie = Some(s);
                break; // TODO: mmmh
            }
        }

        let matched_specie;
        if let Some(specie) = opt_matched_specie {
            matched_specie = specie;
        } else {
            let err = RecordCsvCampionamentoHFBIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: codice_specie non valido (non presente nel riferimento): {}",
                    codice_specie
                ),
            };
            errors.push(err);
            continue;
        }

        //TODO: update this abomination when records change to have an integer directly
        if r.numero_individui() < 1 {
            let err = RecordCsvCampionamentoHFBIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: numero_individui non valido (<1): {}",
                    r.numero_individui()
                ),
            };
            errors.push(err);
            continue;
        }
        let peso = r.peso();

        let hfbi_rec = RecordHFBI {
            specie: matched_specie.clone(),
            numero_individui: r.numero_individui(),
            peso,
        };
        campioni.push(hfbi_rec);
    }
    (campioni, errors)
}

#[derive(Debug)]
pub enum RecordCsvAnagraficaHFBIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordCsvAnagraficaHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCsvAnagraficaHFBIError::ValoreInvalido { msg } => {
                format!("Errore record anagrafica HFBI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

pub fn parse_recordcsv_anagrafica_hfbi<T: RecordCsvAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordCsvAnagraficaHFBIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Troppi record: {}, atteso 1", records.len()),
        };
        errors.push(err);
    }
    if records.is_empty() {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: "Nessun record trovato: atteso 1".to_string(),
        };
        errors.push(err);
        return Err(errors);
    }

    let r = records.first().unwrap();

    if r.codice_stazione().is_empty() {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Codice stazione troppo corto: {}", r.codice_stazione()),
        };
        errors.push(err);
    }

    if r.corpo_idrico().is_empty() {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Corpo idrico troppo corto: {}", r.corpo_idrico()),
        };
        errors.push(err);
    }

    if r.regione().is_empty() {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Regione troppo corta: {}", r.regione()),
        };
        errors.push(err);
    }

    if r.provincia().is_empty() {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Provincia troppo corta: {}", r.provincia()),
        };
        errors.push(err);
    }

    match parse_date(&r.data()) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ParseErrorKind::OutOfRange => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: fuori range".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Impossible => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: valori non possibili".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::NotEnough => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: specifica insufficiente".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Invalid => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: presenza di caratteri non attesi".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooShort => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: terminazione prematura dell'input".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooLong => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: input in eccesso".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::BadFormat => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore nella specifica di formattazione"
                        .to_string(),
                };
                errors.push(err);
            }
            _ => {
                let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore sconosciuto".to_string(),
                };
                errors.push(err);
            }
        },
    }

    if r.lunghezza_stazione() < 0.0 {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!(
                "Lunghezza stazione troppo bassa: {}",
                r.lunghezza_stazione()
            ),
        };
        errors.push(err);
    }

    if r.larghezza_stazione() < 0.0 {
        let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
            msg: format!(
                "Larghezza stazione troppo bassa: {}",
                r.larghezza_stazione()
            ),
        };
        errors.push(err);
    }

    let mut stagione = StagioneHFBI::Primavera;
    match r.stagione() {
        0 => {
            stagione = StagioneHFBI::Primavera;
        }
        1 => {
            stagione = StagioneHFBI::Autunno;
        }
        _ => {
            let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                msg: format!("Stagione HFBI non valido: {}, atteso [0, 1]", r.stagione()),
            };
            errors.push(err);
        }
    }

    let habitat = match r.habitat() {
        0 => HabitatHFBI::Vegetato,
        1 => HabitatHFBI::NonVegetato,
        _ => {
            let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                msg: format!("HabitatHFBI non valido: {}, atteso [0, 1]", r.habitat()),
            };
            errors.push(err);
            HabitatHFBI::Vegetato // To still assign something by default
        }
    };

    let mut tipo_laguna = TipoLagunaCostieraHFBI::MAt1;
    match r.tipo_laguna() {
        1 => {
            tipo_laguna = TipoLagunaCostieraHFBI::MAt1;
        }
        2 => {
            tipo_laguna = TipoLagunaCostieraHFBI::MAt2;
        }
        3 => {
            tipo_laguna = TipoLagunaCostieraHFBI::MAt3;
        }
        _ => {
            let err = RecordCsvAnagraficaHFBIError::ValoreInvalido {
                msg: format!(
                    "TipoLagunaCostieraHFBI non valido: {}, atteso [1, 3]",
                    r.tipo_laguna()
                ),
            };
            errors.push(err);
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let res = AnagraficaHFBI {
        codice_stazione: r.codice_stazione(),
        date_string: r.data(), // Formato gg/mm/aaaa
        corpo_idrico: r.corpo_idrico(),
        posizione: Location {
            regione: r.regione(),
            provincia: r.provincia(),
        },
        lunghezza_media_transetto: r.lunghezza_stazione(),
        larghezza_media_transetto: r.larghezza_stazione(),
        stagione,
        habitat_vegetato: habitat,
        tipo_laguna,
    };
    Ok(res)
}

pub fn check_records_campionamento_hfbi<T: RecordCsvCampionamentoHFBI>(
    records: Vec<T>,
) -> Result<Vec<RecordHFBI>, Vec<RecordCsvCampionamentoHFBIError>> {
    let (records, errors) = parse_recordcsv_campionamento_hfbi(records);

    println!(
        "Campionamento HFBI: Numero record validi: {}",
        records.len()
    );
    println!(
        "Campionamento HFBI: Numero record non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        eprintln!("Errori incontrati durante l'elaborazione dei record per campionamento HFBI: {{");
        //TODO: add process_record_campionamentoNISECI_errors()
        for error in &errors {
            eprintln!("  {}", error);
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record del campionamento HFBI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub fn check_records_anagrafica_hfbi<T: RecordCsvAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordCsvAnagraficaHFBIError>> {
    let res = parse_recordcsv_anagrafica_hfbi(records);

    match res {
        Ok(anagrafica) => {
            println!("Anagrafica HFBI: {}", anagrafica);
            //TODO: handle verbosity
            //println!("Tutti i record dell'anagrafica HFBI sono stati processati con successo!");
            /*
            for record in &records {
                println!("  Record: {{{record}}}");
            }
            */
            Ok(anagrafica)
        }
        Err(errors) => {
            println!(
                "Anagrafica HFBI: Numero record non validi: {}",
                errors.len()
            );
            eprintln!(
                "Errori incontrati durante l'elaborazione dei record per anagrafica HFBI: {{"
            );
            //TODO: add process_record_anagraficaHFBI_errors()
            for error in &errors {
                eprintln!("  {}", error);
            }
            eprintln!("}}");
            Err(errors)
        }
    }
}
