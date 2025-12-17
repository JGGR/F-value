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

use once_cell::sync::Lazy;

use crate::domain::location::Location;

#[derive(Debug, Clone)]
pub enum GruppoEcoHFBI {
    MigratoriMarini,
    Diadromi,
    ResidentiDiEstuario,
    #[allow(dead_code)]
    OccasionaliMarini,
    #[allow(dead_code)]
    OccasionaliDiAcqueDolci,
}

impl fmt::Display for GruppoEcoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            GruppoEcoHFBI::MigratoriMarini => "MigratoriMarini",
            GruppoEcoHFBI::Diadromi => "Diadromi",
            GruppoEcoHFBI::ResidentiDiEstuario => "ResidentiDiEstuario",
            GruppoEcoHFBI::OccasionaliMarini => "OccasionaliMarini",
            GruppoEcoHFBI::OccasionaliDiAcqueDolci => "OccasionaliDiAcqueDolci",
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, Clone)]
pub struct GruppoTrofHFBI {
    pub microbentivori: f32,
    pub macrobentivori: f32,
    pub iperbentivori: f32,
    pub erbivori: f32,
    pub detritivori: f32,
    pub planctivori: f32,
    pub onnivori: f32,
}

impl fmt::Display for GruppoTrofHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "{}, {}, {}, {}, {}, {}, {}",
            self.microbentivori,
            self.macrobentivori,
            self.iperbentivori,
            self.erbivori,
            self.detritivori,
            self.planctivori,
            self.onnivori
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, Clone)]
pub struct SpecieHFBI {
    pub nome_comune: &'static str,
    pub codice_specie: &'static str,
    pub autoctono: bool,
    pub gruppo_eco: GruppoEcoHFBI,
    pub gruppo_trofico: GruppoTrofHFBI,
}

impl fmt::Display for SpecieHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let autoctono_str = match self.autoctono {
            true => "SI".to_string(),
            false => "NO".to_string(),
        };
        let string_representation = format!(
            "{}, {}, {}, {}, {}",
            self.nome_comune,
            self.codice_specie,
            autoctono_str,
            self.gruppo_eco,
            self.gruppo_trofico
        );
        write!(f, "{}", string_representation)
    }
}

pub const RIFERIMENTO_HFBI: [SpecieHFBI; 31] = [
    SpecieHFBI {
        nome_comune: "Cheppia",
        codice_specie: "CH",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::Diadromi,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Anguilla",
        codice_specie: "AN",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::Diadromi,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.2,
            macrobentivori: 0.4,
            iperbentivori: 0.4,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Nono",
        codice_specie: "NO",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.5,
            macrobentivori: 0.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.5,
        },
    },
    SpecieHFBI {
        nome_comune: "Latterino di lago",
        codice_specie: "LAT",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Aguglia",
        codice_specie: "BBE",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Gallinella",
        codice_specie: "CLU",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.4,
            macrobentivori: 0.4,
            iperbentivori: 0.2,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Muggine labbrone",
        codice_specie: "CEL",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.5,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Spigola branzino",
        codice_specie: "DIC",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Alice (Acciuga Europea)",
        codice_specie: "DIC",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 1.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Ghiozzo nero",
        codice_specie: "GHN",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.4,
            macrobentivori: 0.4,
            iperbentivori: 0.2,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Cavalluccio marino",
        codice_specie: "HGU",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.5,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Cavalluccio camuso",
        codice_specie: "HHI",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.5,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Ghiozzetto di laguna",
        codice_specie: "GHL",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0 / 3.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Muggine dorato",
        codice_specie: "CED",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.5,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Muggine calamita",
        codice_specie: "CEC",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::Diadromi,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.5,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Muggine musino",
        codice_specie: "MUS",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.5,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Cefalo",
        codice_specie: "MUG",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::Diadromi,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.5,
            erbivori: 0.0,
            detritivori: 0.5,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Triglia di scoglio",
        codice_specie: "MSU",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 1.0 / 3.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Pesce ago sottile",
        codice_specie: "NOP",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 1.0,
            macrobentivori: 0.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Passera pianuzza",
        codice_specie: "PFL",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.4,
            macrobentivori: 0.4,
            iperbentivori: 0.2,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Ghiozzetto cenerino",
        codice_specie: "GHC",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0 / 3.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Ghiozzetto marmorizzato",
        codice_specie: "GHM",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0 / 3.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Ghiozzetto minuto",
        codice_specie: "GHE",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0 / 3.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Bavosa pavone",
        codice_specie: "BAP",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.5,
            macrobentivori: 0.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.5,
        },
    },
    SpecieHFBI {
        nome_comune: "Sardina",
        codice_specie: "SPI",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 1.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Sogliola comune",
        codice_specie: "SSO",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 1.0 / 3.0,
            iperbentivori: 0.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Orata",
        codice_specie: "SAU",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.4,
            macrobentivori: 0.2,
            iperbentivori: 0.4,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Pesce ago di rio",
        codice_specie: "PAR",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 2.0 / 3.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0 / 3.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Pesce ago adriatico",
        codice_specie: "STA",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.0,
            macrobentivori: 0.0,
            iperbentivori: 1.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Pesce ago cavallino",
        codice_specie: "STY",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 0.2,
            macrobentivori: 0.0,
            iperbentivori: 0.8,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
    SpecieHFBI {
        nome_comune: "Ghiozzo gò",
        codice_specie: "GHG",
        autoctono: true,
        gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
        gruppo_trofico: GruppoTrofHFBI {
            microbentivori: 1.0 / 3.0,
            macrobentivori: 1.0 / 3.0,
            iperbentivori: 1.0 / 3.0,
            erbivori: 0.0,
            detritivori: 0.0,
            planctivori: 0.0,
            onnivori: 0.0,
        },
    },
];

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct RecordHFBI {
    pub specie: SpecieHFBI,
    pub numero_individui: u32, // in millimetri
    pub peso: f32,             // in grammi
}

impl fmt::Display for RecordHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordHFBI: {{ specie: {{{}}}, numero_individui: {{{}}}, peso: {{{}}}",
            self.specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone)]
pub struct CampionamentoHFBI {
    pub campionamento: Vec<RecordHFBI>,
}

impl fmt::Display for CampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = "CampionaHFBI: {".to_string();
        for r in &self.campionamento {
            string_representation = format!("{string_representation}\n  {{{r}}},");
        }
        string_representation = format!("{string_representation}\n}}");
        write!(f, "{}", string_representation)
    }
}

impl CampionamentoHFBI {
    pub fn new(campionamento: Vec<RecordHFBI>) -> Self {
        Self { campionamento }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TipoLagunaCostieraHFBI {
    MAt1,
    MAt2,
    MAt3,
}

impl fmt::Display for TipoLagunaCostieraHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            TipoLagunaCostieraHFBI::MAt1 => "M-AT-1",
            TipoLagunaCostieraHFBI::MAt2 => "M-AT-2",
            TipoLagunaCostieraHFBI::MAt3 => "M-AT-3",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for TipoLagunaCostieraHFBI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == TipoLagunaCostieraHFBI::MAt1 as i32 => Ok(TipoLagunaCostieraHFBI::MAt1),
            x if x == TipoLagunaCostieraHFBI::MAt2 as i32 => Ok(TipoLagunaCostieraHFBI::MAt2),
            x if x == TipoLagunaCostieraHFBI::MAt3 as i32 => Ok(TipoLagunaCostieraHFBI::MAt3),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StagioneHFBI {
    Primavera,
    Autunno,
}

impl fmt::Display for StagioneHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            StagioneHFBI::Primavera => "Primavera",
            StagioneHFBI::Autunno => "Autunno",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for StagioneHFBI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == StagioneHFBI::Primavera as i32 => Ok(StagioneHFBI::Primavera),
            x if x == StagioneHFBI::Autunno as i32 => Ok(StagioneHFBI::Autunno),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum HabitatHFBI {
    Vegetato,
    NonVegetato,
}

impl fmt::Display for HabitatHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            HabitatHFBI::Vegetato => "Vegetato",
            HabitatHFBI::NonVegetato => "Non Vegetato",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for HabitatHFBI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == HabitatHFBI::Vegetato as i32 => Ok(HabitatHFBI::Vegetato),
            x if x == HabitatHFBI::NonVegetato as i32 => Ok(HabitatHFBI::NonVegetato),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct AnagraficaHFBIDraft {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub posizione: Location,
    pub date_string: String, // Formato gg/mm/aaaa
    pub tipo_laguna: TipoLagunaCostieraHFBI,
    pub stagione: StagioneHFBI,
    pub habitat_vegetato: HabitatHFBI,
    pub lunghezza_media_transetto: String,
    pub larghezza_media_transetto: String,
}

#[derive(Clone)]
pub struct AnagraficaHFBI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub posizione: Location,
    pub date_string: String, // Formato gg/mm/aaaa
    pub tipo_laguna: TipoLagunaCostieraHFBI,
    pub stagione: StagioneHFBI,
    pub habitat_vegetato: HabitatHFBI,
    pub lunghezza_media_transetto: f32,
    pub larghezza_media_transetto: f32,
}

impl AnagraficaHFBI {
    pub fn get_lunghezza_media(&self) -> f32 {
        self.lunghezza_media_transetto
    }
    pub fn get_larghezza_media(&self) -> f32 {
        self.larghezza_media_transetto
    }

    pub fn get_cond_riferimento_key(&self) -> CondizioniRiferimentoKeyHFBI {
        CondizioniRiferimentoKeyHFBI {
            tipo_laguna: self.tipo_laguna.clone(),
            stagione: self.stagione.clone(),
            habitat_vegetato: self.habitat_vegetato.clone(),
        }
    }
}

impl fmt::Display for AnagraficaHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!("AnagraficaHFBI: {{ codice_stazione {{{}}}, corpo_idrico: {{{}}}, posizione: {{{}}}, data: {{{}}}, tipo_laguna: {{{}}}, stagione: {{{}}}, habitat: {{{}}}, lunghezza_transetto: {{{}}}, larghezza_transetto: {{{}}} }}",
        self.codice_stazione, self.corpo_idrico, self.posizione, self.date_string, self.tipo_laguna, self.stagione, self.habitat_vegetato, self.lunghezza_media_transetto, self.larghezza_media_transetto);
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone)]
pub struct ValoriIntermediHFBI {
    pub bbent: f32,
    pub bn: f32,
    pub dbent: f32,
    pub ddom: f32,
    pub dhzp: f32,
    pub dmig: f32,
    pub mmi: f32,
}

impl fmt::Display for ValoriIntermediHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = format!(
            "bbent: {}, bn: {}, dbent: {}, ddom: {}, dhzp: {}, dmig: {}, mmi: {}",
            self.bbent, self.bn, self.dbent, self.ddom, self.dhzp, self.dmig, self.mmi
        );

        string_representation = string_representation.to_string(); //FIXME: Why is this here?
        write!(f, "{}", string_representation)
    }
}

impl ValoriIntermediHFBI {
    pub fn log(&self) {
        //TODO: a proper format? we count on the embedded newlines to leverage the
        //chopping on newlines from add_console_message()
        println!("Valori intermedi: {{{self}}}");
    }
}

#[derive(Clone)]
pub struct RisultatoHFBI {
    valore: Option<f32>,
    intermediates: ValoriIntermediHFBI,
}

impl fmt::Display for RisultatoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let valore_str = match self.valore {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let string_representation = format!(
            "RisultatoHFBI: {{ valore HFBI: {{{}}}, intermediates: {{{}}} }}",
            valore_str, self.intermediates
        );
        write!(f, "{}", string_representation)
    }
}

impl RisultatoHFBI {
    pub fn new(valore: Option<f32>, intermediates: ValoriIntermediHFBI) -> Self {
        Self {
            valore,
            intermediates,
        }
    }
    pub fn get_valore(&self) -> Option<f32> {
        self.valore
    }
    pub fn get_intermediates(&self) -> ValoriIntermediHFBI {
        self.intermediates.clone()
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct CondizioniRiferimentoKeyHFBI {
    pub tipo_laguna: TipoLagunaCostieraHFBI,
    pub stagione: StagioneHFBI,
    pub habitat_vegetato: HabitatHFBI,
}

#[derive(Clone)]
pub struct CondizioniRiferimentoHFBI {
    pub bn: f32,
    pub ddom: f32,
    pub dmig: f32,
    pub bbent: f32,
    pub dbent: f32,
    pub dhzp: f32,
}

impl CondizioniRiferimentoHFBI {
    pub fn get_cond_riferimento(
        anagrafica: &AnagraficaHFBI,
    ) -> Option<&'static CondizioniRiferimentoHFBI> {
        let key = anagrafica.get_cond_riferimento_key();
        CONDIZIONI_RIFERIMENTO_HFBI_HASHMAP.get(&key)
    }
}

static CONDIZIONI_RIFERIMENTO_HFBI_HASHMAP: Lazy<
    HashMap<CondizioniRiferimentoKeyHFBI, CondizioniRiferimentoHFBI>,
> = Lazy::new(|| {
    HashMap::from([
        // M-AT-1 data
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.232,
                ddom: 2.052,
                dmig: 3.212,
                bbent: 6.537,
                dbent: 3.768,
                dhzp: 2.856,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.932,
                ddom: 2.268,
                dmig: 2.014,
                bbent: 6.867,
                dbent: 2.944,
                dhzp: 2.570,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.232,
                ddom: 1.784,
                dmig: 3.212,
                bbent: 7.242,
                dbent: 3.153,
                dhzp: 2.369,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.932,
                ddom: 2.001,
                dmig: 2.014,
                bbent: 7.572,
                dbent: 2.329,
                dhzp: 2.083,
            },
        ),
        // M-AT-2 data
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.539,
                ddom: 2.052,
                dmig: 3.212,
                bbent: 5.221,
                dbent: 3.768,
                dhzp: 2.856,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.238,
                ddom: 2.268,
                dmig: 2.014,
                bbent: 5.551,
                dbent: 2.944,
                dhzp: 2.570,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.539,
                ddom: 1.784,
                dmig: 3.212,
                bbent: 5.925,
                dbent: 3.153,
                dhzp: 2.369,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.238,
                ddom: 2.001,
                dmig: 2.014,
                bbent: 6.255,
                dbent: 2.329,
                dhzp: 2.083,
            },
        ),
        // M-AT-3 data
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.217,
                ddom: 2.052,
                dmig: 3.212,
                bbent: 4.561,
                dbent: 3.768,
                dhzp: 2.856,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.917,
                ddom: 2.268,
                dmig: 2.014,
                bbent: 4.891,
                dbent: 2.944,
                dhzp: 2.570,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.217,
                ddom: 1.784,
                dmig: 3.212,
                bbent: 5.265,
                dbent: 3.153,
                dhzp: 2.369,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.917,
                ddom: 2.001,
                dmig: 2.014,
                bbent: 5.595,
                dbent: 2.329,
                dhzp: 2.083,
            },
        ),
    ])
});

/// enum per il risultato finale di un calcolo hfbi
/// (vedi calculate_stato_ecologico)
pub enum StatoEcologicoHFBI {
    Eccellente,
    Buono,
    Sufficiente,
    Scarso,
    Cattivo,
}

impl fmt::Display for StatoEcologicoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            StatoEcologicoHFBI::Eccellente => "Eccellente",
            StatoEcologicoHFBI::Buono => "Buono",
            StatoEcologicoHFBI::Sufficiente => "Sufficiente",
            StatoEcologicoHFBI::Scarso => "Scarso",
            StatoEcologicoHFBI::Cattivo => "Cattivo",
        };
        write!(f, "{}", string_representation)
    }
}
