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

pub(crate) fn calc_dmig(campione: CampionamentoHFBI, anagrafica: AnagraficaHFBI) -> f32 {
    let mut smig = 0;
    let mut bmig = 0.0;
    for specie in campione.campionamento {
        match specie.specie.gruppo_eco {
            GruppoEcoHFBI::Diadromi
            | GruppoEcoHFBI::MigratoriMarini => {
                smig += 1;
                let density_factor = (100 * specie.peso) as f32 /
                (anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto);
                bmig += specie.peso as f32 * density_factor;
            }
            _ => {}
        }
    }

    ((smig - 1) as f32 / bmig.ln() + 1.0).ln()
}
