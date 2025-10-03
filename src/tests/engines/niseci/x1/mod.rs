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

use crate::engines::niseci::x1::calculate_x1;
use crate::tests::test_utils::{
    create_dummy_campionamento_chopped, create_dummy_campionamento_full, create_dummy_riferimento,
};

/// in questo test il campionamento ha
/// tutte le specie attese dal riferimento
#[test]
fn test_calculate_x1_full_campionamento() {
    let riferimento = create_dummy_riferimento();
    let campionamento = create_dummy_campionamento_full();

    assert_eq!(calculate_x1(&campionamento, &riferimento), 1.0);
}

/// in questo test il campionamento non
/// non possiede tutte le specie del riferimento
#[test]
fn test_calculate_x1_mixed_campionamento() {
    let riferimento = create_dummy_riferimento();
    let campionamento = create_dummy_campionamento_chopped();
    let x1_calcolato = calculate_x1(&campionamento, &riferimento);
    let x1_atteso = (1000.0_f32 * 5.0 / 6.0).round() / 1000.0;
    let epsilon: f32 = 1e-6;
    assert!((x1_calcolato - x1_atteso).abs() < epsilon);
}
