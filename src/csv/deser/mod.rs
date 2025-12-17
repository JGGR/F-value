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
use crate::csv::TipoRecordCsv;
use serde::{de, Deserialize, Deserializer};
use std::io::{self, Read};
use std::path::Path;

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

pub fn process_csv_errors(errors: &Vec<csv::Error>, tipo_csv: TipoRecordCsv) -> Vec<String> {
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

pub fn check_path_is_file_ends_with_csv(path: &Path) -> bool {
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

pub fn translate_error_message(msg: &str) -> String {
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

pub mod hfbi;
pub mod niseci;
