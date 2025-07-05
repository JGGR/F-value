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

use crate::domain::hfbi::{CampionamentoHFBI, AnagraficaHFBI, GruppoEcoHFBI};

pub(crate) fn calc_dhzp(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut shzp = 0.0;
    let bhzp = calc_bhzp(campione, anagrafica);
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
                shzp += specie.specie.gruppo_trofico.iperbentivori;
            }
            _ => {}
        }
    }

    let epsilon: f32 = 1e-6;
    if shzp.abs() < epsilon {
        return 0.0;
    }

    if (shzp - 0.2).abs() < epsilon {
        return 0.01;
    }

    (((shzp - 0.2) / bhzp) + 1.0).ln()
}


fn calc_bhzp(campione: &CampionamentoHFBI, anagrafica: &AnagraficaHFBI) -> f32 {
    let mut biohzp = 0.0;
    for specie in &campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
            biohzp += specie.peso as f32 * (specie.specie.gruppo_trofico.iperbentivori)
            }
            _ => {}
        }
    }

    let area = anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto;

    ((biohzp / area) * 100.0 + 1.0).ln()
}
