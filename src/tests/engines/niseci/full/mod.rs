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

use crate::csv::deser::niseci::{
    check_anagrafica_niseci_reader, check_campionamento_niseci_reader,
    check_riferimento_niseci_reader, VeryItalianRecordCsvAnagraficaNISECI,
    VeryItalianRecordCsvCampionamentoNISECI, VeryItalianRecordCsvRiferimentoNISECI,
};
use crate::csv::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci,
};
use crate::domain::niseci::{CampionamentoNISECI, RiferimentoNISECI};
use crate::{
    engines::niseci::full::calculate_niseci,
    tests::test_utils::{
        create_dummy_anagrafica, create_dummy_campionamento_chopped,
        create_dummy_campionamento_full, create_dummy_riferimento,
    },
};
use std::io::Cursor;

const RIFERIMENTO_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../../../templates/riferimento_niseci.csv");
const CAMPIONAMENTO_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../../../templates/campionamento_niseci.csv");
const ANAGRAFICA_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../../../templates/anagrafica_niseci.csv");

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

#[test]
fn calculate_niseci_template() {
    let riferimento_reader = Cursor::new(RIFERIMENTO_NISECI_TEMPLATE_DATA);
    let riferimento_csv_check = check_riferimento_niseci_reader::<
        _,
        VeryItalianRecordCsvRiferimentoNISECI,
    >(riferimento_reader, true);

    assert!(riferimento_csv_check.is_ok());

    let riferimento_csv_records = riferimento_csv_check.expect("is_ok() was checked before");

    let riferimento_value_check = check_records_riferimento_niseci(riferimento_csv_records);

    assert!(riferimento_value_check.is_ok());

    let riferimento_specie = riferimento_value_check.expect("is_ok() was checked before");

    let campionamento_reader = Cursor::new(CAMPIONAMENTO_NISECI_TEMPLATE_DATA);

    let campionamento_csv_check = check_campionamento_niseci_reader::<
        _,
        VeryItalianRecordCsvCampionamentoNISECI,
    >(campionamento_reader, true);

    assert!(campionamento_csv_check.is_ok());

    let campionamento_csv_records = campionamento_csv_check.expect("is_ok() was checked before");

    let campionamento_value_check =
        check_records_campionamento_niseci(campionamento_csv_records, riferimento_specie.clone());

    assert!(campionamento_value_check.is_ok());

    let campionamento_specie = campionamento_value_check.expect("is_ok() was checked before");

    let anagrafica_reader = Cursor::new(ANAGRAFICA_NISECI_TEMPLATE_DATA);

    let anagrafica_csv_check = check_anagrafica_niseci_reader::<
        _,
        VeryItalianRecordCsvAnagraficaNISECI,
    >(anagrafica_reader, true);

    assert!(anagrafica_csv_check.is_ok());

    let anagrafica_csv_records = anagrafica_csv_check.expect("is_ok() was checked before");

    let anagrafica_value_check = check_records_anagrafica_niseci(anagrafica_csv_records);

    assert!(anagrafica_value_check.is_ok());

    let anagrafica = anagrafica_value_check.expect("is_ok() was checked before");

    let campionamento = CampionamentoNISECI {
        campionamento: campionamento_specie,
    };
    let riferimento = RiferimentoNISECI {
        elenco_specie: riferimento_specie,
    };

    let calc_niseci_res = calculate_niseci(&campionamento, &riferimento, &anagrafica);

    assert!(calc_niseci_res.is_ok());

    let (niseci, intermediates) = calc_niseci_res.expect("is_ok() was checked before");

    assert_eq!(niseci, Some(0.209));
    assert_eq!(intermediates.x1, 0.429);
    assert_eq!(intermediates.x2, Some(0.267));
    assert_eq!(intermediates.x3, 1.0);
}
