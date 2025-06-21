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

pub(crate) fn calc_dbent(campione: CampionamentoHFBI, anagrafica: AnagraficaHFBI) -> f32 {
    let mut sbent = 0.0;
    let mut bbent = 0.0;
    for specie in campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini
            | GruppoEcoHFBI::ResidentiDiEstuario => {
                sbent += specie.specie.gruppo_trofico.microbentivori;
                sbent += specie.specie.gruppo_trofico.macrobentivori;
                let density_factor = (100 * specie.peso) as f32 /
                (anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto);
                bbent += specie.peso as f32 * density_factor;
            }
            _ => {}
        }
    }

    ((sbent - 1.0) / bbent.ln() + 1.0).ln()
}
