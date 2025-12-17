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

use crate::csv::deser::{
    check_path_is_file_ends_with_csv, deserialize_comma_f32, process_csv_errors, NormalizerReader,
};
use crate::csv::{RecordCsvAnagraficaHFBI, RecordCsvCampionamentoHFBI, TipoRecordCsv};
use std::any::TypeId;
use std::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCsvCampionamentoHFBI {
    pub codice_specie: String,
    pub numero_individui: u32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub peso: f32,
}

impl RecordCsvCampionamentoHFBI for VeryItalianRecordCsvCampionamentoHFBI {
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn numero_individui(&self) -> u32 {
        self.numero_individui
    }
    fn peso(&self) -> f32 {
        self.peso
    }
}

impl fmt::Display for VeryItalianRecordCsvCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvCampionamentoHFBI {
    pub codice_specie: String,
    pub numero_individui: u32,
    pub peso: f32,
}

impl RecordCsvCampionamentoHFBI for PlainRecordCsvCampionamentoHFBI {
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn numero_individui(&self) -> u32 {
        self.numero_individui
    }
    fn peso(&self) -> f32 {
        self.peso
    }
}

impl fmt::Display for PlainRecordCsvCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_campionamento_hfbi<R, T>(
    mut rdr: csv::Reader<R>,
) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvCampionamentoHFBI + 'static,
{
    let mut records = Vec::new();
    let mut errors = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => errors.push(e),
        }
    }

    (records, errors)
}

pub fn check_campionamento_hfbi_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvCampionamentoHFBI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvCampionamentoHFBI>() => b';',
        _ => b',',
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_campionamento_hfbi(rdr);

    println!(
        "Campionamento HFBI: Numero record csv validi: {}",
        records.len()
    );
    println!(
        "Campionamento HFBI: Numero record csv non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        */
        let processed_errors = process_csv_errors(&errors, TipoRecordCsv::CampionamentoHFBI);
        eprintln!("Errori incontrati durante l'elaborazione csv del campionamento HFBI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record csv del campionamento HFBI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub fn check_campionamento_hfbi_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvCampionamentoHFBI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore campionamento HFBI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_campionamento_hfbi_reader(file, has_headers)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCsvAnagraficaHFBI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub lunghezza_stazione: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub larghezza_stazione: f32,
    pub stagione: u32,
    pub habitat: u32,
    pub tipo_laguna: u32,
}

impl RecordCsvAnagraficaHFBI for VeryItalianRecordCsvAnagraficaHFBI {
    fn codice_stazione(&self) -> String {
        self.codice_stazione.clone()
    }
    fn corpo_idrico(&self) -> String {
        self.corpo_idrico.clone()
    }
    fn regione(&self) -> String {
        self.regione.clone()
    }
    fn provincia(&self) -> String {
        self.provincia.clone()
    }
    fn data(&self) -> String {
        self.data.clone()
    }
    fn lunghezza_stazione(&self) -> f32 {
        self.lunghezza_stazione
    }
    fn larghezza_stazione(&self) -> f32 {
        self.larghezza_stazione
    }
    fn stagione(&self) -> u32 {
        self.stagione
    }
    fn habitat(&self) -> u32 {
        self.habitat
    }
    fn tipo_laguna(&self) -> u32 {
        self.tipo_laguna
    }
}

impl fmt::Display for VeryItalianRecordCsvAnagraficaHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaHFBI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], stagione [{}], habitat [{}],\
            tipo_laguna: [{}]}}",
            self.codice_stazione,
            self.corpo_idrico,
            self.regione,
            self.provincia,
            self.data,
            self.lunghezza_stazione,
            self.larghezza_stazione,
            self.stagione,
            self.habitat,
            self.tipo_laguna
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvAnagraficaHFBI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    pub lunghezza_stazione: f32,
    pub larghezza_stazione: f32,
    pub stagione: u32,
    pub habitat: u32,
    pub tipo_laguna: u32,
}

impl RecordCsvAnagraficaHFBI for PlainRecordCsvAnagraficaHFBI {
    fn codice_stazione(&self) -> String {
        self.codice_stazione.clone()
    }
    fn corpo_idrico(&self) -> String {
        self.corpo_idrico.clone()
    }
    fn regione(&self) -> String {
        self.regione.clone()
    }
    fn provincia(&self) -> String {
        self.provincia.clone()
    }
    fn data(&self) -> String {
        self.data.clone()
    }
    fn lunghezza_stazione(&self) -> f32 {
        self.lunghezza_stazione
    }
    fn larghezza_stazione(&self) -> f32 {
        self.larghezza_stazione
    }
    fn stagione(&self) -> u32 {
        self.stagione
    }
    fn habitat(&self) -> u32 {
        self.habitat
    }
    fn tipo_laguna(&self) -> u32 {
        self.tipo_laguna
    }
}

impl fmt::Display for PlainRecordCsvAnagraficaHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaHFBI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], stagione [{}], habitat [{}],\
            tipo_laguna: [{}]}}",
            self.codice_stazione,
            self.corpo_idrico,
            self.regione,
            self.provincia,
            self.data,
            self.lunghezza_stazione,
            self.larghezza_stazione,
            self.stagione,
            self.habitat,
            self.tipo_laguna
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_anagrafica_hfbi<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvAnagraficaHFBI,
{
    let mut records = Vec::new();
    let mut errors = Vec::new();

    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => errors.push(e),
        }
    }

    (records, errors)
}

pub fn check_anagrafica_hfbi_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvAnagraficaHFBI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvAnagraficaHFBI>() => b';',
        _ => b',',
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_anagrafica_hfbi(rdr);

    println!(
        "Anagrafica HFBI: Numero record csv validi: {}",
        records.len()
    );
    println!(
        "Anagrafica HFBI: Numero record csv non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        */
        let processed_errors = process_csv_errors(&errors, TipoRecordCsv::AnagraficaHFBI);
        eprintln!("Errori incontrati durante l'elaborazione csv dell' anagrafica HFBI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record csv dell'anagrafica HFBI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub fn check_anagrafica_hfbi_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvAnagraficaHFBI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore anagrafica HFBI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_anagrafica_hfbi_reader(file, has_headers)
}
