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

use crate::{engines::niseci::full::calculate_niseci, tests::test_utils::{create_dummy_riferimento, create_dummy_campionamento_full, create_dummy_campionamento_chopped, create_dummy_anagrafica}};

#[test]
fn calculate_dummy_niseci_campionamento_full() {
    let riferimento = create_dummy_riferimento();
    let campionamento = create_dummy_campionamento_full();
    let anagrafica = create_dummy_anagrafica();
    let res = calculate_niseci(&campionamento, &riferimento, &anagrafica);

    assert!(res.is_ok());

    //TODO: assert the expected result
    //assert!(res.expect("calculate_dummy_niseci_campionamento_full(): is_ok was checked") == 3.14);
}

#[test]
fn calculate_dummy_niseci_campionamento_chopped() {
    let riferimento = create_dummy_riferimento();
    let campionamento = create_dummy_campionamento_chopped();
    let anagrafica = create_dummy_anagrafica();
    let res = calculate_niseci(&campionamento, &riferimento, &anagrafica);

    assert!(res.is_ok());

    //TODO: assert the expected result
    //assert!(res.expect("calculate_dummy_niseci_campionamento_chopped(): is_ok was checked") == 3.14);
}
