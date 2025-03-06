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

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::vec::Vec;
use std::fmt;

use super::location::Location;

#[cfg(test)]
use crate::engines::niseci::linear_regression::Point; // Needed by fishes_for_every_passage() only
                                                      // in test builds

#[derive(Debug, Clone)]
pub struct SpecieNISECI {
  pub id: String,
  pub nome: String,
  pub tipo_autoctono: u8,
  pub tipo_alloctono: u8,
  pub specie_attesa: bool,
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

impl fmt::Display for SpecieNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let specie_attesa_str = match self.specie_attesa {
            true => format!("SI"),
            false => format!("NO"),
        };
        let string_representation = format!("SpecieNISECI: {{ id: {{{}}}, nome {{{}}}, tipo_autoctono: {{{}}}, tipo_alloctono: {{{}}}, specie_attesa: {{{}}}",
                self.id, self.nome, self.tipo_autoctono, self.tipo_alloctono, specie_attesa_str);
        write!(f, "{}", string_representation)
    }
}

impl SpecieNISECI {
  pub fn new_dummy_specie() -> SpecieNISECI {
    SpecieNISECI {
      id: "0".to_string(),
      nome: "dummy".to_string(),
      tipo_autoctono: 0,
      tipo_alloctono: 0,
      specie_attesa: true,
      cl_soglia1: 1, // in mm
      cl_soglia2: 2, // in mm
      cl_soglia3: 3, // in mm
      cl_soglia4: 4,
      ad_juv_soglia1: 0.1,
      ad_juv_soglia2: 0.2,
      ad_juv_soglia3: 0.3,
      ad_juv_soglia4: 0.4,
      dens_soglia1: 0.5,
      dens_soglia2: 0.9,
    }
  }
}

#[derive(Clone)]
pub struct RiferimentoNISECI {
  pub elenco_specie: Vec<SpecieNISECI>
}

impl fmt::Display for RiferimentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = format!("RiferimentoNISECI: {{");
        for s in &self.elenco_specie {
            string_representation = format!("{string_representation}\n  {{{s}}},");
        }
        string_representation = format!("{string_representation}\n}}");
        write!(f, "{}", string_representation)
    }
}

impl RiferimentoNISECI {
    pub fn new(elenco_specie: Vec<SpecieNISECI>) -> Self {
        Self {
            elenco_specie: elenco_specie
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordNISECI {
  pub specie: SpecieNISECI,
  pub passaggio_cattura: u8,
  pub lunghezza: u32, /// in millimetri
  pub peso: u32 // in grammi
}

impl fmt::Display for RecordNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!("RecordNISECI: {{ specie: {{{}}}, passaggio_cattura {{{}}}, lunghezza: {{{}}}, peso: {{{}}}",
                self.specie, self.passaggio_cattura, self.lunghezza, self.peso);
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone)]
pub struct CampionamentoNISECI {
  pub campionamento: Vec<RecordNISECI>
}

impl fmt::Display for CampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = format!("CampionaNISECI: {{");
        for r in &self.campionamento {
            string_representation = format!("{string_representation}\n  {{{r}}},");
        }
        string_representation = format!("{string_representation}\n}}");
        write!(f, "{}", string_representation)
    }
}

impl CampionamentoNISECI {

    #[cfg(test)]
    pub fn fishes_for_every_passage(&self) -> Vec<Point<i32>> {
    let mut max_pass = 0;
    for record in self.campionamento.iter() {
        if record.passaggio_cattura > max_pass {
        max_pass = record.passaggio_cattura;
        }
    }

    let mut passaggi: Vec<i32> = vec![0; max_pass as usize];
    for record in self.campionamento.iter() {
        passaggi[(record.passaggio_cattura - 1) as usize] += 1;
    }

    let mut tot = 0;

    // x = pesci totali fino a quel passaggio y = pesci del passaggio
    let mut pass_sum: Vec<Point<i32>> = Vec::with_capacity(max_pass as usize);
    for pass in passaggi.iter() {
        tot += pass;
        pass_sum.push(Point::new(tot, *pass));
    }

        pass_sum
    }

    pub fn new(campionamento: Vec<RecordNISECI>) -> Self {
        Self {
            campionamento: campionamento,
        }
    }

    pub fn get_numero_pesci_alieni_e_indigeni(&self) -> AlieniIndigeni {
      let mut alieni_indigeni = AlieniIndigeni {
        alieni: 0,
        indigeni: 0
      };

      for pesce in &self.campionamento {
        if pesce.specie.tipo_alloctono > 0 && pesce.specie.tipo_alloctono <= 3 {
          alieni_indigeni.alieni += 1;
        } else if pesce.specie.tipo_autoctono == 1 || pesce.specie.tipo_autoctono == 2  {
          alieni_indigeni.indigeni += 1;
        }
      }

      return alieni_indigeni
    }

    pub fn get_tot_specie_autoctone(&self) -> usize {
      let mut map: HashMap<String, bool> = HashMap::new();

      for cattura in &self.campionamento {
        if cattura.specie.tipo_autoctono == 1 || cattura.specie.tipo_autoctono == 1 {
          match map.entry(cattura.specie.id.clone()) {
            Entry::Occupied(_) => {},
            Entry::Vacant(entry) => {
              entry.insert(true);
            }
          };
        }
      }

      map.len()
    }
}

pub struct AlieniIndigeni {
  pub alieni: u32,
  pub indigeni: u32
}

#[derive(Clone)]
pub enum TipoComunitaNISECI {
  Redatta,
  Recuperata,
  Dm260_2010,
  AffinataDalMase
}

impl fmt::Display for TipoComunitaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            TipoComunitaNISECI::Redatta => "Redatta dall'operatore",
            TipoComunitaNISECI::Recuperata => "Recuperata da fonti bibliografiche",
            TipoComunitaNISECI::Dm260_2010 => "DM 260/2010",
            TipoComunitaNISECI::AffinataDalMase => "Affinata dal Mase",
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone)]
pub struct ComunitaNISECI {
  pub tipo: TipoComunitaNISECI,
  pub fonte: Option<String>,
  pub numero_protocollo: Option<String>
}

impl fmt::Display for ComunitaNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match self.tipo {
        TipoComunitaNISECI::Redatta | TipoComunitaNISECI::Dm260_2010 => {
            format!("ComunitaNISECI: {{ tipo: {{{}}}", self.tipo)
        }
        TipoComunitaNISECI::Recuperata => {
            if let Some(fonte) = &self.fonte { //TODO: is this good?
                format!("ComunitaNISECI: {{ tipo: {{{}}}, fonte: {{{}}} ", self.tipo, fonte)
            } else {
                format!("ComunitaNISECI: {{ tipo: {{{}}}, fonte: MANCANTE", self.tipo)
            }
        }
        TipoComunitaNISECI::AffinataDalMase => {
            if let Some(num_proto) = &self.numero_protocollo { //TODO: is this good?
                format!("ComunitaNISECI: {{ tipo: {{{}}}, numero_protocollo: {{{}}} ", self.tipo, num_proto)
            } else {
                format!("ComunitaNISECI: {{ tipo: {{{}}}, numero_protocollo: MANCANTE", self.tipo)
            }
        }
    };
    write!(f, "{}", string_representation)
  }
}

#[derive(Clone)]
pub enum AreaNISECI {
    Alpina,
    Mediterranea
}

impl fmt::Display for AreaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            AreaNISECI::Alpina => format!("AreaNISECI: {{ Alpina }}"),
            AreaNISECI::Mediterranea => format!("AreaNISECI: {{ Mediterranea }}"),
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone)]
pub struct AnagraficaNISECI {
  pub comunita: ComunitaNISECI,
  pub codice_stazione: String,
  pub date_string: String, // Formato gg/mm/aaaa
  pub area: AreaNISECI,
  pub corpo_idrico: String,
  pub bacino_appartenenza: String,
  pub idro_eco_regione: IdroEcoRegioneNISECI,
  pub posizione: Location,
  pub lunghezza_media_stazione: f32,
  pub larghezza_media_stazione: f32
}

impl AnagraficaNISECI {
  pub fn get_lunghezza_media(&self) -> f32 {
    self.lunghezza_media_stazione
  }
  pub fn get_larghezza_media(&self) -> f32 {
    self.larghezza_media_stazione
  }
}

impl fmt::Display for AnagraficaNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = format!("AnagraficaNISECI: {{ comunita: {{{}}}, codice_stazione {{{}}}, area: {{{}}}, corpo_idrico: {{{}}}, bacino_appartenenza: {{{}}}, idro_eco_regione: {{{}}}, posizione: {{{}}}, lunghezza_stazione: {{{}}}, larghezza_stazione: {{{}}} }}",
        self.comunita, self.codice_stazione, self.area, self.corpo_idrico, self.bacino_appartenenza, self.idro_eco_regione, self.posizione, self.lunghezza_media_stazione, self.larghezza_media_stazione);
    write!(f, "{}", string_representation)
  }
}

#[derive(Clone)]
pub enum IdroEcoRegioneNISECI {
  AlpiCentroOrientali,
  AlpiMediterranee,
  AlpiMeridionali,
  AlpiOccidentali,
  AppenninoCentrale,
  AppenninoMeridionale,
  AppenninoPiemontese,
  AppenninoSettentrionale,
  BasilicataTavoliere,
  BassoLazio,
  CalabriaNebrodi,
  Carso,
  CostaAdriatica,
  Monferrato,
  PianuraPadana,
  PrealpiDolomiti,
  PugliaGargano,
  RomaViterbeseVesuvio,
  Sardegna,
  Sicilia,
  Toscana,
}

impl fmt::Display for IdroEcoRegioneNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = match *self {
      IdroEcoRegioneNISECI::AlpiCentroOrientali => "Alpi Centro-orientali",
      IdroEcoRegioneNISECI::AlpiMediterranee => "Alpi Mediterranee",
      IdroEcoRegioneNISECI::AlpiMeridionali => "Alpi Meridionali",
      IdroEcoRegioneNISECI::AlpiOccidentali => "Alpi Occidentali",
      IdroEcoRegioneNISECI::AppenninoCentrale => "Appennino Centrale",
      IdroEcoRegioneNISECI::AppenninoMeridionale => "Appennino Meridionale",
      IdroEcoRegioneNISECI::AppenninoPiemontese => "Appennino Piemontese",
      IdroEcoRegioneNISECI::AppenninoSettentrionale => "Appennino Settentrionale",
      IdroEcoRegioneNISECI::BasilicataTavoliere => "Basilicata Tavoliere",
      IdroEcoRegioneNISECI::BassoLazio => "Basso Lazio",
      IdroEcoRegioneNISECI::CalabriaNebrodi => "Calabria Nebrodi",
      IdroEcoRegioneNISECI::Carso => "Carso",
      IdroEcoRegioneNISECI::CostaAdriatica => "Costa Adriatica",
      IdroEcoRegioneNISECI::Monferrato => "Monferrato",
      IdroEcoRegioneNISECI::PianuraPadana => "Pianura Padana",
      IdroEcoRegioneNISECI::PrealpiDolomiti => "Prealpi Dolomiti",
      IdroEcoRegioneNISECI::PugliaGargano => "Puglia Gargano",
      IdroEcoRegioneNISECI::RomaViterbeseVesuvio => "Roma-Viterbese-Vesuvio",
      IdroEcoRegioneNISECI::Sardegna => "Sardegna",
      IdroEcoRegioneNISECI::Sicilia => "Sicilia",
      IdroEcoRegioneNISECI::Toscana => "Toscana",
    };
    write!(f, "{}", string_representation)
  }
}

#[derive(Clone)]
pub struct ValoriIntermediSpecieNISECI {
    pub densita_stimata: f32,
    pub classi_eta: ClassiEtaSpecieNISECI,
    pub rapporto_ad_juv: Option<f32>,
    pub x2_a_a: u8,
    pub x2_a_b: u8,
}

impl fmt::Display for ValoriIntermediSpecieNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let rapporto_ad_juv_str = match self.rapporto_ad_juv {
        Some(v) => {
            format!("{v}")
        }
        None => {
            format!("NC")
        }
    };
    let string_representation = format!("ValoriIntermediSpecieNISECI: {{ densita stimata: {{{}}}, classi eta: {{{}}}, rapport ad/juv: {{{}}}, x2a_a: {{{}}}, x2a_b: {{{}}} }}",
        self.densita_stimata, self.classi_eta, rapporto_ad_juv_str, self.x2_a_a, self.x2_a_b);
    write!(f, "{}", string_representation)
  }
}

#[derive(Clone)]
pub struct ValoriIntermediNISECI {
    pub specie_specifici: HashMap<String, ValoriIntermediSpecieNISECI>,
    pub x2_a: f32,
    pub x2_b: f32,
    pub x3_a: Option<f32>,
    pub x3_b: Option<f32>,
}

impl fmt::Display for ValoriIntermediNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let x3_a_str = match self.x3_a {
        Some(v) => format!("{v}"),
        None => format!("NC")
    };
    let x3_b_str = match self.x3_b {
        Some(v) => format!("{v}"),
        None => format!("NC")
    };
    let mut string_representation = format!("ValoriIntermediNISECI: {{\n    x2_a: {{{}}}, x2_b: {{{}}}, x3_a: {{{}}}, x3_b: {{{}}}\n    specie specifici:",
        self.x2_a, self.x2_b, x3_a_str, x3_b_str);

    for (k, v) in self.specie_specifici.iter() {
        string_representation = format!("{}\n        specie: {{{}}}, valori: {{{}}}", string_representation, k, v);
    }
    string_representation = format!("{}\n}}", string_representation);
    write!(f, "{}", string_representation)
  }
}

impl ValoriIntermediNISECI {
    pub fn log(&self) {
        //TODO: a proper format? we count on the embedded newlines to leverage the
        //chopping on newlines from add_console_message()
        println!("Valori intermedi: {{{self}}}");
    }
}

#[derive(Clone)]
pub struct RisultatoNISECI {
  valore: f32,
  rqe: f32,
  valori_intermedi: ValoriIntermediNISECI,
}

impl fmt::Display for RisultatoNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = format!("RisultatoNISECI: {{ valore NISECI: {{{}}}, valore RQE NISECI: {{{}}}, valori intermedi: {{{}}} }}", self.valore, self.rqe, self.valori_intermedi);
    write!(f, "{}", string_representation)
  }
}

impl RisultatoNISECI {
    pub fn new(valore: f32, rqe: f32, valori_intermedi: ValoriIntermediNISECI) -> Self {
        Self {
            valore: valore,
            rqe: rqe,
            valori_intermedi: valori_intermedi,
        }
    }
    pub fn get_valore(&self) -> f32 {
        return self.valore;
    }
    pub fn get_rqe(&self) -> f32 {
        return self.rqe;
    }
}

#[derive(Clone, Copy)]
pub struct MetricheX2aB {
    criterio_b: u8,
    rapporto_ad_juv: Option<f32>
}

impl MetricheX2aB {
    pub fn new(criterio_b: u8, rapporto_ad_juv: Option<f32>) -> Self {
        Self {
            criterio_b: criterio_b,
            rapporto_ad_juv: rapporto_ad_juv,
        }
    }
    pub fn get_criterio_b(&self) -> u8 {
        return self.criterio_b;
    }
    pub fn get_rapporto_ad_juv(&self) -> Option<f32> {
        return self.rapporto_ad_juv;
    }
}

#[derive(Clone, Copy)]
pub struct MetricheX2A {
    criterio_a: u8,
    criteri_x2a_b: MetricheX2aB
}

impl MetricheX2A {
    pub fn new(criterio_a: u8, criteri_x2a_b: MetricheX2aB) -> Self {
        Self {
            criterio_a: criterio_a,
            criteri_x2a_b: criteri_x2a_b
        }
    }
    pub fn get_criterio_a(&self) -> u8 {
        return self.criterio_a;
    }
    pub fn get_criterio_b(&self) -> u8 {
        return self.criteri_x2a_b.get_criterio_b();
    }
    pub fn get_rapporto_ad_juv(&self) -> Option<f32> {
        return self.criteri_x2a_b.get_rapporto_ad_juv();
    }
}

/// le classi eta contengono il numero di esemplari trovati
/// nel campionamento per ogni specie catturata
/// suddivisi nelle loro classi di eta (in base alla lunghezza)
#[derive(Clone)]
pub struct ClassiEtaSpecieNISECI {
  pub specie: SpecieNISECI,
  pub cl1: u32,
  pub cl2: u32,
  pub cl3: u32,
  pub cl4: u32,
  pub cl5: u32,
}

impl fmt::Display for ClassiEtaSpecieNISECI {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string_representation = format!("ClassiEtaSpecieNISECI: {{ specie: {{{}}}, cl1: {{{}}}, cl2: {{{}}}, cl3: {{{}}}, cl4: {{{}}}, cl5: {{{}}} }}",
        self.specie, self.cl1, self.cl2, self.cl3, self.cl4, self.cl5);
    write!(f, "{}", string_representation)
  }
}

impl ClassiEtaSpecieNISECI {
  pub fn new() -> ClassiEtaSpecieNISECI {
    ClassiEtaSpecieNISECI {
      specie: SpecieNISECI::new_dummy_specie(),
      cl1: 0,
      cl2: 0,
      cl3: 0,
      cl4: 0,
      cl5: 0,
    }

  }

  pub fn new_cl_prevalorizzata(record: &RecordNISECI) -> ClassiEtaSpecieNISECI {
    let mut classe = ClassiEtaSpecieNISECI::new();
    classe.update_classi_eta(record);
    classe.specie = record.specie.clone();
    classe
  }

  pub fn update_classi_eta(&mut self, record: &RecordNISECI) -> () {
    match ClassiEta::find_classe_eta(record) {
        ClassiEta::CL1 => self.cl1 += 1,
        ClassiEta::CL2 => self.cl2 += 1,
        ClassiEta::CL3 => self.cl3 += 1,
        ClassiEta::CL4 => self.cl4 += 1,
        ClassiEta::CL5 => self.cl5 += 1,
    }
  }

  fn get_how_many_classes(&self) -> usize {
    return [self.cl1, self.cl2, self.cl3, self.cl4, self.cl5]
      .into_iter()
      .filter(|&value| value > 0)
      .count();
  }

  pub fn get_x2_a_criterio_a(&self) -> u8 {
    let count = self.get_how_many_classes();
    if count >= 4 {
      return 1;
    }
    if count == 3 {
      return 2;
    }
    return 3;
  }

  pub fn get_x2_a_criterio_b(&self) -> (u8, Option<f32>) {
    if (self.cl2 + self.cl3) == 0 {
      return (3, None);
    }

    let ad_juv = (self.cl4 + self.cl5) as f32 / (self.cl2 + self.cl3) as f32;
    if ad_juv < self.specie.ad_juv_soglia1 {
      return (3, Some(ad_juv));
    }
    if ad_juv <= self.specie.ad_juv_soglia2 {
      return (2, Some(ad_juv));
    }
    if ad_juv <= self.specie.ad_juv_soglia3 {
      return (1, Some(ad_juv));
    }
    if ad_juv <= self.specie.ad_juv_soglia4 {
      return (2, Some(ad_juv));
    }
    return (3, Some(ad_juv));
  }

  /// questa fn viene usata sia per x2_a che per x3
  /// i suoi test sono esattamente quelli per calculate_x2_a
  pub(crate) fn calculate_struttura_popolazione(&self) -> Result<(f32, MetricheX2A), String> {
    let criterio_a: u8 = self.get_x2_a_criterio_a();
    let (criterio_b, ad_juv): (u8, Option<f32>) = self.get_x2_a_criterio_b();

    if criterio_a == 1 && criterio_b == 3 {
      return Ok((0.5, MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv))));
    }
    if criterio_a == 1 {
      return Ok((1.0, MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv))));
    }
    if criterio_a == 2 && criterio_b == 3 {
      return Ok((0.0, MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv))));
    }
    if criterio_a == 2 {
      return Ok((0.5, MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv))));
    }
    if criterio_a == 3 {
      return Ok((0.0, MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv))));
    }
    return Err(format!("Il Criterio A o B di x2a è diverso da 1 o 2 o 3. criterio A = {}, criterio B = {}", criterio_a, criterio_b));
  }

}


/// enum che aiuta a valorizzare ClassiEtaSpecieNISECI
/// (vedi update_classi_eta)
pub enum ClassiEta {
  CL1,
  CL2,
  CL3,
  CL4,
  CL5,
}

impl ClassiEta {
  pub fn find_classe_eta(record: &RecordNISECI) -> ClassiEta {
    if record.lunghezza < record.specie.cl_soglia1 {
      return ClassiEta::CL1;
    } else if record.lunghezza < record.specie.cl_soglia2 {
      return ClassiEta::CL2;
    } else if record.lunghezza < record.specie.cl_soglia3 {
      return ClassiEta::CL3;
    } else if record.lunghezza < record.specie.cl_soglia4 {
      return ClassiEta::CL4;
    } else {
      return ClassiEta::CL5;
    }
  }
}


/// struct che aiuta nel calcolo di x3
pub struct InfoPopolazioniNISECI {
  pub popolazione_piu_strutt: f32,
  pub species_strutt: u32,
  pub species_mediamente_strutt: u32,
  pub species_destrutt: u32,
  pub tot_species: usize,
  pub criterio_a: u8,
  pub criterio_b: u8,
  pub rapporto_ad_juv: Option<f32>,
}

impl InfoPopolazioniNISECI {
  pub fn new() -> InfoPopolazioniNISECI {
    InfoPopolazioniNISECI {
      popolazione_piu_strutt: 0.0,
      species_strutt: 0,
      species_mediamente_strutt: 0,
      species_destrutt: 0,
      tot_species: 0,
      criterio_a: 0,
      criterio_b: 0,
      rapporto_ad_juv: None,
    }
  }

  pub fn get_info_pop(map: &HashMap<String, ClassiEtaSpecieNISECI>) -> Result<InfoPopolazioniNISECI, Vec<String>> {
    let mut errors: Vec<String> = Vec::with_capacity(map.len()); // prenoto ora e poi restringo dopo

    let mut info_pop = InfoPopolazioniNISECI::new();
    info_pop.tot_species = map.len();
    let epsilon: f32 = 1e-6;
    for (_key, classe) in map {
      match classe.calculate_struttura_popolazione() {
        Ok((popolazione, criteri_x2_a)) => {
          if info_pop.popolazione_piu_strutt < popolazione {
            info_pop.popolazione_piu_strutt = popolazione;
          }
          if (popolazione - 1.0).abs() < epsilon {
            info_pop.species_strutt += 1;
          }
          if (popolazione - 0.5).abs() < epsilon {
            info_pop.species_mediamente_strutt += 1;
          }
          if popolazione.abs() < epsilon  {
            info_pop.species_destrutt += 1;
          }

          info_pop.criterio_a = criteri_x2_a.get_criterio_a();
          info_pop.criterio_b = criteri_x2_a.get_criterio_b();
          info_pop.rapporto_ad_juv = criteri_x2_a.get_rapporto_ad_juv();
        },
        Err(error) => errors.push(error),
      }
    }

    if errors.len() > 0 {
      errors.shrink_to_fit();
      return Err(errors);
    }

    Ok(info_pop)
  }
}

/// struct che aiuta nel calcolo di x3
/// una volta valorizzata avremmo tutte le informazioni utili
/// a calcolare x3
/// per ogni tipo di alloctono abbiamo una InfoPopolazioniNISECI
pub struct InfoPopolazioniAlieneNISECI {
  pub tipo_1: InfoPopolazioniNISECI,
  pub tipo_2: InfoPopolazioniNISECI,
  pub tipo_3: InfoPopolazioniNISECI,
  pub tot_specie_aliene: usize,
  pub tot_specie_autoctone: usize,
}

impl InfoPopolazioniAlieneNISECI {

  #[cfg(test)]
  pub fn new() -> InfoPopolazioniAlieneNISECI {
    InfoPopolazioniAlieneNISECI {
      tipo_1: InfoPopolazioniNISECI::new(),
      tipo_2: InfoPopolazioniNISECI::new(),
      tipo_3: InfoPopolazioniNISECI::new(),
      tot_specie_aliene: 0,
      tot_specie_autoctone: 0
    }
  }

  pub fn get_info_pop_aliene(classi_eta: &ClassiEtaAlieniNISECI) -> Result<InfoPopolazioniAlieneNISECI, Vec<String>> {
    let tipo_1 = match InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_1) {
      Ok(info) => info,
      Err(err) => return Err(err),
    };
    let tipo_2 = match InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_2) {
      Ok(info) => info,
      Err(err) => return Err(err),
    };
    let tipo_3 = match InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_3) {
      Ok(info) => info,
      Err(err) => return Err(err),
    };

    let info_pop_aliene = InfoPopolazioniAlieneNISECI {
      tipo_1,
      tipo_2,
      tipo_3,
      tot_specie_aliene: classi_eta.tot_specie_aliene,
      tot_specie_autoctone: classi_eta.tot_specie_autoctone
    };

    Ok(info_pop_aliene)
  }

  pub fn get_species_mediamente_strutt(&self) -> u32 {
    self.tipo_1.species_mediamente_strutt + self.tipo_2.species_mediamente_strutt + self.tipo_3.species_mediamente_strutt
  }
  pub fn get_species_destrutt(&self) -> u32 {
    self.tipo_1.species_destrutt + self.tipo_2.species_destrutt + self.tipo_3.species_destrutt
  }
}


/// struct che aiuta nel calcolo x3
/// ci aiuta a suddividere le specie aliene in base alla tipologia
pub struct ClassiEtaAlieniNISECI {
  pub map_tipo_1: HashMap<String, ClassiEtaSpecieNISECI>,
  pub map_tipo_2: HashMap<String, ClassiEtaSpecieNISECI>,
  pub map_tipo_3: HashMap<String, ClassiEtaSpecieNISECI>,
  pub tot_specie_aliene: usize,
  pub tot_specie_autoctone: usize
}

impl ClassiEtaAlieniNISECI {
  pub fn new() -> ClassiEtaAlieniNISECI {
    ClassiEtaAlieniNISECI {
      map_tipo_1: HashMap::with_capacity(10),
      map_tipo_2: HashMap::with_capacity(10),
      map_tipo_3: HashMap::with_capacity(10),
      tot_specie_aliene: 0,
      tot_specie_autoctone: 0
    }
  }
}


/// struct che quando valorizzata
/// esprime, data una SpecieNISECI,
/// il numero di esemplari trovati
/// suddivisi in base al numero di passaggio
pub struct EsemplariPerCattura {
  pub specie: SpecieNISECI,
  pub mappa: HashMap<u8, u32> // la key è il numero del passaggio
}

impl EsemplariPerCattura {
  pub fn new_prevalorized(numero_passaggio: u8, specie: &SpecieNISECI) -> EsemplariPerCattura {
    let mut mappa: HashMap<u8, u32> = HashMap::new();
    mappa.insert(numero_passaggio, 1);

    EsemplariPerCattura {
      specie: specie.clone(),
      mappa: mappa
    }
  }

  pub fn fill_passaggio(&mut self, numero_passaggio: u8) -> () {
    match self.mappa.entry(numero_passaggio) {
        Entry::Occupied(occupied) => {
          let numero_esemplari = occupied.get() + 1;
          self.mappa.insert(numero_passaggio, numero_esemplari);
        },
        Entry::Vacant(_) => {
          self.mappa.insert(numero_passaggio, 1);
        }
    }
  }
}

/// enum per il risultato finale di un calcolo niseci
/// (vedi calculate_stato_ecologico)
pub enum StatoEcologicoNISECI {
    Elevato,
    Buono,
    Moderato,
    Scadente,
    Cattivo
}

impl fmt::Display for StatoEcologicoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            StatoEcologicoNISECI::Elevato => "Elevato",
            StatoEcologicoNISECI::Buono => "Buono",
            StatoEcologicoNISECI::Moderato => "Moderato",
            StatoEcologicoNISECI::Scadente => "Scadente",
            StatoEcologicoNISECI::Cattivo => "Cattivo",
        };
        write!(f, "{}", string_representation)
    }
}
