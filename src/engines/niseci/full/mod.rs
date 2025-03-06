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

use std::collections::{hash_map::Entry, HashMap};
use crate::model::niseci::{CampionamentoNISECI, RiferimentoNISECI, AnagraficaNISECI, AreaNISECI, StatoEcologicoNISECI, ValoriIntermediSpecieNISECI, ValoriIntermediNISECI};

use super::x1::calculate_x1;
use super::x2::calculate_x2;
use super::x3::calculate_x3;

const RQE_NISECI_MAGIC_ADDEND: f32 = 1.1283;
const RQE_NISECI_MAGIC_QUOTIENT: f32 = 1.0603;
const STATO_ECOLOGICO_NISECI_SOGLIA_ELEVATO: f32 = 0.8;
const STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_ALPINA: f32 = 0.52;
const STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_MEDITERRANEA: f32 = 0.6;
const STATO_ECOLOGICO_NISECI_SOGLIA_MODERATO: f32 = 0.4;
const STATO_ECOLOGICO_NISECI_SOGLIA_SCADENTE: f32 = 0.2;

pub fn calculate_niseci(campionamento: &CampionamentoNISECI, riferimento: &RiferimentoNISECI, anagrafica: &AnagraficaNISECI) -> Result<(f32, ValoriIntermediNISECI), Vec<String>> {
    let mut errors = Vec::new();
    let x1 = calculate_x1(campionamento, riferimento);

    let x2 = calculate_x2(campionamento, anagrafica);
    match x2 {
        Ok(_) => {},
        Err(x2_errors) => {
            for e in x2_errors {
                errors.push(format!("Errore durante calcolo x2: {}", e));
            }
            return Err(errors);
        }
    }
    let (x2, criteri_x2) = x2.expect("calc_niseci() returned earlier on Err match");

    let mut valori_intermedi_specie: HashMap<String, ValoriIntermediSpecieNISECI> = HashMap::new();

    let submetriche_map = criteri_x2.get_submetriche_map();
    for (key, val) in submetriche_map.iter() {
        let criteri_x2_a = val.get_metriche_x2_a();
        let classi_eta = val.get_classi_eta();
        let specie = key.clone();
        let densita_stimata = val.get_metriche_x2_b().get_densita_stimata();
        let val = ValoriIntermediSpecieNISECI {
            classi_eta: classi_eta,
            densita_stimata: densita_stimata,
            x2_a_a: criteri_x2_a.get_criterio_a(),
            x2_a_b: criteri_x2_a.get_criterio_b(),
            rapporto_ad_juv: criteri_x2_a.get_rapporto_ad_juv(),
        };
        match valori_intermedi_specie.entry(specie) {
            Entry::Occupied(_) => {},
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(val);
            }
        }
    }

    let x3 = calculate_x3(campionamento);
    match x3 {
        Ok(_) => {},
        Err(x3_errors) => {
            for e in x3_errors {
                errors.push(format!("Errore durante calcolo x3: {}", e));
            }
            return Err(errors);
        }
    }
    let (x3, criteri_x3) = x3.expect("calc_niseci() returned earlier on Err match");

    let mut x1_x2_errors = Vec::new();
    if x1 < 0.0 {
        x1_x2_errors.push(format!("Errore risultato x1: valore negativo: {}", x1));
    }
    if x2 < 0.0 {
        x1_x2_errors.push(format!("Errore risultato x2: valore negativo: {}", x2));
    }
    if x1_x2_errors.len() != 0 {
        return Err(x1_x2_errors);
    }
    let niseci = (0.1 * x1.sqrt()) +
        (0.1 * x2.sqrt()) +
        (0.8 * (x1 * x2)) -
        ( (0.1 * (1.0 - x3)) *
          ((0.1 * x1.sqrt()) + (0.1 * x2.sqrt()) + (0.8 * (x1 * x2)))
        );

    let intermediates = ValoriIntermediNISECI {
        specie_specifici: valori_intermedi_specie,
        x2_a: criteri_x2.get_criterio_a(),
        x2_b: criteri_x2.get_criterio_b(),
        x3_a: criteri_x3.get_criterio_a(),
        x3_b: criteri_x3.get_criterio_b(),
    };
    return Ok((niseci, intermediates));
}

pub fn calculate_rqe_niseci(niseci: f32) -> f32 {
    return (niseci.log(10.0) +  RQE_NISECI_MAGIC_ADDEND ) / RQE_NISECI_MAGIC_QUOTIENT;
}

pub fn calculate_stato_ecologico(niseci: f32, area: &AreaNISECI) -> StatoEcologicoNISECI {
    let rqe_niseci = calculate_rqe_niseci(niseci);
    if rqe_niseci >= STATO_ECOLOGICO_NISECI_SOGLIA_ELEVATO {
        return StatoEcologicoNISECI::Elevato;
    }
    match area {
        AreaNISECI::Alpina => {
            if rqe_niseci >= STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_ALPINA {
                return StatoEcologicoNISECI::Buono;
            }
        },
        AreaNISECI::Mediterranea => {
            if rqe_niseci >= STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_MEDITERRANEA {
                return StatoEcologicoNISECI::Buono;
            }
        }
    }
    if rqe_niseci >= STATO_ECOLOGICO_NISECI_SOGLIA_MODERATO {
        return StatoEcologicoNISECI::Moderato;
    }
    if rqe_niseci >= STATO_ECOLOGICO_NISECI_SOGLIA_SCADENTE {
        return StatoEcologicoNISECI::Scadente;
    }
    return StatoEcologicoNISECI::Cattivo;
}
