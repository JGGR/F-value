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
use crate::csv::{
    RecordCsvAnagraficaNISECI, RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI,
    TipoRecordCsv,
};
use std::any::TypeId;
use std::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCsvRiferimentoNISECI {
    pub nome_comune: String,
    pub nome_latino: String,
    pub codice_specie: String,
    pub origine: String,
    pub tipo_autoctono: u32,
    pub allo_nocivita: u32,
    pub specie_attesa: u32,
    pub cl_soglia1: u32, // in mm
    pub cl_soglia2: u32, // in mm
    pub cl_soglia3: u32, // in mm
    pub cl_soglia4: u32, // in mm
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia1: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia2: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia3: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia4: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub dens_soglia1: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub dens_soglia2: f32,
}

impl RecordCsvRiferimentoNISECI for VeryItalianRecordCsvRiferimentoNISECI {
    fn nome_comune(&self) -> String {
        self.nome_comune.clone()
    }
    fn nome_latino(&self) -> String {
        self.nome_latino.clone()
    }
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn origine(&self) -> String {
        self.origine.clone()
    }
    fn tipo_autoctono(&self) -> u32 {
        self.tipo_autoctono
    }
    fn allo_nocivita(&self) -> u32 {
        self.allo_nocivita
    }
    fn specie_attesa(&self) -> u32 {
        self.specie_attesa
    }
    fn cl_soglia1(&self) -> u32 {
        self.cl_soglia1
    }
    fn cl_soglia2(&self) -> u32 {
        self.cl_soglia2
    }
    fn cl_soglia3(&self) -> u32 {
        self.cl_soglia3
    }
    fn cl_soglia4(&self) -> u32 {
        self.cl_soglia4
    }
    fn ad_juv_soglia1(&self) -> f32 {
        self.ad_juv_soglia1
    }
    fn ad_juv_soglia2(&self) -> f32 {
        self.ad_juv_soglia2
    }
    fn ad_juv_soglia3(&self) -> f32 {
        self.ad_juv_soglia3
    }
    fn ad_juv_soglia4(&self) -> f32 {
        self.ad_juv_soglia4
    }
    fn dens_soglia1(&self) -> f32 {
        self.dens_soglia1
    }
    fn dens_soglia2(&self) -> f32 {
        self.dens_soglia2
    }
}

impl fmt::Display for VeryItalianRecordCsvRiferimentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvRiferimentoNISECI: {{ nome_comune: [{}], nome_latino: [{}], codice_specie: [{}], origine: [{}], tipo_autoctono: [{}], allo_nocivita: [{}], specie_attesa: [{}], cl_soglia1: [{}], cl_soglia2: [{}], cl_soglia3: [{}], cl_soglia4: [{}], ad_juv_soglia1: [{}], ad_juv_soglia2: [{}], ad_juv_soglia3: [{}], ad_juv_soglia4: [{}], dens_soglia1: [{}], dens_soglia2: [{}] }}",
              self.nome_comune, self.nome_latino, self.codice_specie, self.origine,
              self.tipo_autoctono, self.allo_nocivita, self.specie_attesa,
              self.cl_soglia1, self.cl_soglia2, self.cl_soglia3, self.cl_soglia4,
              self.ad_juv_soglia1, self.ad_juv_soglia2, self.ad_juv_soglia3, self.ad_juv_soglia4,
              self.dens_soglia1, self.dens_soglia2
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvRiferimentoNISECI {
    pub nome_comune: String,
    pub nome_latino: String,
    pub codice_specie: String,
    pub origine: String,
    pub tipo_autoctono: u32,
    pub allo_nocivita: u32,
    pub specie_attesa: u32,
    pub cl_soglia1: u32, // in mm
    pub cl_soglia2: u32, // in mm
    pub cl_soglia3: u32, // in mm
    pub cl_soglia4: u32, // in mm
    pub ad_juv_soglia1: f32,
    pub ad_juv_soglia2: f32,
    pub ad_juv_soglia3: f32,
    pub ad_juv_soglia4: f32,
    pub dens_soglia1: f32,
    pub dens_soglia2: f32,
}

impl RecordCsvRiferimentoNISECI for PlainRecordCsvRiferimentoNISECI {
    fn nome_comune(&self) -> String {
        self.nome_comune.clone()
    }
    fn nome_latino(&self) -> String {
        self.nome_latino.clone()
    }
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn origine(&self) -> String {
        self.origine.clone()
    }
    fn tipo_autoctono(&self) -> u32 {
        self.tipo_autoctono
    }
    fn allo_nocivita(&self) -> u32 {
        self.allo_nocivita
    }
    fn specie_attesa(&self) -> u32 {
        self.specie_attesa
    }
    fn cl_soglia1(&self) -> u32 {
        self.cl_soglia1
    }
    fn cl_soglia2(&self) -> u32 {
        self.cl_soglia2
    }
    fn cl_soglia3(&self) -> u32 {
        self.cl_soglia3
    }
    fn cl_soglia4(&self) -> u32 {
        self.cl_soglia4
    }
    fn ad_juv_soglia1(&self) -> f32 {
        self.ad_juv_soglia1
    }
    fn ad_juv_soglia2(&self) -> f32 {
        self.ad_juv_soglia2
    }
    fn ad_juv_soglia3(&self) -> f32 {
        self.ad_juv_soglia3
    }
    fn ad_juv_soglia4(&self) -> f32 {
        self.ad_juv_soglia4
    }
    fn dens_soglia1(&self) -> f32 {
        self.dens_soglia1
    }
    fn dens_soglia2(&self) -> f32 {
        self.dens_soglia2
    }
}

impl fmt::Display for PlainRecordCsvRiferimentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvRiferimentoNISECI: {{ nome_comune: [{}], nome_latino: [{}], codice_specie: [{}], origine: [{}], tipo_autoctono: [{}], allo_nocivita: [{}], specie_attesa: [{}], cl_soglia1: [{}], cl_soglia2: [{}], cl_soglia3: [{}], cl_soglia4: [{}], ad_juv_soglia1: [{}], ad_juv_soglia2: [{}], ad_juv_soglia3: [{}], ad_juv_soglia4: [{}], dens_soglia1: [{}], dens_soglia2: [{}] }}",
              self.nome_comune, self.nome_latino, self.codice_specie, self.origine,
              self.tipo_autoctono, self.allo_nocivita, self.specie_attesa,
              self.cl_soglia1, self.cl_soglia2, self.cl_soglia3, self.cl_soglia4,
              self.ad_juv_soglia1, self.ad_juv_soglia2, self.ad_juv_soglia3, self.ad_juv_soglia4,
              self.dens_soglia1, self.dens_soglia2
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_riferimento_niseci<R, T>(
    mut rdr: csv::Reader<R>,
) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvRiferimentoNISECI,
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

pub fn check_riferimento_niseci_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvRiferimentoNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvRiferimentoNISECI>() => b';',
        _ => b',',
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_riferimento_niseci(rdr);

    println!(
        "Riferimento NISECI: Numero record csv validi: {}",
        records.len()
    );
    println!(
        "Riferimento NISECI: Numero record csv non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        */
        let processed_errors = process_csv_errors(&errors, TipoRecordCsv::RiferimentoNISECI);
        eprintln!("Errori incontrati durante l'elaborazione csv del riferimento NISECI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record csv del riferimento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub fn check_riferimento_niseci_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvRiferimentoNISECI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore riferimento NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_riferimento_niseci_reader(file, has_headers)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCsvCampionamentoNISECI {
    pub data: String,
    pub stazione: String,
    pub num_passaggio: u32,
    pub codice_specie: String,
    pub lunghezza: u32,
    pub peso: u32,
}

impl RecordCsvCampionamentoNISECI for VeryItalianRecordCsvCampionamentoNISECI {
    fn data(&self) -> String {
        self.data.clone()
    }
    fn stazione(&self) -> String {
        self.stazione.clone()
    }
    fn num_passaggio(&self) -> u32 {
        self.num_passaggio
    }
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn lunghezza(&self) -> u32 {
        self.lunghezza
    }
    fn peso(&self) -> u32 {
        self.peso
    }
}

impl fmt::Display for VeryItalianRecordCsvCampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvCampionamentoNISECI: {{ data: [{}], stazione: [{}], num_passaggio: [{}], codice_specie: [{}], lunghezza: [{}], peso: [{}] }}",
              self.data, self.stazione, self.num_passaggio,
              self.codice_specie, self.lunghezza, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvCampionamentoNISECI {
    pub data: String,
    pub stazione: String,
    pub num_passaggio: u32,
    pub codice_specie: String,
    pub lunghezza: u32,
    pub peso: u32,
}

impl RecordCsvCampionamentoNISECI for PlainRecordCsvCampionamentoNISECI {
    fn data(&self) -> String {
        self.data.clone()
    }
    fn stazione(&self) -> String {
        self.stazione.clone()
    }
    fn num_passaggio(&self) -> u32 {
        self.num_passaggio
    }
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn lunghezza(&self) -> u32 {
        self.lunghezza
    }
    fn peso(&self) -> u32 {
        self.peso
    }
}

impl fmt::Display for PlainRecordCsvCampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvCampionamentoNISECI: {{ data: [{}], stazione: [{}], num_passaggio: [{}], codice_specie: [{}], lunghezza: [{}], peso: [{}] }}",
              self.data, self.stazione, self.num_passaggio,
              self.codice_specie, self.lunghezza, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_campionamento_niseci<R, T>(
    mut rdr: csv::Reader<R>,
) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvCampionamentoNISECI + 'static,
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

pub fn check_campionamento_niseci_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvCampionamentoNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvCampionamentoNISECI>() => b';',
        _ => b',',
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_campionamento_niseci(rdr);

    println!(
        "Campionamento NISECI: Numero record csv validi: {}",
        records.len()
    );
    println!(
        "Campionamento NISECI: Numero record csv non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        */
        let processed_errors = process_csv_errors(&errors, TipoRecordCsv::CampionamentoNISECI);
        eprintln!("Errori incontrati durante l'elaborazione csv del campionamento NISECI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record csv del campionamento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub fn check_campionamento_niseci_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvCampionamentoNISECI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore campionamento NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_campionamento_niseci_reader(file, has_headers)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCsvAnagraficaNISECI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub lunghezza_stazione: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub larghezza_stazione: f32,
    pub tipo_comunita: u32,
    pub fonte: String,
    pub numero_protocollo: String,
    pub idro_eco_regione: u32,
    pub area_alpina: u32,
    pub nome_bacino: String,
}

impl RecordCsvAnagraficaNISECI for VeryItalianRecordCsvAnagraficaNISECI {
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
    fn tipo_comunita(&self) -> u32 {
        self.tipo_comunita
    }
    fn fonte(&self) -> String {
        self.fonte.clone()
    }
    fn numero_protocollo(&self) -> String {
        self.numero_protocollo.clone()
    }
    fn idro_eco_regione(&self) -> u32 {
        self.idro_eco_regione
    }
    fn area_alpina(&self) -> u32 {
        self.area_alpina
    }
    fn nome_bacino(&self) -> String {
        self.nome_bacino.clone()
    }
}

impl fmt::Display for VeryItalianRecordCsvAnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaNISECI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], tipo_comunita [{}], fonte [{}],\
            numero_protocollo: [{}], idro_eco_regione: [{}],\
            area_alpina: [{}], nome_bacino: [{}]}}",
            self.codice_stazione,
            self.corpo_idrico,
            self.regione,
            self.provincia,
            self.data,
            self.lunghezza_stazione,
            self.larghezza_stazione,
            self.tipo_comunita,
            self.fonte,
            self.numero_protocollo,
            self.idro_eco_regione,
            self.area_alpina,
            self.nome_bacino
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvAnagraficaNISECI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    pub lunghezza_stazione: f32,
    pub larghezza_stazione: f32,
    pub tipo_comunita: u32,
    pub fonte: String,
    pub numero_protocollo: String,
    pub idro_eco_regione: u32,
    pub area_alpina: u32,
    pub nome_bacino: String,
}

impl RecordCsvAnagraficaNISECI for PlainRecordCsvAnagraficaNISECI {
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
    fn tipo_comunita(&self) -> u32 {
        self.tipo_comunita
    }
    fn fonte(&self) -> String {
        self.fonte.clone()
    }
    fn numero_protocollo(&self) -> String {
        self.numero_protocollo.clone()
    }
    fn idro_eco_regione(&self) -> u32 {
        self.idro_eco_regione
    }
    fn area_alpina(&self) -> u32 {
        self.area_alpina
    }
    fn nome_bacino(&self) -> String {
        self.nome_bacino.clone()
    }
}

impl fmt::Display for PlainRecordCsvAnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaNISECI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], tipo_comunita [{}], fonte [{}],\
            numero_protocollo: [{}], idro_eco_regione: [{}],\
            area_alpina: [{}], nome_bacino: [{}]}}",
            self.codice_stazione,
            self.corpo_idrico,
            self.regione,
            self.provincia,
            self.data,
            self.lunghezza_stazione,
            self.larghezza_stazione,
            self.tipo_comunita,
            self.fonte,
            self.numero_protocollo,
            self.idro_eco_regione,
            self.area_alpina,
            self.nome_bacino
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_anagrafica_niseci<R, T>(
    mut rdr: csv::Reader<R>,
) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvAnagraficaNISECI,
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

pub fn check_anagrafica_niseci_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvAnagraficaNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvAnagraficaNISECI>() => b';',
        _ => b',',
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(has_headers)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_anagrafica_niseci(rdr);

    println!(
        "Anagrafica NISECI: Numero record csv validi: {}",
        records.len()
    );
    println!(
        "Anagrafica NISECI: Numero record csv non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        */
        let processed_errors = process_csv_errors(&errors, TipoRecordCsv::AnagraficaNISECI);
        eprintln!("Errori incontrati durante l'elaborazione csv dell' anagrafica NISECI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record csv dell'anagrafica NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(records)
    }
}

pub fn check_anagrafica_niseci_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvAnagraficaNISECI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore anagrafica NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_anagrafica_niseci_reader(file, has_headers)
}
