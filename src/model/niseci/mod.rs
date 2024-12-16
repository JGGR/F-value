use std::vec::Vec;
use std::fmt;

use super::location::Location;

#[derive(Clone)]
pub struct SpecieNISECI {
  pub id: String,
  pub nome: String,
  pub tipo_autoctono: u8, // si potrebbe unire autoctono con alloctono?
  pub tipo_alloctono: u8, // ricordo che ne avevamo parlato
  pub specie_attesa: bool
}

pub struct RiferimentoNISECI {
  elenco_specie: Vec<SpecieNISECI>
}

pub struct RecordNISECI {
  pub specie: SpecieNISECI,
  pub passaggio_cattura: u8,
  pub lunghezza: u32, /// in millimetri
  pub peso: u32 // in grammi
}

pub struct CampionamentoNISECI {
  campionamento: Vec<RecordNISECI>
}

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

pub struct ComunitaNISECI {
  tipo: TipoComunitaNISECI,
  fonte: Option<String>,
  numero_protocollo: Option<String>
}

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

pub struct RisultatoNISECI {
  valore: f32,
  rqe: f32,
  anagrafica: AnagraficaNISECI
}
