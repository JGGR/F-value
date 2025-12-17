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
use crate::csv::deser::hfbi::{
    check_anagrafica_hfbi_reader, check_campionamento_hfbi_reader,
    VeryItalianRecordCsvAnagraficaHFBI, VeryItalianRecordCsvCampionamentoHFBI,
};
use crate::csv::parser::hfbi::{
    check_records_anagrafica_hfbi, check_records_campionamento_hfbi,
};
use crate::domain::hfbi::CampionamentoHFBI;
use crate::engines::hfbi::full::calculate_hfbi;
use std::io::Cursor;

const CAMPIONAMENTO_HFBI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../../../templates/campionamento_hfbi.csv");
const ANAGRAFICA_HFBI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../../../templates/anagrafica_hfbi.csv");

#[test]
fn calculate_hfbi_template() {
    let campionamento_reader = Cursor::new(CAMPIONAMENTO_HFBI_TEMPLATE_DATA);

    let campionamento_csv_check = check_campionamento_hfbi_reader::<
        _,
        VeryItalianRecordCsvCampionamentoHFBI,
    >(campionamento_reader, true);

    assert!(campionamento_csv_check.is_ok());

    let campionamento_csv_records = campionamento_csv_check.expect("is_ok() was checked before");

    let campionamento_value_check = check_records_campionamento_hfbi(campionamento_csv_records);

    assert!(campionamento_value_check.is_ok());

    let campionamento_specie = campionamento_value_check.expect("is_ok() was checked before");

    let anagrafica_reader = Cursor::new(ANAGRAFICA_HFBI_TEMPLATE_DATA);

    let anagrafica_csv_check = check_anagrafica_hfbi_reader::<_, VeryItalianRecordCsvAnagraficaHFBI>(
        anagrafica_reader,
        true,
    );

    assert!(anagrafica_csv_check.is_ok());

    let anagrafica_csv_records = anagrafica_csv_check.expect("is_ok() was checked before");

    let anagrafica_value_check = check_records_anagrafica_hfbi(anagrafica_csv_records);

    assert!(anagrafica_value_check.is_ok());

    let anagrafica = anagrafica_value_check.expect("is_ok() was checked before");

    let campionamento = CampionamentoHFBI {
        campionamento: campionamento_specie,
    };
    let calc_hfbi_res = calculate_hfbi(&campionamento, &anagrafica);

    assert!(calc_hfbi_res.is_ok());

    let (hfbi, intermediates) = calc_hfbi_res.expect("is_ok() was checked before");

    assert_eq!(hfbi, 1.3);
    assert_eq!(intermediates.mmi, 0.362);
    assert_eq!(intermediates.bbent, 3.638);
    assert_eq!(intermediates.bn, 1.587);
    assert_eq!(intermediates.dbent, 0.154);
    assert_eq!(intermediates.ddom, 0.551);
    assert_eq!(intermediates.dhzp, 0.417);
    assert_eq!(intermediates.dmig, 0.421);
}
