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

use std::fmt;
use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use std::any::TypeId;
use serde::Deserialize;
use serde::de::{self, Deserializer};
use chrono::NaiveDate;
use chrono::format::ParseErrorKind;
use crate::domain::niseci::{SpecieNISECI, RecordNISECI, AnagraficaNISECI, AreaNISECI, TipoComunitaNISECI, ComunitaNISECI, IdroEcoRegioneNISECI};
use crate::domain::location::Location;

// This must be kept aligned with RecordCsvRiferimentoNISECI definition.
// TODO: get this stuff with some macro?
pub(crate) const RIFERIMENTO_NISECI_HEADER_FIELDS: [&str; 17] = [ "nomeComune", "nomeLatino", "codiceSpecie", "origine", "tipoAutoctono", "alloNocivita", "specieAttesa", "clSoglia1", "clSoglia2", "clSoglia3", "clSoglia4", "adJuvSoglia1", "adJuvSoglia2", "adJuvSoglia3", "adJuvSoglia4", "densSoglia1", "densSoglia2" ];
pub(crate) const RIFERIMENTO_NISECI_HEADER_FIELD_TYPES: [&str; 17] = [ "String", "String", "String", "String", "u32", "u32", "u32", "u32", "u32", "u32", "u32", "f32", "f32", "f32", "f32", "f32", "f32" ];
pub(crate) const RIFERIMENTO_NISECI_HEADER: &str = "\
nomeComune;nomeLatino;codiceSpecie;origine;tipoAutoctono;alloNocivita;specieAttesa;clSoglia1;clSoglia2;clSoglia3;clSoglia4;adJuvSoglia1;adJuvSoglia2;adJuvSoglia3;adJuvSoglia4;densSoglia1;densSoglia2";

// This must be kept aligned with RecordCsvCampionamentoNISECI definition.
// TODO: get this stuff with some macro?
pub(crate) const CAMPIONAMENTO_NISECI_HEADER_FIELDS: [&str; 6] = [ "data", "stazione", "numPassaggio", "codiceSpecie", "lunghezza", "peso" ];
pub(crate) const CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES: [&str; 6] = [ "String", "String", "u32", "String", "u32", "u32" ];
pub(crate) const CAMPIONAMENTO_NISECI_HEADER: &str = "\
data;stazione;numPassaggio;codiceSpecie;lunghezza;peso";

// This must be kept aligned with RecordCsvAnagraficaNISECI definition.
// TODO: get this stuff with some macro?
pub(crate) const ANAGRAFICA_NISECI_HEADER_FIELDS: [&str; 13] = [
"codiceStazione", "corpoIdrico", "regione", "provincia", "data", "lunghezzaStazione", "larghezzaStazione", "tipoComunita", "fonte", "numeroProtocollo", "idroEcoRegione", "areaAlpina", "nomeBacino" ];
pub(crate) const ANAGRAFICA_NISECI_HEADER_FIELD_TYPES: [&str; 13] = [ "String", "String", "String", "String", "String", "f32", "f32", "u32", "String", "String", "u32", "u32", "String"];
pub(crate) const ANAGRAFICA_NISECI_HEADER: &str = "\
codiceStazione;corpoIdrico;regione;provincia;data;lunghezzaStazione;larghezzaStazione;tipoComunita;fonte;numeroProtocollo;idroEcoRegione;areaAlpina;nomeBacino";

pub(crate) fn parse_date(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    let normalized = date_str.replace("/", "-"); // Replace all / with -
    NaiveDate::parse_from_str(&normalized, "%d-%m-%Y")
}

#[derive(Copy,Clone)]
pub(crate) enum TipoRecordCsv {
    RiferimentoNISECI,
    CampionamentoNISECI,
    AnagraficaNISECI,
}

fn deserialize_comma_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let s = s.replace(',', "."); // Replace comma with dot
    s.parse::<f32>().map_err(de::Error::custom)
}

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

pub(crate) trait RecordCsvRiferimentoNISECI: serde::de::DeserializeOwned {
    #[allow(dead_code)]
    fn nome_comune(&self) -> String;
    fn nome_latino(&self) -> String;
    fn codice_specie(&self) -> String;
    fn origine(&self) -> String;
    fn tipo_autoctono(&self) -> u32;
    fn allo_nocivita(&self) -> u32;
    fn specie_attesa(&self) -> u32;
    fn cl_soglia1(&self) -> u32;
    fn cl_soglia2(&self) -> u32;
    fn cl_soglia3(&self) -> u32;
    fn cl_soglia4(&self) -> u32;
    fn ad_juv_soglia1(&self) -> f32;
    fn ad_juv_soglia2(&self) -> f32;
    fn ad_juv_soglia3(&self) -> f32;
    fn ad_juv_soglia4(&self) -> f32;
    fn dens_soglia1(&self) -> f32;
    fn dens_soglia2(&self) -> f32;
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
    fn nome_comune(&self) -> String { self.nome_comune.clone() }
    fn nome_latino(&self) -> String { self.nome_latino.clone() }
    fn codice_specie(&self) -> String { self.codice_specie.clone() }
    fn origine(&self) -> String { self.origine.clone() }
    fn tipo_autoctono(&self) -> u32 { self.tipo_autoctono }
    fn allo_nocivita(&self) -> u32 { self.allo_nocivita }
    fn specie_attesa(&self) -> u32 { self.specie_attesa }
    fn cl_soglia1(&self) -> u32 { self.cl_soglia1 }
    fn cl_soglia2(&self) -> u32 { self.cl_soglia2 }
    fn cl_soglia3(&self) -> u32 { self.cl_soglia3 }
    fn cl_soglia4(&self) -> u32 { self.cl_soglia4 }
    fn ad_juv_soglia1(&self) -> f32 { self.ad_juv_soglia1 }
    fn ad_juv_soglia2(&self) -> f32 { self.ad_juv_soglia2 }
    fn ad_juv_soglia3(&self) -> f32 { self.ad_juv_soglia3 }
    fn ad_juv_soglia4(&self) -> f32 { self.ad_juv_soglia4 }
    fn dens_soglia1(&self) -> f32 { self.dens_soglia1 }
    fn dens_soglia2(&self) -> f32 { self.dens_soglia2 }
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
    fn nome_comune(&self) -> String { self.nome_comune.clone() }
    fn nome_latino(&self) -> String { self.nome_latino.clone() }
    fn codice_specie(&self) -> String { self.codice_specie.clone() }
    fn origine(&self) -> String { self.origine.clone() }
    fn tipo_autoctono(&self) -> u32 { self.tipo_autoctono }
    fn allo_nocivita(&self) -> u32 { self.allo_nocivita }
    fn specie_attesa(&self) -> u32 { self.specie_attesa }
    fn cl_soglia1(&self) -> u32 { self.cl_soglia1 }
    fn cl_soglia2(&self) -> u32 { self.cl_soglia2 }
    fn cl_soglia3(&self) -> u32 { self.cl_soglia3 }
    fn cl_soglia4(&self) -> u32 { self.cl_soglia4 }
    fn ad_juv_soglia1(&self) -> f32 { self.ad_juv_soglia1 }
    fn ad_juv_soglia2(&self) -> f32 { self.ad_juv_soglia2 }
    fn ad_juv_soglia3(&self) -> f32 { self.ad_juv_soglia3 }
    fn ad_juv_soglia4(&self) -> f32 { self.ad_juv_soglia4 }
    fn dens_soglia1(&self) -> f32 { self.dens_soglia1 }
    fn dens_soglia2(&self) -> f32 { self.dens_soglia2 }
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

pub(crate) trait RecordCsvCampionamentoNISECI: serde::de::DeserializeOwned {
    #[allow(dead_code)]
    fn data(&self) -> String;
    #[allow(dead_code)]
    fn stazione(&self) -> String;
    fn num_passaggio(&self) -> u32;
    fn codice_specie(&self) -> String;
    fn lunghezza(&self) -> u32;
    fn peso(&self) -> u32;
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
    fn data(&self) -> String { self.data.clone() }
    fn stazione(&self) -> String { self.stazione.clone() }
    fn num_passaggio(&self) -> u32 { self.num_passaggio }
    fn codice_specie(&self) -> String { self.codice_specie.clone() }
    fn lunghezza(&self) -> u32 { self.lunghezza }
    fn peso(&self) -> u32 { self.peso }
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
    fn data(&self) -> String { self.data.clone() }
    fn stazione(&self) -> String { self.stazione.clone() }
    fn num_passaggio(&self) -> u32 { self.num_passaggio }
    fn codice_specie(&self) -> String { self.codice_specie.clone() }
    fn lunghezza(&self) -> u32 { self.lunghezza }
    fn peso(&self) -> u32 { self.peso }
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

pub(crate) fn parse_csv_campionamento_niseci<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvCampionamentoNISECI + 'static
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

#[derive(Debug)]
pub(crate) enum RecordCsvCampionamentoNISECIError {
    ValoreInvalido { msg : String }, //TODO: add position?
}

impl fmt::Display for RecordCsvCampionamentoNISECIError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match self {
      RecordCsvCampionamentoNISECIError::ValoreInvalido { msg } => format!("Errore record campionamento NISECI: {}", msg),
    };
    write!(f, "{}", string_representation)
  }
}

pub(crate) fn parse_recordcsv_campionamento_niseci<T: RecordCsvCampionamentoNISECI>(records: Vec<T>, riferimento_specie: Vec<SpecieNISECI>) -> (Vec<RecordNISECI>,Vec<RecordCsvCampionamentoNISECIError>) {
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        if r.codice_specie().is_empty() {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (lunghezza < 1)") };
            errors.push(err);
            continue;
        }
        let codice_specie = r.codice_specie();
        let mut opt_matched_specie = None;
        for s in &riferimento_specie { // FIXME: this is O(n^2).
            if s.id == codice_specie {
                opt_matched_specie = Some(s);
                break; // TODO: mmmh
            }
        }


        let matched_specie;
        if let Some(specie) = opt_matched_specie {
            matched_specie = specie;
        } else {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (non presente nel riferimento): {}", codice_specie) };
            errors.push(err);
            continue;
        }


        //TODO: update this abomination when records change to have an integer directly
        if r.num_passaggio() < 1 {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: num_passaggio non valido (<1): {}", r.num_passaggio()) };
            errors.push(err);
            continue;
        }
        let passaggio_cattura = r.num_passaggio();

        let niseci_rec = RecordNISECI {
            specie: matched_specie.clone(),
            passaggio_cattura: passaggio_cattura as u8,
            lunghezza: r.lunghezza(),
            peso: r.peso()
        };
        campioni.push(niseci_rec);
    }
    (campioni, errors)
}

pub(crate) fn parse_csv_riferimento_niseci<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvRiferimentoNISECI
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

#[derive(Debug)]
pub(crate) enum RecordCsvRiferimentoNISECIError {
    ValoreInvalido { msg : String }, //TODO: add position?
    SoglieCLNonCrescenti { msg : String },
    SoglieADJUVNonCrescenti { msg: String }
}

impl fmt::Display for RecordCsvRiferimentoNISECIError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
let string_representation = match self {
      RecordCsvRiferimentoNISECIError::ValoreInvalido { msg } => format!("Errore record riferimento NISECI: {}", msg),
      RecordCsvRiferimentoNISECIError::SoglieCLNonCrescenti { msg } => format!("Errore record riferimento NISECI: {}", msg),
      RecordCsvRiferimentoNISECIError::SoglieADJUVNonCrescenti { msg } => format!("Errore record riferimento NISECI: {}", msg),
    };
    write!(f, "{}", string_representation)
  }
}

pub(crate) fn parse_recordcsv_riferimento_niseci<T: RecordCsvRiferimentoNISECI>(records: Vec<T>) -> (Vec<SpecieNISECI>,Vec<RecordCsvRiferimentoNISECIError>)
{
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
            },
            "AUT" => {},
            _ => {
                let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: origine invalida (non \"AUT\" o \"ALL\"): {}", r.origine()) };
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
                    let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: tipo_autoctono non valido (non 1 o 2): {}", r.tipo_autoctono()) };
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
                    let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: allo_nocivita non valido (non [0..3]): {}", r.allo_nocivita()) };
                    errors.push(err);
                    continue;
                }
            }
        }

        if r.codice_specie().is_empty() {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (lunghezza < 1)") };
            errors.push(err);
            continue;
        }

        let id = r.codice_specie();

        if used_id_specie.contains(&id) {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (ridefinizione)") };
            errors.push(err);
            continue;
        }

        let nome =  r.nome_latino(); //TODO: controllare se dovrebbe essere nome_comune

        //TODO: update when SpecieNISECI has the missing fields

        let epsilon: f32 = 1e-6;

        // Check dens_soglia
        if r.dens_soglia1() < 0.0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia1 non valido (< 0)") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1().abs() < epsilon && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia1 non valido (== 0) per una specie attesa") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2() < 0.0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia2 non valido (< 0)") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2().abs() < epsilon && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia2 non valido (== 0) per una specie attesa") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1() >= r.dens_soglia2() && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia1 maggiore di dens_soglia2 per una specie attesa") };
            errors.push(err);
            continue;
        }

        if !check_soglie_cl(&r) {
            let err = RecordCsvRiferimentoNISECIError::SoglieCLNonCrescenti {
                msg: format!("Record {idx}: soglie CL non crescenti" )
            };
            errors.push(err);
            continue;
        }
        if !check_soglie_ad_juv(&r) {
            let err = RecordCsvRiferimentoNISECIError::SoglieADJUVNonCrescenti {
                msg: format!("Record {idx}: soglie AD/JUV non crescenti" )
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

pub(crate) trait RecordCsvAnagraficaNISECI: serde::de::DeserializeOwned {
    fn codice_stazione(&self) -> String;
    fn corpo_idrico(&self) -> String;
    fn regione(&self) -> String;
    fn provincia(&self) -> String;
    fn data(&self) -> String;
    fn lunghezza_stazione(&self) -> f32;
    fn larghezza_stazione(&self) -> f32;
    fn tipo_comunita(&self) -> u32;
    fn fonte(&self) -> String;
    fn numero_protocollo(&self) -> String;
    fn idro_eco_regione(&self) -> u32;
    fn area_alpina(&self) -> u32;
    fn nome_bacino(&self) -> String;
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
    fn codice_stazione(&self) -> String { self.codice_stazione.clone() }
    fn corpo_idrico(&self) -> String { self.corpo_idrico.clone() }
    fn regione(&self) -> String { self.regione.clone() }
    fn provincia(&self) -> String { self.provincia.clone() }
    fn data(&self) -> String { self.data.clone() }
    fn lunghezza_stazione(&self) -> f32 { self.lunghezza_stazione }
    fn larghezza_stazione(&self) -> f32 { self.larghezza_stazione }
    fn tipo_comunita(&self) -> u32 { self.tipo_comunita }
    fn fonte(&self) -> String { self.fonte.clone() }
    fn numero_protocollo(&self) -> String { self.numero_protocollo.clone() }
    fn idro_eco_regione(&self) -> u32 { self.idro_eco_regione }
    fn area_alpina(&self) -> u32 { self.area_alpina }
    fn nome_bacino(&self) -> String { self.nome_bacino.clone() }
}

impl fmt::Display for VeryItalianRecordCsvAnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaNISECI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], tipo_comunita [{}], fonte [{}],\
            numero_protocollo: [{}], idro_eco_regione: [{}],\
            area_alpina: [{}], nome_bacino: [{}]}}",
            self.codice_stazione, self.corpo_idrico, self.regione, self.provincia,
            self.data, self.lunghezza_stazione, self.larghezza_stazione,
            self.tipo_comunita, self.fonte, self.numero_protocollo,
            self.idro_eco_regione, self.area_alpina, self.nome_bacino
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
    fn codice_stazione(&self) -> String { self.codice_stazione.clone() }
    fn corpo_idrico(&self) -> String { self.corpo_idrico.clone() }
    fn regione(&self) -> String { self.regione.clone() }
    fn provincia(&self) -> String { self.provincia.clone() }
    fn data(&self) -> String { self.data.clone() }
    fn lunghezza_stazione(&self) -> f32 { self.lunghezza_stazione }
    fn larghezza_stazione(&self) -> f32 { self.larghezza_stazione }
    fn tipo_comunita(&self) -> u32 { self.tipo_comunita }
    fn fonte(&self) -> String { self.fonte.clone() }
    fn numero_protocollo(&self) -> String { self.numero_protocollo.clone() }
    fn idro_eco_regione(&self) -> u32 { self.idro_eco_regione }
    fn area_alpina(&self) -> u32 { self.area_alpina }
    fn nome_bacino(&self) -> String { self.nome_bacino.clone() }
}

impl fmt::Display for PlainRecordCsvAnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaNISECI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], tipo_comunita [{}], fonte [{}],\
            numero_protocollo: [{}], idro_eco_regione: [{}],\
            area_alpina: [{}], nome_bacino: [{}]}}",
            self.codice_stazione, self.corpo_idrico, self.regione, self.provincia,
            self.data, self.lunghezza_stazione, self.larghezza_stazione,
            self.tipo_comunita, self.fonte, self.numero_protocollo,
            self.idro_eco_regione, self.area_alpina, self.nome_bacino
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug)]
pub(crate) enum RecordCsvAnagraficaNISECIError {
    ValoreInvalido { msg : String }, //TODO: add position?
}

impl fmt::Display for RecordCsvAnagraficaNISECIError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match self {
      RecordCsvAnagraficaNISECIError::ValoreInvalido { msg } => format!("Errore record anagrafica NISECI: {}", msg),
    };
    write!(f, "{}", string_representation)
  }
}


pub(crate) fn parse_csv_anagrafica_niseci<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCsvAnagraficaNISECI
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

pub(crate) fn parse_recordcsv_anagrafica_niseci<T: RecordCsvAnagraficaNISECI>(records: Vec<T>) -> Result<AnagraficaNISECI, Vec<RecordCsvAnagraficaNISECIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Troppi record: {}, atteso 1", records.len()) };
        errors.push(err);
    }
    if records.is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Nessun record trovato: atteso 1".to_string() };
        errors.push(err);
        return Err(errors);
    }

    let r = records.first().unwrap();

    if r.codice_stazione().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Codice stazione troppo corto: {}", r.codice_stazione()) };
        errors.push(err);
    }

    if r.corpo_idrico().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Corpo idrico troppo corto: {}", r.corpo_idrico()) };
        errors.push(err);
    }

    if r.regione().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Regione troppo corta: {}", r.regione()) };
        errors.push(err);
    }

    if r.provincia().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Provincia troppo corta: {}", r.provincia()) };
        errors.push(err);
    }

    match parse_date(&r.data()) {
        Ok(_) => {},
        Err(e) => {
            match e.kind() {
                ParseErrorKind::OutOfRange => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: fuori range".to_string() };
                    errors.push(err);
                },
                ParseErrorKind::Impossible => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: valori non possibili".to_string() };
                    errors.push(err);
                },
                ParseErrorKind::NotEnough => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: specifica insufficiente".to_string() };
                    errors.push(err);
                },
                ParseErrorKind::Invalid => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: presenza di caratteri non attesi".to_string() };
                    errors.push(err);
                },
                ParseErrorKind::TooShort => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: terminazione prematura dell'input".to_string() };
                    errors.push(err);
                },
                ParseErrorKind::TooLong => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: input in eccesso".to_string() };
                    errors.push(err);
                },
                ParseErrorKind::BadFormat => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: errore nella specifica di formattazione".to_string() };
                    errors.push(err);
                },
                _ => {
                    let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : "Data fornita non valida: errore sconosciuto".to_string() };
                    errors.push(err);
                }
            }
        }
    }

    if r.lunghezza_stazione() < 0.0 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Lunghezza stazione troppo bassa: {}", r.lunghezza_stazione()) };
        errors.push(err);
    }

    if r.larghezza_stazione() < 0.0 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Larghezza stazione troppo bassa: {}", r.larghezza_stazione()) };
        errors.push(err);
    }

    let mut tipo_comunita = TipoComunitaNISECI::Redatta;
    match r.tipo_comunita() {
        0 => { /* Redatta */ },
        1 => {
            tipo_comunita = TipoComunitaNISECI::Recuperata;
        },
        2 => {
            tipo_comunita = TipoComunitaNISECI::Dm260_2010;
        },
        3 => {
            tipo_comunita = TipoComunitaNISECI::AffinataDalMase;
        },
        _ => {
            let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Tipo comunita NISECI non valido: {}, atteso [0, 3]", r.tipo_comunita()) };
            errors.push(err);
        }
    }

    match tipo_comunita {
        TipoComunitaNISECI::Recuperata => {
            if r.fonte().is_empty() {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Fonte troppo corta: {}", r.fonte()) };
                errors.push(err);
            }
        }
        TipoComunitaNISECI::AffinataDalMase => {
            if r.numero_protocollo().is_empty() {
                let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Numero protocollo troppo corto: {}", r.numero_protocollo()) };
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
            let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("IdroEcoRegioneNISECI non valido: {}, atteso [0, 20]", r.idro_eco_regione()) };
            errors.push(err);
            IdroEcoRegioneNISECI::Toscana // To still assign something by default
        }
    };

    let mut area = AreaNISECI::Mediterranea;
    if r.area_alpina() > 0 {
        area = AreaNISECI::Alpina;
    }

    if r.nome_bacino().is_empty() {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Nome bacino troppo corto: {}", r.nome_bacino()) };
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
            provincia: r.provincia()
        },
        lunghezza_media_stazione: r.lunghezza_stazione(),
        larghezza_media_stazione: r.larghezza_stazione(),
    };
    Ok(res)

}

pub(crate) fn check_anagrafica_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>,Vec<csv::Error>>
where
    T: RecordCsvAnagraficaNISECI + 'static
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>();  // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvAnagraficaNISECI>() => { b';' },
        _ => { b',' }
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_anagrafica_niseci(rdr);

    println!("Anagrafica NISECI: Numero record csv validi: {}", records.len());
    println!("Anagrafica NISECI: Numero record csv non validi: {}", errors.len());

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

pub(crate) fn check_records_anagrafica_niseci<T: RecordCsvAnagraficaNISECI>(records: Vec<T>) -> Result<AnagraficaNISECI,Vec<RecordCsvAnagraficaNISECIError>> {

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
            println!("Anagrafica NISECI: Numero record non validi: {}", errors.len());
            eprintln!("Errori incontrati durante l'elaborazione dei record per anagrafica NISECI: {{");
            //TODO: add process_record_anagraficaNISECI_errors()
            for error in &errors {
                eprintln!("  {}", error);
            }
            eprintln!("}}");
            Err(errors)
        }
    }
}

pub(crate) fn check_anagrafica_niseci_path<T>(path: PathBuf) -> Result<Vec<T>,Vec<csv::Error>>
where
    T: RecordCsvAnagraficaNISECI + 'static
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(ErrorKind::Other, "Errore anagrafica NISECI: il file non è un .csv"));
        let err_vec: Vec<csv::Error> = vec!(err);
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_anagrafica_niseci_reader(file)
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
        msg.replace("invalid digit found in string", "tipo non valido: numero, attesa stringa")
            .replace("field", "campo")
    } else if msg.contains("invalid float literal") {
        msg.replace("invalid float literal", "tipo non valido: atteso decimale").replace("field", "campo")
    } else if msg.contains("cannot parse") && msg.contains("from empty string") {
        // NOTE: there's a leading space in " from empty string", it enables us to attach the ","
        // to the previous part
        msg.replace("cannot parse", "campo vuoto: atteso")
            .replace("field","campo")
            .replace("float","decimale")
            .replace("integer","intero")
            .replace(" from empty string",", trovato: stringa vuota")
    } else if msg.contains("fields, but the previous record has") {
        msg.replace("found record with","numero campi: trovato record con")
            .replace("but the previous record has", "ma il record precedente ha")
            .replace("fields", "campi")
    } else {
        eprintln!("Unmatched translation for {msg}");
        msg.to_string() // Default to original message if no match
    }
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
            } else { // TODO: How can we hit this branch?
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
                                    field_str = format!("{} ({})", field_idx, RIFERIMENTO_NISECI_HEADER_FIELDS[field_idx]);
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                            TipoRecordCsv::CampionamentoNISECI => {
                                if field_idx < CAMPIONAMENTO_NISECI_HEADER_FIELDS.len() {
                                    field_str = format!("{} ({})", field_idx, CAMPIONAMENTO_NISECI_HEADER_FIELDS[field_idx]);
                                } else {
                                    field_str = "???".to_string();
                                }
                            }
                            TipoRecordCsv::AnagraficaNISECI => {
                                if field_idx < ANAGRAFICA_NISECI_HEADER_FIELDS.len() {
                                    field_str = format!("{} ({})", field_idx, ANAGRAFICA_NISECI_HEADER_FIELDS[field_idx]);
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
                    csv::DeserializeErrorKind::Message( msg ) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(msg));
                    }
                    csv::DeserializeErrorKind::Unsupported( msg ) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(msg));
                    }
                    csv::DeserializeErrorKind::UnexpectedEndOfRow => {
                        curr_err = format!("{curr_err}: Fine riga inatteso");
                    }
                    csv::DeserializeErrorKind::InvalidUtf8 ( utf8err ) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(&utf8err.to_string()));
                    }
                    csv::DeserializeErrorKind::ParseBool ( boolerr ) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(&boolerr.to_string()));
                    }
                    csv::DeserializeErrorKind::ParseInt ( interr ) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(&interr.to_string()));
                    }
                    csv::DeserializeErrorKind::ParseFloat ( floaterr ) => {
                        curr_err = format!("{curr_err}: {}", translate_error_message(&floaterr.to_string()));
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
            csv::ErrorKind::UnequalLengths { pos, expected_len, len } => {
                res.push(format!(
                    "  Errore numero campi alla posizione: {}: lunghezza attesa {}, trovata {}",
                    parse_csv_pos(pos),
                    expected_len,
                    len
                    // no translate_error_message() anche se teoricamente lo supporta
                ));
            }
            _ => {
                res.push(format!("  Errore sconosciuto: {}", translate_error_message(&error.to_string())));
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
                if ! (ex == "csv" || ex == "CSV") {
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

pub(crate) fn check_campionamento_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>,Vec<csv::Error>>
where
    T: RecordCsvCampionamentoNISECI + 'static
{
    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>();  // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvCampionamentoNISECI>() => { b';' },
        _ => { b',' }
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_campionamento_niseci(rdr);

    println!("Campionamento NISECI: Numero record csv validi: {}", records.len());
    println!("Campionamento NISECI: Numero record csv non validi: {}", errors.len());

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

pub(crate) fn check_campionamento_niseci_path<T>(path: PathBuf) -> Result<Vec<T>,Vec<csv::Error>>
where
    T: RecordCsvCampionamentoNISECI + 'static
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(ErrorKind::Other, "Errore campionamento NISECI: il file non è un .csv"));
        let err_vec: Vec<csv::Error> = vec!(err);
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_campionamento_niseci_reader(file)
}

pub(crate) fn check_records_campionamento_niseci<T: RecordCsvCampionamentoNISECI>(records: Vec<T>, riferimento_specie: Vec<SpecieNISECI>) -> Result<Vec<RecordNISECI>,Vec<RecordCsvCampionamentoNISECIError>> {

    let (records, errors) = parse_recordcsv_campionamento_niseci(records, riferimento_specie);

    println!("Campionamento NISECI: Numero record validi: {}", records.len());
    println!("Campionamento NISECI: Numero record non validi: {}", errors.len());

    if !errors.is_empty() {
        eprintln!("Errori incontrati durante l'elaborazione dei record per campionamento NISECI: {{");
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

pub(crate) fn check_riferimento_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>,Vec<csv::Error>>
where
    T: RecordCsvRiferimentoNISECI + 'static
{

    let normalizing_reader = NormalizerReader::new(reader);

    let type_id = TypeId::of::<T>();  // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCsvRiferimentoNISECI>() => { b';' },
        _ => { b',' }
    };

    let rdr = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(normalizing_reader);
    let (records, errors) = parse_csv_riferimento_niseci(rdr);

    println!("Riferimento NISECI: Numero record csv validi: {}", records.len());
    println!("Riferimento NISECI: Numero record csv non validi: {}", errors.len());

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

pub(crate) fn check_riferimento_niseci_path<T>(path: PathBuf) -> Result<Vec<T>,Vec<csv::Error>>
where
    T: RecordCsvRiferimentoNISECI + 'static
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::new(ErrorKind::Other, "Errore riferimento NISECI: il file non è un .csv"));
        let err_vec: Vec<csv::Error> = vec!(err);
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_riferimento_niseci_reader(file)
}

pub(crate) fn check_records_riferimento_niseci<T: RecordCsvRiferimentoNISECI>(records: Vec<T>) -> Result<Vec<SpecieNISECI>,Vec<RecordCsvRiferimentoNISECIError>> {

    let (records, errors) = parse_recordcsv_riferimento_niseci(records);

    println!("Riferimento NISECI: Numero record validi: {}", records.len());
    println!("Riferimento NISECI: Numero record non validi: {}", errors.len());

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

pub(crate) fn check_campionamento_hfbi_path(_path: PathBuf) -> bool {
    todo!("Implement check campionamento HFBI");
}

fn check_soglie_cl<T: RecordCsvRiferimentoNISECI>(r: &T) -> bool {

    if r.cl_soglia1() < r.cl_soglia2() && r.cl_soglia2() < r.cl_soglia3() && r.cl_soglia3() < r.cl_soglia4() {
        return true;
    }
    false
}

fn check_soglie_ad_juv<T: RecordCsvRiferimentoNISECI>(r: &T) -> bool {
    if r.ad_juv_soglia1() < r.ad_juv_soglia2() && r.ad_juv_soglia2() < r.ad_juv_soglia3() && r.ad_juv_soglia3() < r.ad_juv_soglia4() {
        return true;
    }
    false
}
