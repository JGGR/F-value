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

use super::{
    RecordCsvAnagraficaHFBI, RecordCsvAnagraficaNISECI, RecordCsvCampionamentoHFBI,
    RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI,
};
use crate::domain::hfbi::{
    AnagraficaHFBI, HabitatHFBI, RecordHFBI, StagioneHFBI, TipoLagunaCostieraHFBI, RIFERIMENTO_HFBI,
};
use crate::domain::location::Location;
use crate::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, ComunitaNISECI, IdroEcoRegioneNISECI, RecordNISECI, SpecieNISECI,
    TipoComunitaNISECI,
};
use chrono::format::ParseErrorKind;
use chrono::NaiveDate;
use std::fmt;

#[derive(Debug)]
pub(crate) enum RecordCsvRiferimentoNISECIError {
    ValoreInvalido { msg: String }, //TODO: add position?
    SoglieCLNonCrescenti { msg: String },
    SoglieADJUVNonCrescenti { msg: String },
}

impl fmt::Display for RecordCsvRiferimentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCsvRiferimentoNISECIError::ValoreInvalido { msg } => {
                format!("Errore record riferimento NISECI: {}", msg)
            }
            RecordCsvRiferimentoNISECIError::SoglieCLNonCrescenti { msg } => {
                format!("Errore record riferimento NISECI: {}", msg)
            }
            RecordCsvRiferimentoNISECIError::SoglieADJUVNonCrescenti { msg } => {
                format!("Errore record riferimento NISECI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

pub(crate) fn parse_recordcsv_riferimento_niseci<T: RecordCsvRiferimentoNISECI>(
    records: Vec<T>,
) -> (Vec<SpecieNISECI>, Vec<RecordCsvRiferimentoNISECIError>) {
    let mut specie = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    let mut used_id_specie = Vec::new(); // Stores already-parsed ids to detect doubles
    for r in records {
        idx += 1;
        let mut origine_autoctono = true;
        match r.origine().as_str() {
            "ALL" => {
                origine_autoctono = false;
            }
            "AUT" => {}
            _ => {
                let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                    msg: format!(
                        "Record {idx}: origine invalida (non \"AUT\" o \"ALL\"): {}",
                        r.origine()
                    ),
                };
                errors.push(err);
                continue;
            }
        }
        let specie_attesa = r.specie_attesa() > 0; // TODO: possiamo prendere qualsiasi non-zero come
                                                   // "atteso"?
        let tipo_autoctono: u8;
        let tipo_alloctono: u8;
        if origine_autoctono {
            match r.tipo_autoctono() {
                1 | 2 => {
                    tipo_autoctono = r.tipo_autoctono() as u8;
                }
                _ => {
                    let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                        msg: format!(
                            "Record {idx}: tipo_autoctono non valido (non 1 o 2): {}",
                            r.tipo_autoctono()
                        ),
                    };
                    errors.push(err);
                    continue;
                }
            }
            tipo_alloctono = 0;
        } else {
            tipo_autoctono = 0;
            match r.allo_nocivita() {
                0..=3 => {
                    tipo_alloctono = r.allo_nocivita() as u8;
                }
                _ => {
                    let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                        msg: format!(
                            "Record {idx}: allo_nocivita non valido (non [0..3]): {}",
                            r.allo_nocivita()
                        ),
                    };
                    errors.push(err);
                    continue;
                }
            }
        }

        if r.codice_specie().is_empty() {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }

        let id = r.codice_specie();

        if used_id_specie.contains(&id) {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (ridefinizione)"),
            };
            errors.push(err);
            continue;
        }

        let nome = r.nome_latino(); //TODO: controllare se dovrebbe essere nome_comune

        //TODO: update when SpecieNISECI has the missing fields

        let epsilon: f32 = 1e-6;

        // Check dens_soglia
        if r.dens_soglia1() < 0.0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia1 non valido (< 0)"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1().abs() < epsilon && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia1 non valido (== 0) per una specie attesa"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2() < 0.0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia2 non valido (< 0)"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2().abs() < epsilon && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia2 non valido (== 0) per una specie attesa"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1() >= r.dens_soglia2() && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: dens_soglia1 maggiore di dens_soglia2 per una specie attesa"
                ),
            };
            errors.push(err);
            continue;
        }

        if !check_soglie_cl(&r) {
            let err = RecordCsvRiferimentoNISECIError::SoglieCLNonCrescenti {
                msg: format!("Record {idx}: soglie CL non crescenti"),
            };
            errors.push(err);
            continue;
        }
        if !check_soglie_ad_juv(&r) {
            let err = RecordCsvRiferimentoNISECIError::SoglieADJUVNonCrescenti {
                msg: format!("Record {idx}: soglie AD/JUV non crescenti"),
            };
            errors.push(err);
            continue;
        }

        let specie_rec = SpecieNISECI {
            id: id.clone(),
            nome,
            tipo_autoctono,
            tipo_alloctono,
            specie_attesa,
            cl_soglia1: r.cl_soglia1(), // in cm
            cl_soglia2: r.cl_soglia2(), // in cm
            cl_soglia3: r.cl_soglia3(), // in cm
            cl_soglia4: r.cl_soglia4(), // in cm
            ad_juv_soglia1: r.ad_juv_soglia1(),
            ad_juv_soglia2: r.ad_juv_soglia2(),
            ad_juv_soglia3: r.ad_juv_soglia3(),
            ad_juv_soglia4: r.ad_juv_soglia4(),
            dens_soglia1: r.dens_soglia1(),
            dens_soglia2: r.dens_soglia2(),
        };
        specie.push(specie_rec);
        used_id_specie.push(id);
    }

    (specie, errors)
}

#[derive(Debug)]
pub(crate) enum RecordCsvCampionamentoNISECIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordCsvCampionamentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCsvCampionamentoNISECIError::ValoreInvalido { msg } => {
                format!("Errore record campionamento NISECI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

pub(crate) fn parse_recordcsv_campionamento_niseci<T: RecordCsvCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> (Vec<RecordNISECI>, Vec<RecordCsvCampionamentoNISECIError>) {
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        if r.codice_specie().is_empty() {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }
        let codice_specie = r.codice_specie();
        let mut opt_matched_specie = None;
        for s in &riferimento_specie {
            // FIXME: this is O(n^2).
            if s.id == codice_specie {
                opt_matched_specie = Some(s);
                break; // TODO: mmmh
            }
        }

        let matched_specie;
        if let Some(specie) = opt_matched_specie {
            matched_specie = specie;
        } else {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: codice_specie non valido (non presente nel riferimento): {}",
                    codice_specie
                ),
            };
            errors.push(err);
            continue;
        }

        //TODO: update this abomination when records change to have an integer directly
        if r.num_passaggio() < 1 {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: num_passaggio non valido (<1): {}",
                    r.num_passaggio()
                ),
            };
            errors.push(err);
            continue;
        }
        let passaggio_cattura = r.num_passaggio();

        let niseci_rec = RecordNISECI {
            specie: matched_specie.clone(),
            passaggio_cattura: passaggio_cattura as u8,
            lunghezza: r.lunghezza(),
            peso: r.peso(),
        };
        campioni.push(niseci_rec);
    }
    (campioni, errors)
}

#[derive(Debug)]
pub(crate) enum RecordCsvAnagraficaNISECIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordCsvAnagraficaNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCsvAnagraficaNISECIError::ValoreInvalido { msg } => {
                format!("Errore record anagrafica NISECI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

pub(crate) fn parse_recordcsv_anagrafica_niseci<T: RecordCsvAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordCsvAnagraficaNISECIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Troppi record: {}, atteso 1", records.len()),
        };
        errors.push(err);
    }
    if records.is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: "Nessun record trovato: atteso 1".to_string(),
        };
        errors.push(err);
        return Err(errors);
    }

    let r = records.first().unwrap();

    if r.codice_stazione().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Codice stazione troppo corto: {}", r.codice_stazione()),
        };
        errors.push(err);
    }

    if r.corpo_idrico().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Corpo idrico troppo corto: {}", r.corpo_idrico()),
        };
        errors.push(err);
    }

    if r.regione().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Regione troppo corta: {}", r.regione()),
        };
        errors.push(err);
    }

    if r.provincia().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Provincia troppo corta: {}", r.provincia()),
        };
        errors.push(err);
    }

    match parse_date(&r.data()) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ParseErrorKind::OutOfRange => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: fuori range".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Impossible => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: valori non possibili".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::NotEnough => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: specifica insufficiente".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Invalid => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: presenza di caratteri non attesi".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooShort => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: terminazione prematura dell'input".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooLong => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: input in eccesso".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::BadFormat => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore nella specifica di formattazione"
                        .to_string(),
                };
                errors.push(err);
            }
            _ => {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore sconosciuto".to_string(),
                };
                errors.push(err);
            }
        },
    }

    if r.lunghezza_stazione() < 0.0 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!(
                "Lunghezza stazione troppo bassa: {}",
                r.lunghezza_stazione()
            ),
        };
        errors.push(err);
    }

    if r.larghezza_stazione() < 0.0 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!(
                "Larghezza stazione troppo bassa: {}",
                r.larghezza_stazione()
            ),
        };
        errors.push(err);
    }

    let mut tipo_comunita = TipoComunitaNISECI::Redatta;
    match r.tipo_comunita() {
        0 => { /* Redatta */ }
        1 => {
            tipo_comunita = TipoComunitaNISECI::Recuperata;
        }
        2 => {
            tipo_comunita = TipoComunitaNISECI::Dm260_2010;
        }
        3 => {
            tipo_comunita = TipoComunitaNISECI::AffinataDalMase;
        }
        _ => {
            let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                msg: format!(
                    "Tipo comunita NISECI non valido: {}, atteso [0, 3]",
                    r.tipo_comunita()
                ),
            };
            errors.push(err);
        }
    }

    match tipo_comunita {
        TipoComunitaNISECI::Recuperata => {
            if r.fonte().is_empty() {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: format!("Fonte troppo corta: {}", r.fonte()),
                };
                errors.push(err);
            }
        }
        TipoComunitaNISECI::AffinataDalMase => {
            if r.numero_protocollo().is_empty() {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                    msg: format!("Numero protocollo troppo corto: {}", r.numero_protocollo()),
                };
                errors.push(err);
            }
        }
        _ => {}
    }

    let idro_eco_regione = match r.idro_eco_regione() {
        0 => IdroEcoRegioneNISECI::AlpiCentroOrientali,
        1 => IdroEcoRegioneNISECI::AlpiMediterranee,
        2 => IdroEcoRegioneNISECI::AlpiMeridionali,
        3 => IdroEcoRegioneNISECI::AlpiOccidentali,
        4 => IdroEcoRegioneNISECI::AppenninoCentrale,
        5 => IdroEcoRegioneNISECI::AppenninoMeridionale,
        6 => IdroEcoRegioneNISECI::AppenninoPiemontese,
        7 => IdroEcoRegioneNISECI::AppenninoSettentrionale,
        8 => IdroEcoRegioneNISECI::BasilicataTavoliere,
        9 => IdroEcoRegioneNISECI::BassoLazio,
        10 => IdroEcoRegioneNISECI::CalabriaNebrodi,
        11 => IdroEcoRegioneNISECI::Carso,
        12 => IdroEcoRegioneNISECI::CostaAdriatica,
        13 => IdroEcoRegioneNISECI::Monferrato,
        14 => IdroEcoRegioneNISECI::PianuraPadana,
        15 => IdroEcoRegioneNISECI::PrealpiDolomiti,
        16 => IdroEcoRegioneNISECI::PugliaGargano,
        17 => IdroEcoRegioneNISECI::RomaViterbeseVesuvio,
        18 => IdroEcoRegioneNISECI::Sardegna,
        19 => IdroEcoRegioneNISECI::Sicilia,
        20 => IdroEcoRegioneNISECI::Toscana,
        _ => {
            let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
                msg: format!(
                    "IdroEcoRegioneNISECI non valido: {}, atteso [0, 20]",
                    r.idro_eco_regione()
                ),
            };
            errors.push(err);
            IdroEcoRegioneNISECI::Toscana // To still assign something by default
        }
    };

    let mut area = AreaNISECI::Mediterranea;
    if r.area_alpina() > 0 {
        area = AreaNISECI::Alpina;
    }

    if r.nome_bacino().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Nome bacino troppo corto: {}", r.nome_bacino()),
        };
        errors.push(err);
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let res = AnagraficaNISECI {
        comunita: ComunitaNISECI {
            tipo: tipo_comunita,
            fonte: Some(r.fonte()),
            numero_protocollo: Some(r.numero_protocollo()),
        },
        codice_stazione: r.codice_stazione(),
        date_string: r.data(), // Formato gg/mm/aaaa
        area,
        corpo_idrico: r.corpo_idrico(),
        bacino_appartenenza: r.nome_bacino(),
        idro_eco_regione,
        posizione: Location {
            regione: r.regione(),
            provincia: r.provincia(),
        },
        lunghezza_media_stazione: r.lunghezza_stazione(),
        larghezza_media_stazione: r.larghezza_stazione(),
    };
    Ok(res)
}

pub(crate) fn check_records_riferimento_niseci<T: RecordCsvRiferimentoNISECI>(
    records: Vec<T>,
) -> Result<Vec<SpecieNISECI>, Vec<RecordCsvRiferimentoNISECIError>> {
    let (records, errors) = parse_recordcsv_riferimento_niseci(records);

    println!(
        "Riferimento NISECI: Numero record validi: {}",
        records.len()
    );
    println!(
        "Riferimento NISECI: Numero record non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        eprintln!("Errori incontrati durante l'elaborazione dei record per riferimento NISECI: {{");
        //TODO: add process_record_riferimentoNISECI_errors()
        for error in &errors {
            eprintln!("  {}", error);
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record del riferimento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub(crate) fn check_records_campionamento_niseci<T: RecordCsvCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> Result<Vec<RecordNISECI>, Vec<RecordCsvCampionamentoNISECIError>> {
    let (records, errors) = parse_recordcsv_campionamento_niseci(records, riferimento_specie);

    println!(
        "Campionamento NISECI: Numero record validi: {}",
        records.len()
    );
    println!(
        "Campionamento NISECI: Numero record non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        eprintln!(
            "Errori incontrati durante l'elaborazione dei record per campionamento NISECI: {{"
        );
        //TODO: add process_record_campionamentoNISECI_errors()
        for error in &errors {
            eprintln!("  {}", error);
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record del campionamento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub(crate) fn check_records_anagrafica_niseci<T: RecordCsvAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordCsvAnagraficaNISECIError>> {
    let res = parse_recordcsv_anagrafica_niseci(records);

    match res {
        Ok(anagrafica) => {
            println!("Anagrafica NISECI: {}", anagrafica);
            //TODO: handle verbosity
            //println!("Tutti i record dell'anagrafica NISECI sono stati processati con successo!");
            /*
            for record in &records {
                println!("  Record: {{{record}}}");
            }
            */
            Ok(anagrafica)
        }
        Err(errors) => {
            println!(
                "Anagrafica NISECI: Numero record non validi: {}",
                errors.len()
            );
            eprintln!(
                "Errori incontrati durante l'elaborazione dei record per anagrafica NISECI: {{"
            );
            //TODO: add process_record_anagraficaNISECI_errors()
            for error in &errors {
                eprintln!("  {}", error);
            }
            eprintln!("}}");
            Err(errors)
        }
    }
}

#[derive(Debug)]
pub(crate) enum RecordCsvCampionamentoHFBIError {
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

pub(crate) fn parse_recordcsv_campionamento_hfbi<T: RecordCsvCampionamentoHFBI>(
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
            peso: peso,
        };
        campioni.push(hfbi_rec);
    }
    (campioni, errors)
}

#[derive(Debug)]
pub(crate) enum RecordCsvAnagraficaHFBIError {
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

pub(crate) fn parse_recordcsv_anagrafica_hfbi<T: RecordCsvAnagraficaHFBI>(
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
        stagione: stagione,
        habitat_vegetato: habitat,
        tipo_laguna: tipo_laguna,
    };
    Ok(res)
}

pub(crate) fn check_records_campionamento_hfbi<T: RecordCsvCampionamentoHFBI>(
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

pub(crate) fn check_records_anagrafica_hfbi<T: RecordCsvAnagraficaHFBI>(
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

pub(crate) fn parse_date(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    let normalized = date_str.replace("/", "-"); // Replace all / with -
    NaiveDate::parse_from_str(&normalized, "%d-%m-%Y")
}

fn check_soglie_cl<T: RecordCsvRiferimentoNISECI>(r: &T) -> bool {
    if r.cl_soglia1() < r.cl_soglia2()
        && r.cl_soglia2() < r.cl_soglia3()
        && r.cl_soglia3() < r.cl_soglia4()
    {
        return true;
    }
    false
}

fn check_soglie_ad_juv<T: RecordCsvRiferimentoNISECI>(r: &T) -> bool {
    if r.ad_juv_soglia1() < r.ad_juv_soglia2()
        && r.ad_juv_soglia2() < r.ad_juv_soglia3()
        && r.ad_juv_soglia3() < r.ad_juv_soglia4()
    {
        return true;
    }
    false
}
