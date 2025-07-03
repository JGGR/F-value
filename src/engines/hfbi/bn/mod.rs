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

pub(crate) fn calc_bn(campione: CampionamentoHFBI, anagrafica: AnagraficaHFBI) -> f32 {
    let mut b = 0.0;
    let mut n = 0.0;
    for specie in campione.campionamento {
        let density_factor = (100 * specie.peso) as f32 /
        (anagrafica.lunghezza_media_transetto * anagrafica.larghezza_media_transetto);
        b += specie.peso as f32 * density_factor;
        n += 1.0 * density_factor;
    }

    ((b / n) +1.0).ln()
}
