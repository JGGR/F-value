use std::vec::Vec;
use std::fmt;

use crate::engines::niseci::linear_regression::Point;

use super::location::Location;

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
}

impl fmt::Display for SpecieNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!("SpecieNISECI: {{ id: {{{}}}, nome {{{}}}, tipo_autoctono: {{{}}}, tipo_alloctono: {{{}}}, specie_attesa: {{{}}}",
                self.id, self.nome, self.tipo_autoctono, self.tipo_alloctono, self.specie_attesa);
        write!(f, "{}", string_representation)
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
  tipo: TipoComunitaNISECI,
  fonte: Option<String>,
  numero_protocollo: Option<String>
}

#[derive(Clone)]
pub struct AnagraficaNISECI {
  comunita: ComunitaNISECI,
  codice_stazione: u32,
  nome_fiume: String,
  bacino_appartenenza: String,
  idro_eco_regione: IdroEcoRegioneNISECI,
  posizione: Location,
  lunghezza_media_stazione: f32,
  larghezza_media_stazione: f32,
  denasita_stimata: u32
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
pub struct RisultatoNISECI {
  valore: f32,
  rqe: f32,
  anagrafica: AnagraficaNISECI
}

pub struct ClassiEtaSpecieNISECI {
  pub cl1: i32,
  pub cl2: i32,
  pub cl3: i32,
  pub cl4: i32,
  pub cl5: i32,
}


impl ClassiEtaSpecieNISECI {
  pub fn new() -> ClassiEtaSpecieNISECI {
    ClassiEtaSpecieNISECI {
      cl1: 0,
      cl2: 0,
      cl3: 0,
      cl4: 0,
      cl5: 0,
    }

  }

  pub fn new_cl_prevalorizzata(cl: ClassiEta) -> ClassiEtaSpecieNISECI {
    let mut classe = ClassiEtaSpecieNISECI::new();
    match cl {
      ClassiEta::CL1 => classe.cl1 = 1,
      ClassiEta::CL2 => classe.cl2 = 1,
      ClassiEta::CL3 => classe.cl3 = 1,
      ClassiEta::CL4 => classe.cl4 = 1,
      ClassiEta::CL5 => classe.cl5 = 1,
    }

    classe
  }
}

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

