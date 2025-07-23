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
    ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_NISECI_HEADER_FIELDS,
    CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_NISECI_HEADER_FIELDS,
    RIFERIMENTO_NISECI_HEADER_FIELDS,
};
use crate::core::csv::{
    RecordCsvAnagraficaHFBI, RecordCsvAnagraficaNISECI, RecordCsvCampionamentoHFBI,
    RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI, TipoRecordCsv,
};
use serde::{de, Deserialize, Deserializer};
use std::any::TypeId;
use std::fmt;
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

struct NormalizerReader<R: Read> {
    inner: R,
}

impl<R: Read> NormalizerReader<R> {
    fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: Read> Read for NormalizerReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.read(buf)?;

        // Change very italian accented characters in place
        for byte in buf.iter_mut().take(size) {
            match *byte {
                b'\xF2' => *byte = b'o',
                b'\xE0' => *byte = b'a',
                b'\xE8' => *byte = b'e',
                b'\xF9' => *byte = b'u',
                b'\xEC' => *byte = b'i',
                _ => {}
            }
        }

        Ok(size)
    }
}

fn deserialize_comma_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let s = s.replace(',', "."); // Replace comma with dot
    s.parse::<f32>().map_err(de::Error::custom)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VeryItalianRecordCsvRiferimentoNISECI {
    pub(crate) nome_comune: String,
    pub(crate) nome_latino: String,
    pub(crate) codice_specie: String,
    pub(crate) origine: String,
    pub(crate) tipo_autoctono: u32,
    pub(crate) allo_nocivita: u32,
    pub(crate) specie_attesa: u32,
    pub(crate) cl_soglia1: u32, // in mm
    pub(crate) cl_soglia2: u32, // in mm
    pub(crate) cl_soglia3: u32, // in mm
    pub(crate) cl_soglia4: u32, // in mm
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) ad_juv_soglia1: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) ad_juv_soglia2: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) ad_juv_soglia3: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) ad_juv_soglia4: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) dens_soglia1: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) dens_soglia2: f32,
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
pub(crate) struct PlainRecordCsvRiferimentoNISECI {
    pub(crate) nome_comune: String,
    pub(crate) nome_latino: String,
    pub(crate) codice_specie: String,
    pub(crate) origine: String,
    pub(crate) tipo_autoctono: u32,
    pub(crate) allo_nocivita: u32,
    pub(crate) specie_attesa: u32,
    pub(crate) cl_soglia1: u32, // in mm
    pub(crate) cl_soglia2: u32, // in mm
    pub(crate) cl_soglia3: u32, // in mm
    pub(crate) cl_soglia4: u32, // in mm
    pub(crate) ad_juv_soglia1: f32,
    pub(crate) ad_juv_soglia2: f32,
    pub(crate) ad_juv_soglia3: f32,
    pub(crate) ad_juv_soglia4: f32,
    pub(crate) dens_soglia1: f32,
    pub(crate) dens_soglia2: f32,
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

pub(crate) fn parse_csv_riferimento_niseci<R, T>(
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

pub(crate) fn check_riferimento_niseci_reader<R: Read, T>(
    reader: R,
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

pub(crate) fn check_riferimento_niseci_path<T>(path: PathBuf) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvRiferimentoNISECI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(
            ErrorKind::Other,
            "Errore riferimento NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_riferimento_niseci_reader(file)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VeryItalianRecordCsvCampionamentoNISECI {
    pub(crate) data: String,
    pub(crate) stazione: String,
    pub(crate) num_passaggio: u32,
    pub(crate) codice_specie: String,
    pub(crate) lunghezza: u32,
    pub(crate) peso: u32,
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
pub(crate) struct PlainRecordCsvCampionamentoNISECI {
    pub(crate) data: String,
    pub(crate) stazione: String,
    pub(crate) num_passaggio: u32,
    pub(crate) codice_specie: String,
    pub(crate) lunghezza: u32,
    pub(crate) peso: u32,
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

pub(crate) fn parse_csv_campionamento_niseci<R, T>(
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

pub(crate) fn check_campionamento_niseci_reader<R: Read, T>(
    reader: R,
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

pub(crate) fn check_campionamento_niseci_path<T>(path: PathBuf) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvCampionamentoNISECI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(
            ErrorKind::Other,
            "Errore campionamento NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_campionamento_niseci_reader(file)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VeryItalianRecordCsvAnagraficaNISECI {
    pub(crate) codice_stazione: String,
    pub(crate) corpo_idrico: String,
    pub(crate) regione: String,
    pub(crate) provincia: String,
    pub(crate) data: String,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) lunghezza_stazione: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) larghezza_stazione: f32,
    pub(crate) tipo_comunita: u32,
    pub(crate) fonte: String,
    pub(crate) numero_protocollo: String,
    pub(crate) idro_eco_regione: u32,
    pub(crate) area_alpina: u32,
    pub(crate) nome_bacino: String,
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
pub(crate) struct PlainRecordCsvAnagraficaNISECI {
    pub(crate) codice_stazione: String,
    pub(crate) corpo_idrico: String,
    pub(crate) regione: String,
    pub(crate) provincia: String,
    pub(crate) data: String,
    pub(crate) lunghezza_stazione: f32,
    pub(crate) larghezza_stazione: f32,
    pub(crate) tipo_comunita: u32,
    pub(crate) fonte: String,
    pub(crate) numero_protocollo: String,
    pub(crate) idro_eco_regione: u32,
    pub(crate) area_alpina: u32,
    pub(crate) nome_bacino: String,
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

pub(crate) fn parse_csv_anagrafica_niseci<R, T>(
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

pub(crate) fn check_anagrafica_niseci_reader<R: Read, T>(
    reader: R,
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

pub(crate) fn check_anagrafica_niseci_path<T>(path: PathBuf) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvAnagraficaNISECI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(
            ErrorKind::Other,
            "Errore anagrafica NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_anagrafica_niseci_reader(file)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VeryItalianRecordCsvCampionamentoHFBI {
    pub(crate) codice_specie: String,
    pub(crate) numero_individui: u32,
    pub(crate) peso: u32,
}

impl RecordCsvCampionamentoHFBI for VeryItalianRecordCsvCampionamentoHFBI {
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn numero_individui(&self) -> u32 {
        self.numero_individui
    }
    fn peso(&self) -> u32 {
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
pub(crate) struct PlainRecordCsvCampionamentoHFBI {
    pub(crate) codice_specie: String,
    pub(crate) numero_individui: u32,
    pub(crate) peso: u32,
}

impl RecordCsvCampionamentoHFBI for PlainRecordCsvCampionamentoHFBI {
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn numero_individui(&self) -> u32 {
        self.numero_individui
    }
    fn peso(&self) -> u32 {
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

pub(crate) fn parse_csv_campionamento_hfbi<R, T>(
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

pub(crate) fn check_campionamento_hfbi_reader<R: Read, T>(
    reader: R,
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

pub(crate) fn check_campionamento_hfbi_path<T>(path: PathBuf) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvCampionamentoHFBI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(
            ErrorKind::Other,
            "Errore campionamento HFBI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_campionamento_hfbi_reader(file)
}

fn parse_csv_pos(pos: &Option<csv::Position>) -> String {
    let res;
    match pos {
        Some(p) => {
            // These should be equal. We may show the value only once if they are
            let line_offset = p.line();
            let record_offset = p.record();

            if line_offset == record_offset {
                res = format!("Riga: {}", line_offset);
            } else {
                // TODO: How can we hit this branch?
                res = format!("Riga: {} Record: {}", line_offset, record_offset);
            }

            // We ignore this since I don't think users may care?
            // let byte_offset = p.byte();
            // res = format!("Riga: {} Record: {} Char: {} ", line_offset, record_offset, byte_offset);
        }
        None => {
            res = "none".to_string();
        }
    }
    res
}

pub(crate) fn process_csv_errors(errors: &Vec<csv::Error>, tipo_csv: TipoRecordCsv) -> Vec<String> {
    let mut res = Vec::new();
    for error in errors {
        match error.kind() {
            csv::ErrorKind::Deserialize { pos, err } => {
                let field_str;
                match err.field() {
                    Some(f) => {
                        // Deduce name for field from index in the header
                        // f is u64 starting from 0
                        let field_idx = f as usize;
                        match tipo_csv {
                            TipoRecordCsv::RiferimentoNISECI => {
                                if field_idx < RIFERIMENTO_NISECI_HEADER_FIELDS.len() {
                                    field_str = format!(
                                        "{} ({})",
                                        field_idx, RIFERIMENTO_NISECI_HEADER_FIELDS[field_idx]
                                    );
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                            TipoRecordCsv::CampionamentoNISECI => {
                                if field_idx < CAMPIONAMENTO_NISECI_HEADER_FIELDS.len() {
                                    field_str = format!(
                                        "{} ({})",
                                        field_idx, CAMPIONAMENTO_NISECI_HEADER_FIELDS[field_idx]
                                    );
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                            TipoRecordCsv::AnagraficaNISECI => {
                                if field_idx < ANAGRAFICA_NISECI_HEADER_FIELDS.len() {
                                    field_str = format!(
                                        "{} ({})",
                                        field_idx, ANAGRAFICA_NISECI_HEADER_FIELDS[field_idx]
                                    );
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                            TipoRecordCsv::CampionamentoHFBI => {
                                if field_idx < CAMPIONAMENTO_HFBI_HEADER_FIELDS.len() {
                                    field_str = format!(
                                        "{} ({})",
                                        field_idx, CAMPIONAMENTO_HFBI_HEADER_FIELDS[field_idx]
                                    );
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                            TipoRecordCsv::AnagraficaHFBI => {
                                if field_idx < ANAGRAFICA_HFBI_HEADER_FIELDS.len() {
                                    field_str = format!(
                                        "{} ({})",
                                        field_idx, ANAGRAFICA_HFBI_HEADER_FIELDS[field_idx]
                                    );
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                        }
                    }
                    None => {
                        field_str = "none".to_string();
                    }
                }
                let mut curr_err = format!(
                    "  Errore di deserializzazione alla posizione: {}: campo {}",
                    parse_csv_pos(pos),
                    field_str,
                );
                match err.kind() {
                    csv::DeserializeErrorKind::Message(msg) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(msg));
                    }
                    csv::DeserializeErrorKind::Unsupported(msg) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(msg));
                    }
                    csv::DeserializeErrorKind::UnexpectedEndOfRow => {
                        curr_err = format!("{curr_err}: Fine riga inatteso");
                    }
                    csv::DeserializeErrorKind::InvalidUtf8(utf8err) => {
                        curr_err = format!(
                            "{curr_err}: {}",
                            translate_error_message(&utf8err.to_string())
                        );
                    }
                    csv::DeserializeErrorKind::ParseBool(boolerr) => {
                        curr_err = format!(
                            "{curr_err}: {}",
                            translate_error_message(&boolerr.to_string())
                        );
                    }
                    csv::DeserializeErrorKind::ParseInt(interr) => {
                        curr_err = format!(
                            "{curr_err}: {}",
                            translate_error_message(&interr.to_string())
                        );
                    }
                    csv::DeserializeErrorKind::ParseFloat(floaterr) => {
                        curr_err = format!(
                            "{curr_err}: {}",
                            translate_error_message(&floaterr.to_string())
                        );
                    }
                }
                res.push(curr_err);
            }
            csv::ErrorKind::Io(io_error) => {
                res.push(format!(
                    "  Errore di I/O: {}",
                    translate_error_message(&io_error.to_string())
                ));
            }
            csv::ErrorKind::Utf8 { pos, err } => {
                res.push(format!(
                    "  Errore UTF-8 alla posizione: {}: {}",
                    parse_csv_pos(pos),
                    translate_error_message(&err.to_string())
                ));
            }
            csv::ErrorKind::UnequalLengths {
                pos,
                expected_len,
                len,
            } => {
                res.push(format!(
                    "  Errore numero campi alla posizione: {}: lunghezza attesa {}, trovata {}",
                    parse_csv_pos(pos),
                    expected_len,
                    len // no translate_error_message() anche se teoricamente lo supporta
                ));
            }
            _ => {
                res.push(format!(
                    "  Errore sconosciuto: {}",
                    translate_error_message(&error.to_string())
                ));
            }
        }
    }
    res
}

pub(crate) fn check_path_is_file_ends_with_csv(path: &Path) -> bool {
    if !path.exists() {
        eprintln!("Error: Passed path does not exist");
        false
    } else if !path.is_file() {
        eprintln!("Error: Passed path is not a regular file");
        false
    } else {
        let ext = path.extension();
        match ext {
            Some(ex) => {
                if !(ex == "csv" || ex == "CSV") {
                    eprintln!("Error: Passed path does not end with .csv");
                    return false;
                }
                true
            }
            None => {
                eprintln!("Error: Passed path does not end with .csv");
                false
            }
        }
    }
}

pub(crate) fn translate_error_message(msg: &str) -> String {
    if msg.starts_with("missing field") {
        msg.replace("missing field", "campo mancante")
    } else if msg.starts_with("invalid type") {
        msg.replace("invalid type", "tipo non valido")
    } else if msg.starts_with("unexpected end of input") {
        msg.replace("unexpected end of input", "fine inaspettata dell'input")
    } else if msg.contains("invalid UTF-8 sequence") {
        msg.replace("invalid UTF-8 sequence", "sequenza UTF-8 non valida")
    } else if msg.contains("file not found") {
        msg.replace("file not found", "file non trovato")
    } else if msg.contains("invalid digit found in string") {
        msg.replace(
            "invalid digit found in string",
            "tipo non valido: numero, attesa stringa",
        )
        .replace("field", "campo")
    } else if msg.contains("invalid float literal") {
        msg.replace("invalid float literal", "tipo non valido: atteso decimale")
            .replace("field", "campo")
    } else if msg.contains("cannot parse") && msg.contains("from empty string") {
        // NOTE: there's a leading space in " from empty string", it enables us to attach the ","
        // to the previous part
        msg.replace("cannot parse", "campo vuoto: atteso")
            .replace("field", "campo")
            .replace("float", "decimale")
            .replace("integer", "intero")
            .replace(" from empty string", ", trovato: stringa vuota")
    } else if msg.contains("fields, but the previous record has") {
        msg.replace("found record with", "numero campi: trovato record con")
            .replace("but the previous record has", "ma il record precedente ha")
            .replace("fields", "campi")
    } else {
        eprintln!("Unmatched translation for {msg}");
        msg.to_string() // Default to original message if no match
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct VeryItalianRecordCsvAnagraficaHFBI {
    pub(crate) codice_stazione: String,
    pub(crate) corpo_idrico: String,
    pub(crate) regione: String,
    pub(crate) provincia: String,
    pub(crate) data: String,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) lunghezza_stazione: f32,
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub(crate) larghezza_stazione: f32,
    pub(crate) stagione: u32,
    pub(crate) habitat: u32,
    pub(crate) tipo_laguna: u32,
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
pub(crate) struct PlainRecordCsvAnagraficaHFBI {
    pub(crate) codice_stazione: String,
    pub(crate) corpo_idrico: String,
    pub(crate) regione: String,
    pub(crate) provincia: String,
    pub(crate) data: String,
    pub(crate) lunghezza_stazione: f32,
    pub(crate) larghezza_stazione: f32,
    pub(crate) stagione: u32,
    pub(crate) habitat: u32,
    pub(crate) tipo_laguna: u32,
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

pub(crate) fn parse_csv_anagrafica_hfbi<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
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

pub(crate) fn check_anagrafica_hfbi_reader<R: Read, T>(reader: R) -> Result<Vec<T>, Vec<csv::Error>>
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

pub(crate) fn check_anagrafica_hfbi_path<T>(path: PathBuf) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCsvAnagraficaHFBI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(
            ErrorKind::Other,
            "Errore anagrafica HFBI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_anagrafica_hfbi_reader(file)
}
