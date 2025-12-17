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

use crate::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, CampionamentoNISECI, RiferimentoNISECI, StatoEcologicoNISECI,
    ValoriIntermediNISECI, ValoriIntermediSpecieNISECI,
};
use crate::engines::niseci::x2::MetricheX2;
use std::collections::{hash_map::Entry, HashMap};

use super::x1::calculate_x1;
use super::x2::calculate_x2;
use super::x2::calculate_x2_per_alloctone;
use super::x3::calculate_x3;

const RQE_NISECI_MAGIC_ADDEND: f32 = std::f32::consts::FRAC_2_SQRT_PI;
const RQE_NISECI_MAGIC_QUOTIENT: f32 = 1.0603;
const STATO_ECOLOGICO_NISECI_SOGLIA_ELEVATO: f32 = 0.8;
const STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_ALPINA: f32 = 0.52;
const STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_MEDITERRANEA: f32 = 0.6;
const STATO_ECOLOGICO_NISECI_SOGLIA_MODERATO: f32 = 0.4;
const STATO_ECOLOGICO_NISECI_SOGLIA_SCADENTE: f32 = 0.2;

pub fn calculate_niseci(
    campionamento: &CampionamentoNISECI,
    riferimento: &RiferimentoNISECI,
    anagrafica: &AnagraficaNISECI,
) -> Result<(Option<f32>, ValoriIntermediNISECI), Vec<String>> {
    let mut errors = Vec::new();
    let x1 = calculate_x1(campionamento, riferimento);

    let x2 = calculate_x2(campionamento, anagrafica, true);
    match x2 {
        Ok(_) => {}
        Err(x2_errors) => {
            for e in x2_errors {
                errors.push(format!("Errore durante calcolo x2: {}", e));
            }
            return Err(errors);
        }
    }
    let (x2, criteri_x2) = x2.expect("calc_niseci() returned earlier on Err match");

    // calculate x2 for specie non attese
    let x2_non_attese = calculate_x2(campionamento, anagrafica, false);
    match x2_non_attese {
        Ok(_) => {}
        Err(x2_non_attese_errors) => {
            for e in x2_non_attese_errors {
                errors.push(format!("Errore durante calcolo x2_non_attese: {}", e));
            }
            return Err(errors);
        }
    }

    let (_x2_non_attese, criteri_x2_non_attese) =
        x2_non_attese.expect("calc_niseci() returned earlier on Err match");

    // calculate x2 for specie alloctone
    let x2_per_alloctone = calculate_x2_per_alloctone(campionamento, anagrafica);
    match x2_per_alloctone {
        Ok(_) => {}
        Err(x2_per_alloctone_errors) => {
            for e in x2_per_alloctone_errors {
                errors.push(format!("Errore durante calcolo x2_per_alloctone: {}", e));
            }
            return Err(errors);
        }
    }

    let (_x2_per_alloctone, criteri_x2_per_alloctone) =
        x2_per_alloctone.expect("calc_niseci() returned earlier on Err match");

    let mut valori_intermedi_specie: HashMap<String, ValoriIntermediSpecieNISECI> = HashMap::new();

    // add valori intermedi specie attese
    valori_intermedi_specie.extend(get_valori_intermedi_specie(&criteri_x2));
    // add valori intermedi specie non attese
    let intermedi_non_attese = get_valori_intermedi_specie(&criteri_x2_non_attese);
    valori_intermedi_specie.extend(intermedi_non_attese);
    // add valori intermedi specie alloctone
    valori_intermedi_specie.extend(get_valori_intermedi_specie(&criteri_x2_per_alloctone));

    let x3 = calculate_x3(campionamento);
    match x3 {
        Ok(_) => {}
        Err(x3_errors) => {
            for e in x3_errors {
                errors.push(format!("Errore durante calcolo x3: {}", e));
            }
            return Err(errors);
        }
    }
    let (x3, criteri_x3) = x3.expect("calc_niseci() returned earlier on Err match");

    if let Some(ref crit) = criteri_x3 {
        let submetriche_map_x3 = crit.get_submetriche_map();
        for (key, val) in submetriche_map_x3 {
            let classi_eta = val.get_classi_eta();
            let specie = key.clone();
            let densita_stimata = -1.0; //TODO: check if this is correct
            let subvalue_a = val.get_criterio_a();
            let subvalue_b = val.get_criterio_b();
            let val = ValoriIntermediSpecieNISECI {
                classi_eta,
                densita_stimata,
                quantita_stimata: 0,
                x2_b: 0.0,
                rapporto_ad_juv: val.get_rapporto_ad_juv(),
                x2_a_a: subvalue_a,
                x2_a_b: subvalue_b,
            };
            match valori_intermedi_specie.entry(specie) {
                Entry::Occupied(_) => {}
                Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(val);
                }
            }
        }
    }

    let mut x1_x2_errors = Vec::new();
    if x1 < 0.0 {
        x1_x2_errors.push(format!("Errore risultato x1: valore negativo: {}", x1));
    }
    if let Some(val) = x2 {
        if val < 0.0 {
            x1_x2_errors.push(format!("Errore risultato x2: valore negativo: {}", val));
        }
    }
    if !x1_x2_errors.is_empty() {
        return Err(x1_x2_errors);
    }

    let intermediates = ValoriIntermediNISECI {
        x1,
        x2,
        x3,
        specie_specifici: valori_intermedi_specie,
        x2_a: criteri_x2.get_criterio_a(),
        x2_b: criteri_x2.get_criterio_b(),
        x3_a: criteri_x3.as_ref().map(|v| v.get_criterio_a()),
        x3_b: criteri_x3.as_ref().map(|v| v.get_criterio_b()),
    };

    match x2 {
        Some(x2_val) => {
            let niseci = (0.1 * x1.sqrt()) + (0.1 * x2_val.sqrt()) + (0.8 * (x1 * x2_val))
                - ((0.1 * (1.0 - x3))
                    * ((0.1 * x1.sqrt()) + (0.1 * x2_val.sqrt()) + (0.8 * (x1 * x2_val))));
            let rounded_niseci = (1000.0 * niseci).round() / 1000.0;
            Ok((Some(rounded_niseci), intermediates))
        }
        None => {
            // Nel caso in cui nessuna specie attesa sia presente nel campionamento
            Ok((None, intermediates))
        }
    }
}

pub fn calculate_rqe_niseci(niseci: Option<f32>) -> Option<f32> {
    let rqe =
        niseci.map(|val| (val.log(10.0) + RQE_NISECI_MAGIC_ADDEND) / RQE_NISECI_MAGIC_QUOTIENT);
    if let Some(r) = rqe {
        let rounded_rqe = (100.0 * r).round() / 100.0;
        Some(rounded_rqe)
    } else {
        rqe
    }
}

pub fn calculate_stato_ecologico_niseci(
    niseci: Option<f32>,
    area: &AreaNISECI,
) -> Option<StatoEcologicoNISECI> {
    let rqe_niseci = calculate_rqe_niseci(niseci);
    match rqe_niseci {
        Some(val) => {
            if val >= STATO_ECOLOGICO_NISECI_SOGLIA_ELEVATO {
                return Some(StatoEcologicoNISECI::Elevato);
            }
            match area {
                AreaNISECI::Alpina => {
                    if val >= STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_ALPINA {
                        return Some(StatoEcologicoNISECI::Buono);
                    }
                }
                AreaNISECI::Mediterranea => {
                    if val >= STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_MEDITERRANEA {
                        return Some(StatoEcologicoNISECI::Buono);
                    }
                }
            }
            if val >= STATO_ECOLOGICO_NISECI_SOGLIA_MODERATO {
                return Some(StatoEcologicoNISECI::Moderato);
            }
            if val >= STATO_ECOLOGICO_NISECI_SOGLIA_SCADENTE {
                return Some(StatoEcologicoNISECI::Scadente);
            }
            Some(StatoEcologicoNISECI::Cattivo)
        }
        None => None,
    }
}

fn get_valori_intermedi_specie(
    criteri: &MetricheX2,
) -> HashMap<String, ValoriIntermediSpecieNISECI> {
    let mut valori_intermedi_specie: HashMap<String, ValoriIntermediSpecieNISECI> = HashMap::new();

    let submetriche_map = criteri.get_submetriche_map();
    for (key, val) in submetriche_map.iter() {
        let criteri_x2_a = val.get_metriche_x2_a();
        let classi_eta = val.get_classi_eta();
        let specie = key.clone();
        let densita_stimata = val.get_metriche_x2_b().get_densita_stimata();
        let quantita_stimata = val.get_metriche_x2_b().get_quantita_stimata();
        let x2_b = val.get_metriche_x2_b().get_x2_b();
        let val = ValoriIntermediSpecieNISECI {
            classi_eta,
            densita_stimata,
            quantita_stimata,
            x2_b,
            x2_a_a: criteri_x2_a.get_criterio_a(),
            x2_a_b: criteri_x2_a.get_criterio_b(),
            rapporto_ad_juv: criteri_x2_a.get_rapporto_ad_juv(),
        };
        match valori_intermedi_specie.entry(specie) {
            Entry::Occupied(_) => {}
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(val);
            }
        }
    }

    valori_intermedi_specie
}
