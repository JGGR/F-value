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

pub mod view;
pub mod controller;
pub mod cli;

use raylib::prelude::*;
use std::fmt;
use std::path::PathBuf;
use std::io::Read;
use std::fs::File;
use crate::model::niseci::{SpecieNISECI, RecordNISECI, AnagraficaNISECI, AreaNISECI, TipoComunitaNISECI, ComunitaNISECI, IdroEcoRegioneNISECI};
use crate::model::location::Location;
use serde::Deserialize;
use serde::de::{self, Deserializer};

pub const EXIT_KEY: raylib::consts::KeyboardKey = raylib::consts::KeyboardKey::KEY_ESCAPE;
pub const PROJECT_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const PROJECT_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const PROJECT_VERSION_FULL: &'static str = env!("VERSION_STRING");
pub const SHORT_PROJECT_VERSION: &'static str = env!("SHORT_VERSION_STRING");
pub const PROJECT_BUILD_TYPE: &'static str = env!("BUILD_TYPE");
pub const PROJECT_BRANCH: &'static str = env!("BRANCH_NAME");
pub const _COMMIT_HASH: &'static str = env!("COMMIT_HASH");
pub const COMMIT_HASH_PLUS: &'static str = env!("COMMIT_HASH_PLUS");
pub const ESOX_SCREEN_WIDTH: i32 = 960;
pub const ESOX_SCREEN_HEIGHT: i32 = 540;
pub const DARK_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_dark.rgs");
pub const BLUISH_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_bluish.rgs");
pub const CANDY_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_candy.rgs");
pub const CHERRY_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_cherry.rgs");
pub const CYBER_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_cyber.rgs");
pub const JUNGLE_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_jungle.rgs");
pub const LAVANDA_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_lavanda.rgs");
pub const TERMINAL_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_terminal.rgs");
pub const ASHES_THEME_DATA: &[u8] = include_bytes!("../../assets/styles/style_ashes.rgs");
pub const CONSOLE_FONT_DATA: &[u8] = include_bytes!("../../assets/ubuntu.mono.ttf");

#[cfg(all(windows, debug_assertions))]
pub const SUPPORT_HEADLESS: bool = true;

#[cfg(all(windows, not(debug_assertions)))]
pub const SUPPORT_HEADLESS: bool = false; // This is due to windows_subsystem being "windows"

#[cfg(not(windows))]
pub const SUPPORT_HEADLESS: bool = true;

pub const RIFERIMENTO_NISECI_HEADER_FIELDS: [&str; 17] = [ "nomeComune", "nomeLatino", "codiceSpecie", "origine", "tipoAutoctono", "alloNocivita", "specieAttesa", "clSoglia1", "clSoglia2", "clSoglia3", "clSoglia4", "adJuvSoglia1", "adJuvSoglia2", "adJuvSoglia3", "adJuvSoglia4", "densSoglia1", "densSoglia2" ];
pub const RIFERIMENTO_NISECI_HEADER: &str = "\
nomeComune;nomeLatino;codiceSpecie;origine;tipoAutoctono;alloNocivita;specieAttesa;clSoglia1;clSoglia2;clSoglia3;clSoglia4;adJuvSoglia1;adJuvSoglia2;adJuvSoglia3;adJuvSoglia4;densSoglia1;densSoglia2";

pub const CAMPIONAMENTO_NISECI_HEADER_FIELDS: [&str; 7] = [ "data", "stazione", "superficie", "numPassaggio", "codiceSpecie", "lunghezza", "peso" ];
pub const CAMPIONAMENTO_NISECI_HEADER: &str = "\
data;stazione;superficie;numPassaggio;codiceSpecie;lunghezza;peso";

pub const ANAGRAFICA_NISECI_HEADER_FIELDS: [&str; 14] = [
"codiceStazione", "corpoIdrico", "regione", "provincia", "data", "lunghezzaStazione", "larghezzaStazione", "tipoComunita", "fonte", "numeroProtocollo", "codiceSpecie", "idroEcoRegione", "areaAlpina", "nomeBacino" ];
pub const ANAGRAFICA_NISECI_HEADER: &str = "\
codiceStazione;corpoIdrico;regione;provincia;data;lunghezzaStazione;larghezzaStazione;tipoComunita;fonte;numeroProtocollo;codiceSpecie;idroEcoRegione;areaAlpina;nomeBacino";

//TODO: add test to check if this string respects the discriminant ordering in GuiTheme
pub const GUI_THEME_COMBOBOX_STR: &str = "Light;Dark;Bluish;Candy;Cherry;Cyber;Jungle;Lavanda;Terminal;Ashes";

#[derive(Copy, Clone)]
pub enum CurrentView {
    HOME,
    SECOND,
    SelezioneIndice,
    SelezioneFileInput,
    ValidazioneFileInput,
    SelezioneInfoAggiuntive,
    ValidazioneInfoAggiuntive,
    ProduzioneOutput,
    ProduzionePDF,
    CONSOLE
}

impl fmt::Display for CurrentView {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match *self {
      CurrentView::HOME => "HOME",
      CurrentView::SECOND => "SECOND",
      CurrentView::SelezioneIndice => "Selezione Indice",
      CurrentView::SelezioneFileInput => "Selezione File Input",
      CurrentView::ValidazioneFileInput => "Validazione File Input",
      CurrentView::SelezioneInfoAggiuntive => "Selezione Info Aggiuntive",
      CurrentView::ValidazioneInfoAggiuntive => "Validazione Info Aggiuntive",
      CurrentView::ProduzioneOutput => "Produzione Output",
      CurrentView::ProduzionePDF => "Produzione PDF",
      CurrentView::CONSOLE => "CONSOLE",
    };
    write!(f, "{}", string_representation)
  }
}

#[derive(Copy,Clone)]
pub enum GuiTheme {
    Light,
    Dark,
    Bluish,
    Candy,
    Cherry,
    Cyber,
    Jungle,
    Lavanda,
    Terminal,
    Ashes
}

impl fmt::Display for GuiTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            GuiTheme::Light => "Light",
            GuiTheme::Dark => "Dark",
            GuiTheme::Bluish => "Bluish",
            GuiTheme::Candy => "Candy",
            GuiTheme::Cherry => "Cherry",
            GuiTheme::Cyber => "Cyber",
            GuiTheme::Jungle => "Jungle",
            GuiTheme::Lavanda => "Lavanda",
            GuiTheme::Terminal => "Terminal",
            GuiTheme::Ashes => "Ashes",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for GuiTheme {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == GuiTheme::Light as i32 => Ok(GuiTheme::Light),
            x if x == GuiTheme::Dark as i32 => Ok(GuiTheme::Dark),
            x if x == GuiTheme::Bluish as i32 => Ok(GuiTheme::Bluish),
            x if x == GuiTheme::Candy as i32 => Ok(GuiTheme::Candy),
            x if x == GuiTheme::Cherry as i32 => Ok(GuiTheme::Cherry),
            x if x == GuiTheme::Cyber as i32 => Ok(GuiTheme::Cyber),
            x if x == GuiTheme::Jungle as i32 => Ok(GuiTheme::Jungle),
            x if x == GuiTheme::Lavanda as i32 => Ok(GuiTheme::Lavanda),
            x if x == GuiTheme::Terminal as i32 => Ok(GuiTheme::Terminal),
            x if x == GuiTheme::Ashes as i32 => Ok(GuiTheme::Ashes),
            _ => Err(()),
        }
    }
}

pub struct MainState {
    pub frame_counter: u32,
    pub showing_quit_win: bool,
    pub should_quit: bool,
    pub showing_info_box: bool,
    pub showing_settings_box: bool,
    pub current_view: CurrentView,
    pub previous_view: Option<CurrentView>,
    pub theme: GuiTheme,
    pub gui_theme_combobox_active: i32,
    pub default_font_height: i32,
    pub current_font_height: i32,
    pub default_txt_spacing: i32,
    pub default_txt_color: Color,
    pub current_font: WeakFont,
    pub default_bg_color: Color,
}

impl MainState {
    pub fn new(default_font_height: i32, current_font_height: i32, default_txt_spacing: i32, current_font: WeakFont, default_txt_color: Color, default_bg_color: Color) -> Self {
        Self {
            frame_counter: 0,
            showing_quit_win: false,
            should_quit: false,
            showing_info_box: false,
            showing_settings_box: false,
            current_view: CurrentView::HOME,
            previous_view: None,
            theme: GuiTheme::Light,
            gui_theme_combobox_active: GuiTheme::Light as i32,
            default_font_height: default_font_height,
            current_font_height: current_font_height,
            default_txt_spacing: default_txt_spacing,
            default_txt_color: default_txt_color,
            current_font: current_font,
            default_bg_color: default_bg_color,
        }
    }

    pub fn set_current_view(&mut self, view: CurrentView) -> () {
        self.previous_view = Some(self.current_view);
        self.current_view = view;
    }

    pub fn get_gui_should_lock(&self) -> bool {
        return self.showing_quit_win || self.showing_info_box || self.showing_settings_box;
    }
}

pub fn propwidth(d: &RaylibDrawHandle<'_>, to_scale: i32) -> i32
{
    if to_scale < 0 || to_scale > ESOX_SCREEN_WIDTH {
        panic!("propw():  invalid to_scale value received: {to_scale}");
    }
    let current_screen_width = d.get_screen_width();
    return current_screen_width * to_scale / ESOX_SCREEN_WIDTH;
}

pub fn propheight(d: &RaylibDrawHandle<'_>, to_scale: i32) -> i32
{
    if to_scale < 0 || to_scale > ESOX_SCREEN_HEIGHT {
        panic!("proph():  invalid to_scale value received: {to_scale}");
    }
    let current_screen_height = d.get_screen_height();
    return current_screen_height * to_scale / ESOX_SCREEN_HEIGHT;
}

#[derive(Copy,Clone)]
pub enum TipoRecordCsv {
    RiferimentoNISECI,
    CampionamentoNISECI,
    AnagraficaNISECI,
    CampionamentoHFBI,
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
pub struct RecordCsvRiferimentoNISECI { //TODO: add position
    pub nome_comune: String,
    pub nome_latino: String,
    pub codice_specie: String,
    pub origine: String,
    pub tipo_autoctono: i32,
    pub allo_nocivita: i32,
    pub specie_attesa: i32,
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

impl fmt::Display for RecordCsvRiferimentoNISECI {
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
pub struct RecordCsvCampionamentoNISECI { //TODO: add position
    //id: i32,
    pub data: String,
    pub stazione: String,
    pub superficie: i32,
    pub num_passaggio: String,
    pub codice_specie: String,
    pub lunghezza: i32,
    pub peso: i32,
}

impl fmt::Display for RecordCsvCampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCsvCampionamentoNISECI: {{ data: [{}], stazione: [{}], superficie: [{}], num_passaggio: [{}], codice_specie: [{}], lunghezza: [{}], peso: [{}] }}",
              // id: [{}], before the }}
              //self.id,
              self.data, self.stazione, self.superficie,
              self.num_passaggio, self.codice_specie, self.lunghezza, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_campionamento_niseci<R>(mut rdr: csv::Reader<R>) -> (Vec<RecordCsvCampionamentoNISECI>, Vec<csv::Error>) where R: std::io::Read {
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

pub enum RecordCsvCampionamentoNISECIError {
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

pub fn parse_recordcsv_campionamento_niseci(records: Vec<RecordCsvCampionamentoNISECI>, riferimento_specie: Vec<SpecieNISECI>) -> (Vec<RecordNISECI>,Vec<RecordCsvCampionamentoNISECIError>) {
    //TODO: update when the model includes the missing fields
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        if r.codice_specie.len() < 1 {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (lunghezza < 1)") };
            errors.push(err);
            continue;
        }
        let codice_specie = r.codice_specie;
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


        let passaggio_cattura;
        //TODO: update this abomination when records change to have an integer directly
        match r.num_passaggio.as_str() {
            "c1" => {
                passaggio_cattura = 1;
            }
            "c2" => {
                passaggio_cattura = 2;
            }
            "c3" => {
                passaggio_cattura = 3;
            }
            "c4" => {
                passaggio_cattura = 4;
            }
            _ => {
                let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: num_passaggio non valido (non in [\"c1\", \"c2\" ... \"c4\"]): {}", r.num_passaggio) };
                errors.push(err);
                continue;
            }
        }

        if r.lunghezza < 0 {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: lunghezza < 0") };
            errors.push(err);
            continue;
        }

        if r.peso < 0 {
            let err = RecordCsvCampionamentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: peso < 0") };
            errors.push(err);
            continue;
        }

        let niseci_rec = RecordNISECI {
            specie: matched_specie.clone(),
            passaggio_cattura: passaggio_cattura as u8,
            lunghezza: r.lunghezza as u32,
            peso: r.peso as u32
        };
        campioni.push(niseci_rec);
    }
    (campioni, errors)
}

pub fn parse_csv_riferimento_niseci<R>(mut rdr: csv::Reader<R>) -> (Vec<RecordCsvRiferimentoNISECI>, Vec<csv::Error>) where R: std::io::Read {
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

pub enum RecordCsvRiferimentoNISECIError {
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

pub fn parse_recordcsv_riferimento_niseci(records: Vec<RecordCsvRiferimentoNISECI>) -> (Vec<SpecieNISECI>,Vec<RecordCsvRiferimentoNISECIError>) {
    //TODO: update when the model includes the missing fields
    let mut specie = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    let mut used_id_specie = Vec::new(); // Stores already-parsed ids to detect doubles
    for r in records {
        idx += 1;
        let mut origine_autoctono = true;
        match r.origine.as_str() {
            "ALL" => {
                origine_autoctono = false;
            },
            "AUT" => {},
            _ => {
                let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: origine invalida (non \"AUT\" o \"ALL\"): {}", r.origine) };
                errors.push(err);
                continue;
            }
        }
        if r.specie_attesa < 0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: specie_attesa < 0") };
            errors.push(err);
            continue;
        }
        let specie_attesa = r.specie_attesa > 0; // TODO: possiamo prendere qualsiasi non-zero come
                                                 // "atteso"?
        if r.tipo_autoctono < 0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: tipo_autoctono < 0") };
            errors.push(err);
            continue;
        }

        let tipo_autoctono: u8;
        let tipo_alloctono: u8;
        if origine_autoctono {
            match r.tipo_autoctono {
                1 | 2 => {
                    tipo_autoctono = r.tipo_autoctono as u8;
                }
                _ => {
                    let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: tipo_autoctono non valido (non 1 o 2): {}", r.tipo_autoctono) };
                    errors.push(err);
                    continue;
                }
            }
            tipo_alloctono = 0;
        } else {
            tipo_autoctono = 0;
            match r.allo_nocivita {
                0 | 1 | 2 | 3 => {
                    tipo_alloctono = r.allo_nocivita as u8;
                }
                _ => {
                    let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: allo_nocivita non valido (non [0..3]): {}", r.allo_nocivita) };
                    errors.push(err);
                    continue;
                }
            }
        }

        if r.codice_specie.len() < 1 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (lunghezza < 1)") };
            errors.push(err);
            continue;
        }

        let id = r.codice_specie.clone();

        if used_id_specie.contains(&id) {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: codice_specie non valido (ridefinizione)") };
            errors.push(err);
            continue;
        }

        let nome =  r.nome_latino.clone(); //TODO: controllare se dovrebbe essere nome_comune

        //TODO: update when SpecieNISECI has the missing fields

        // Check dens_soglia
        if r.dens_soglia1 < 0.0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia1 non valido (< 0)") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1 == 0.0 && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia1 non valido (== 0) per una specie attesa") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2 < 0.0 {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia2 non valido (< 0)") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2 == 0.0 && specie_attesa {
            let err = RecordCsvRiferimentoNISECIError::ValoreInvalido { msg : format!("Record {idx}: dens_soglia2 non valido (== 0) per una specie attesa") };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1 >= r.dens_soglia2 && specie_attesa {
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
            nome: nome,
            tipo_autoctono: tipo_autoctono,
            tipo_alloctono: tipo_alloctono,
            specie_attesa: specie_attesa,
            cl_soglia1: r.cl_soglia1, // in cm
            cl_soglia2: r.cl_soglia2, // in cm
            cl_soglia3: r.cl_soglia3, // in cm
            cl_soglia4: r.cl_soglia4, // in cm
            ad_juv_soglia1: r.ad_juv_soglia1,
            ad_juv_soglia2: r.ad_juv_soglia2,
            ad_juv_soglia3: r.ad_juv_soglia3,
            ad_juv_soglia4: r.ad_juv_soglia4,
            dens_soglia1: r.dens_soglia1,
            dens_soglia2: r.dens_soglia2,
        };
        specie.push(specie_rec);
        used_id_specie.push(id);
    }

    (specie, errors)
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordCsvAnagraficaNISECI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    pub lunghezza_stazione: u32,
    pub larghezza_stazione: u32,
    pub tipo_comunita: u32,
    pub fonte: String,
    pub numero_protocollo: String,
    pub codice_specie: String,
    pub idro_eco_regione: u32,
    pub area_alpina: u32,
    pub nome_bacino: String,
}

impl fmt::Display for RecordCsvAnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaNISECI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], tipo_comunita [{}], fonte [{}],\
            numero_protocollo: [{}], codice_specie: [{}], idro_eco_regione: [{}],\
            area_alpina: [{}], nome_bacino: [{}]}}",
            self.codice_stazione, self.corpo_idrico, self.regione, self.provincia,
            self.data, self.lunghezza_stazione, self.larghezza_stazione,
            self.tipo_comunita, self.fonte, self.numero_protocollo, self.codice_specie,
            self.idro_eco_regione, self.area_alpina, self.nome_bacino
        );
        write!(f, "{}", string_representation)
    }
}

pub enum RecordCsvAnagraficaNISECIError {
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

pub fn parse_csv_anagrafica_niseci<R>(mut rdr: csv::Reader<R>) -> (Vec<RecordCsvAnagraficaNISECI>, Vec<csv::Error>) where R: std::io::Read {
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

pub fn parse_recordcsv_anagrafica_niseci(records: Vec<RecordCsvAnagraficaNISECI>) -> Result<AnagraficaNISECI, Vec<RecordCsvAnagraficaNISECIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Troppi record: {}, atteso 1", records.len()) };
        errors.push(err);
    }
    if records.len() < 1 {
        let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Nessun record trovato: atteso 1") };
        errors.push(err);
        return Err(errors);
    }

    let r = records.get(1).unwrap();
    let mut area = AreaNISECI::Mediterranea;
    if r.area_alpina > 0 {
        area = AreaNISECI::Alpina;
    }
    let mut tipo_comunita = TipoComunitaNISECI::Redatta;
    match r.tipo_comunita {
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
            let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("Tipo comunita NISECI non valido: {}, atteso [0, 3]", r.tipo_comunita) };
            errors.push(err);
        }
    }

    let mut idro_eco_regione = IdroEcoRegioneNISECI::Toscana;
    idro_eco_regione = match r.idro_eco_regione {
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
            let err = RecordCsvAnagraficaNISECIError::ValoreInvalido { msg : format!("IdroEcoRegioneNISECI non valido: {}, atteso [0, 20]", r.idro_eco_regione) };
            errors.push(err);
            IdroEcoRegioneNISECI::Toscana // To still assign something by default
        }
    };

    if errors.len() > 0 {
        return Err(errors);
    }

    let res = AnagraficaNISECI {
        comunita: ComunitaNISECI {
            tipo: tipo_comunita,
            fonte: Some(r.fonte.clone()),
            numero_protocollo: Some(r.numero_protocollo.clone()),
        },
        codice_stazione: r.codice_stazione.clone(),
        date_string: r.data.clone(), // Formato gg/mm/aaaa
        area: area,
        corpo_idrico: r.corpo_idrico.clone(),
        bacino_appartenenza: r.nome_bacino.clone(),
        idro_eco_regione: idro_eco_regione,
        posizione: Location {
            regione: r.regione.clone(),
            provincia: r.provincia.clone()
        },
        lunghezza_media_stazione: r.lunghezza_stazione as f32,
        larghezza_media_stazione: r.larghezza_stazione as f32,
    };
    return Ok(res);

}

pub fn check_anagrafica_niseci_reader<R: Read>(reader: R) -> Result<Vec<RecordCsvAnagraficaNISECI>,Vec<csv::Error>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(reader);
    let (records, errors) = parse_csv_anagrafica_niseci(rdr);

    println!("Anagrafica NISECI: Numero record csv validi: {}", records.len());
    println!("Anagrafica NISECI: Numero record csv non validi: {}", errors.len());

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
        return Err(errors);
    } else {
        println!("Tutti i record csv dell'anagrafica NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        return Ok(records);
    }
}

pub fn check_records_anagrafica_niseci(records: Vec<RecordCsvAnagraficaNISECI>) -> Result<AnagraficaNISECI,Vec<RecordCsvAnagraficaNISECIError>> {

    let res = parse_recordcsv_anagrafica_niseci(records);

    match res {
        Ok(anagrafica) => {
            println!("Anagrafica NISECI: {}", anagrafica);
            println!("Tutti i record dell'anagrafica NISECI sono stati processati con successo!");
            /*
            for record in &records {
                println!("  Record: {{{record}}}");
            }
            */
            return Ok(anagrafica);
        }
        Err(errors) => {
            println!("Anagrafica NISECI: Numero record non validi: {}", errors.len());
            eprintln!("Errori incontrati durante l'elaborazione dei record per anagrafica NISECI: {{");
            //TODO: add process_record_anagraficaNISECI_errors()
            for error in &errors {
                eprintln!("  {}", error);
            }
            eprintln!("}}");
            return Err(errors);
        }
    }
}

pub fn check_anagrafica_niseci_path(path: PathBuf) -> Result<Vec<RecordCsvAnagraficaNISECI>,Vec<csv::Error>> {
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        return Err(Vec::new());
    }
    let file = File::open(path).expect("Unable to open file");
    return check_anagrafica_niseci_reader(file);
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
        msg.replace("invalid digit found in string", "tipo non valido: numero, attesa stringa")
            .replace("field", "campo")
    } else if msg.contains("invalid float literal") {
        msg.replace("invalid float literal", "tipo non valido: atteso razionale").replace("field", "campo")
    } else if msg.contains("cannot parse") && msg.contains("from empty string") {
        // NOTE: there's a leading space in " from empty string", it enables us to attach the ","
        // to the previous part
        msg.replace("cannot parse", "campo vuoto: atteso")
            .replace("field","campo")
            .replace("float","razionale")
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
    return res;
}

pub fn process_csv_errors(errors: &Vec<csv::Error>, tipo_csv: TipoRecordCsv) -> Vec<String> {
    let mut res = Vec::new();
    for error in errors {
        match error.kind() {
            csv::ErrorKind::Deserialize { pos, err } => {
                let mut field_str;
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
                            TipoRecordCsv::CampionamentoHFBI => {
                                // TODO: implement this for HFBI
                                todo!("Implement this for HFBI");
                                field_str = field_idx.to_string();
                            }
                        }
                    }
                    None => {
                        field_str = "none".to_string();
                    }
                }
                let mut curr_err = format!(
                    "  Errore di deserializzazione alla posizione: {}: campo {}",
                    parse_csv_pos(&pos),
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
                    parse_csv_pos(&pos),
                    translate_error_message(&err.to_string())
                ));
            }
            csv::ErrorKind::UnequalLengths { pos, expected_len, len } => {
                res.push(format!(
                    "  Errore numero campi alla posizione: {}: lunghezza attesa {}, trovata {}",
                    parse_csv_pos(&pos),
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
    return res;
}

fn check_path_is_file_ends_with_csv(path: &PathBuf) -> bool {
    if !path.exists() {
        eprintln!("Error: Passed path does not exist");
        return false;
    } else if !path.is_file() {
        eprintln!("Error: Passed path is not a regular file");
        return false;
    } else {
        let ext = path.extension();
        match ext {
            Some(ex) => {
                if ! (ex == "csv") {
                    eprintln!("Error: Passed path does not end with .csv");
                    return false;
                }
                return true;
            }
            None => {
                eprintln!("Error: Passed path does not end with .csv");
                return false;
            }
        }
    }
}

pub fn check_campionamento_niseci_reader<R: Read>(reader: R) -> Result<Vec<RecordCsvCampionamentoNISECI>,Vec<csv::Error>> {
    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(reader);
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
        return Err(errors);
    } else {
        println!("Tutti i record csv del campionamento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        return Ok(records);
    }
}

pub fn check_campionamento_niseci_path(path: PathBuf) -> Result<Vec<RecordCsvCampionamentoNISECI>,Vec<csv::Error>> {
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        return Err(Vec::new());
    }
    let file = File::open(path).expect("Unable to open file");
    return check_campionamento_niseci_reader(file);
}

pub fn check_records_campionamento_niseci(records: Vec<RecordCsvCampionamentoNISECI>, riferimento_specie: Vec<SpecieNISECI>) -> Result<Vec<RecordNISECI>,Vec<RecordCsvCampionamentoNISECIError>> {

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
        return Err(errors);
    } else {
        println!("Tutti i record del campionamento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        return Ok(records);
    }
}

pub fn check_riferimento_niseci_reader<R: Read>(reader: R) -> Result<Vec<RecordCsvRiferimentoNISECI>,Vec<csv::Error>> {

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(reader);
    let (records, errors) = parse_csv_riferimento_niseci(rdr);

    println!("Riferimento NISECI: Numero record csv validi: {}", records.len());
    println!("Riferimento NISECI: Numero record csv non validi: {}", errors.len());

    if !errors.is_empty() {
        eprintln!("Errori incontrati durante l'elaborazione csv del riferimento NISECI: {{");
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        */
        let processed_errors = process_csv_errors(&errors, TipoRecordCsv::RiferimentoNISECI);
        eprintln!("Errori incontrati durante l'elaborazione csv del campionamento NISECI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        return Err(errors);
    } else {
        println!("Tutti i record csv del riferimento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        return Ok(records);
    }
}

pub fn check_riferimento_niseci_path(path: PathBuf) -> Result<Vec<RecordCsvRiferimentoNISECI>,Vec<csv::Error>> {
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        return Err(Vec::new());
    }
    let file = File::open(path).expect("Unable to open file");
    return check_riferimento_niseci_reader(file);
}

pub fn check_records_riferimento_niseci(records: Vec<RecordCsvRiferimentoNISECI>) -> Result<Vec<SpecieNISECI>,Vec<RecordCsvRiferimentoNISECIError>> {

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
        return Err(errors);
    } else {
        println!("Tutti i record del riferimento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        return Ok(records);
    }
}

pub fn check_campionamento_hfbi_path(_path: PathBuf) -> bool {
    todo!("Implement check campionamento HFBI");
}

fn check_soglie_cl(r: &RecordCsvRiferimentoNISECI) -> bool {
    if r.cl_soglia1 < r.cl_soglia2 && r.cl_soglia2 < r.cl_soglia3 && r.cl_soglia3 < r.cl_soglia4 {
        return true;
    }
    false
}

fn check_soglie_ad_juv(r: &RecordCsvRiferimentoNISECI) -> bool {
    if r.ad_juv_soglia1 < r.ad_juv_soglia2 && r.ad_juv_soglia2 < r.ad_juv_soglia3 && r.ad_juv_soglia3 < r.ad_juv_soglia4 {
        return true;
    }
    false
}
