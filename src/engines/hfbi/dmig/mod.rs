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

use std::collections::{HashMap};

use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI, GruppoEcoHFBI, SpecieHFBI};

pub(crate) fn calc_dmig(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let bmig = calc_bmig(campione, anagrafica);

    let mut specie_map: HashMap<String, SpecieHFBI> = HashMap::with_capacity(10);
    // trovo il numero di specie trovate
    for cattura in &campione.campionamento {
        match cattura.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini => {
                specie_map.insert(cattura.specie.codice_specie.to_string(), cattura.specie.clone());
            },
            _ => {}
        }
    }

    let smig = specie_map.len();

    if smig == 0 {
        return 0.0;
    }

    if smig == 1 {
        return 0.01;
    }

    (((smig as f32 - 1.0) / bmig) + 1.0).ln()
}

fn calc_bmig(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut biomig = 0.0;
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini => {
                biomig += specie.peso as f32
            }
            _ => {}
        }
    }

    let area = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;

    ((biomig / area) * 100.0 +1.0).ln()
}
