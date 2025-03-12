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

use std::io::Cursor;

use crate::{check_campionamento_niseci_reader, check_records_campionamento_niseci, check_records_riferimento_niseci, check_riferimento_niseci_reader, check_anagrafica_niseci_reader, check_records_anagrafica_niseci, model::niseci::SpecieNISECI, translate_error_message, RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI, RecordCsvAnagraficaNISECI, CAMPIONAMENTO_NISECI_HEADER, RIFERIMENTO_NISECI_HEADER, ANAGRAFICA_NISECI_HEADER};

#[test]
fn test_csv_riferimento_niseci_found_string_expect_int() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;abc;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_found_string_expect_float() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;abc;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_riferimento_niseci_found_empty_string_expect_float() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_riferimento_niseci_found_float_expect_int() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1.0;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_err_found_cl_negative() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;abc;Italia;1;0;1;-10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_lessfields() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_valid_csv_riferimento_niseci() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;abc;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_riferimento_niseci() {
    let csv_data = RIFERIMENTO_NISECI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader(reader);

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_riferimento_niseci() {
    let record_1 = RecordCsvRiferimentoNISECI {
        nome_comune: "Cervo".to_string(),
        nome_latino: "Cervus elaphus".to_string(),
        codice_specie: "1234".to_string(),
        origine: "AUT".to_string(),
        tipo_autoctono: 1,
        allo_nocivita: 0,
        specie_attesa: 1,
        cl_soglia1: 10,
        cl_soglia2: 20,
        cl_soglia3: 30,
        cl_soglia4: 40,
        ad_juv_soglia1: 0.1,
        ad_juv_soglia2: 0.2,
        ad_juv_soglia3: 0.3,
        ad_juv_soglia4: 0.4,
        dens_soglia1: 0.1,
        dens_soglia2: 0.2,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_riferimento_niseci(recordcsv_data);

    assert!(!result.is_err());
}

#[test]
fn test_recordcsv_riferimento_niseci_soglie_cl_error() {
    let record_1 = RecordCsvRiferimentoNISECI {
        nome_comune: "Cervo".to_string(),
        nome_latino: "Cervus elaphus".to_string(),
        codice_specie: "1234".to_string(),
        origine: "AUT".to_string(),
        tipo_autoctono: 1,
        allo_nocivita: 0,
        specie_attesa: 1,
        cl_soglia1: 10,
        cl_soglia2: 50,
        cl_soglia3: 30,
        cl_soglia4: 40,
        ad_juv_soglia1: 0.1,
        ad_juv_soglia2: 0.2,
        ad_juv_soglia3: 0.3,
        ad_juv_soglia4: 0.4,
        dens_soglia1: 0.1,
        dens_soglia2: 0.2,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_riferimento_niseci(recordcsv_data);

    assert!(result.is_err());

    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_recordcsv_riferimento_niseci_soglie_ad_juv_error() {
    let record_1 = RecordCsvRiferimentoNISECI {
        nome_comune: "Cervo".to_string(),
        nome_latino: "Cervus elaphus".to_string(),
        codice_specie: "1234".to_string(),
        origine: "AUT".to_string(),
        tipo_autoctono: 1,
        allo_nocivita: 0,
        specie_attesa: 1,
        cl_soglia1: 10,
        cl_soglia2: 20,
        cl_soglia3: 30,
        cl_soglia4: 40,
        ad_juv_soglia1: 0.1,
        ad_juv_soglia2: 0.7,
        ad_juv_soglia3: 0.3,
        ad_juv_soglia4: 0.4,
        dens_soglia1: 0.1,
        dens_soglia2: 0.2,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_riferimento_niseci(recordcsv_data);

    assert!(result.is_err());

    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_csv_campionamento_niseci_found_string_expect_int() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;abc;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_campionamento_niseci_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_campionamento_niseci_found_float_expect_int() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;2.75;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_campionamento_niseci_lessfields() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;2.75",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_valid_csv_campionamento_niseci() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;275;152
        07/07/2019;2190627 Reno 390;1;BA;275;152
        abc;2190627 Reno 390;1;BA;275;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader(reader);

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_campionamento_niseci() {
    let csv_data = CAMPIONAMENTO_NISECI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader(reader);

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_campionamento_niseci() {
    let specie_1 = SpecieNISECI {
        id: "1234".to_string(),
        nome: "Cervus elaphus".to_string(),
        tipo_autoctono: 1,
        tipo_alloctono: 0,
        specie_attesa: true,
        cl_soglia1: 0, // in cm
        cl_soglia2: 0, // in cm
        cl_soglia3: 0, // in cm
        cl_soglia4: 0, // in cm
        ad_juv_soglia1: 0.0,
        ad_juv_soglia2: 0.0,
        ad_juv_soglia3: 0.0,
        ad_juv_soglia4: 0.0,
        dens_soglia1: 1.0,
        dens_soglia2: 2.0,
    };

    let riferimento_specie = vec![specie_1];

    let record_1 = RecordCsvCampionamentoNISECI {
        data: "07/07/2007".to_string(),
        stazione: "Foo".to_string(),
        num_passaggio: 1,
        codice_specie: "1234".to_string(),
        lunghezza: 100,
        peso: 100
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_campionamento_niseci(recordcsv_data, riferimento_specie);

    assert!(!result.is_err());
}

#[test]
fn test_csv_anagrafica_niseci_found_string_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;abc;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_anagrafica_niseci_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_anagrafica_niseci_found_float_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100,0;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_anagrafica_niseci_lessfields() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;0;;;20;1",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader(reader);

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = translate_error_message(&errors[0].to_string());
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_valid_csv_anagrafica_niseci() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader(reader);

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_anagrafica_niseci() {
    let csv_data = ANAGRAFICA_NISECI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader(reader);

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_anagrafica_niseci() {
    let record_1 = RecordCsvAnagraficaNISECI {
        codice_stazione: "Foo".to_string(),
        corpo_idrico: "Bar".to_string(),
        regione: "Foo".to_string(),
        provincia: "Bar".to_string(),
        data: "07/07/2007".to_string(),
        lunghezza_stazione: 100.0,
        larghezza_stazione: 6.0,
        tipo_comunita: 0,
        fonte: "foo".to_string(),
        numero_protocollo: "foo".to_string(),
        idro_eco_regione: 0,
        area_alpina: 0,
        nome_bacino: "Foo".to_string()
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_anagrafica_niseci(recordcsv_data);

    assert!(!result.is_err());
}
