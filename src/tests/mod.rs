#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{RIFERIMENTO_NISECI_HEADER, CAMPIONAMENTO_NISECI_HEADER};
    use crate::{translate_error_message};
    use crate::{check_riferimento_niseci_reader, check_campionamento_niseci_reader};
    use crate::{RecordCsvRiferimentoNISECI, check_records_riferimento_niseci};
    use crate::{RecordCsvCampionamentoNISECI, check_records_campionamento_niseci};
    use crate::model::niseci::SpecieNISECI;
    use crate::engines::niseci::linear_regression::{calculate_quantita_stimata, gradient_descent_iterate};

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
    fn test_csv_campionamento_niseci_found_string_expect_int() {
        let csv_data = format!(
            "{}\n07/07/2019;2190627 Reno 390;abc;c1;BA;275;152",
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
            "{}\n07/07/2019;2190627 Reno 390;;c1;BA;275;152",
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
            "{}\n07/07/2019;2190627 Reno 390;75.0;c1;BA;275;152",
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
            "{}\n07/07/2019;2190627 Reno 390;75.0;c1;BA;275",
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
            "{}\n07/07/2019;2190627 Reno 390;750;c1;BA;275;152
            07/07/2019;2190627 Reno 390;750;1;BA;275;152
            abc;2190627 Reno 390;750;c1;BA;275;152",
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
            specie_attesa: true
        };

        let riferimento_specie = vec![specie_1];

        let record_1 = RecordCsvCampionamentoNISECI {
            data: "07/07/2007".to_string(),
            stazione: "Foo".to_string(),
            superficie: 420,
            num_passaggio: "c1".to_string(),
            codice_specie: "1234".to_string(),
            lunghezza: 100,
            peso: 100
        };
        let recordcsv_data = vec![record_1];
        let result = check_records_campionamento_niseci(recordcsv_data, riferimento_specie);

        assert!(!result.is_err());
    }

    #[test]
    fn test_linear_regression() {
        let records = [100, 75, 50];

        let (m_final, b_final) = gradient_descent_iterate(&records);

        println!("{}, {}", m_final, b_final);
        assert_eq!(m_final, -25);
        assert_eq!(b_final, 125);
    }

    #[test]
    fn test_quantita_stimata() {
        let passaggi = [100, 75, 50];

        let quantita_stimata = calculate_quantita_stimata(&passaggi);

        assert!(quantita_stimata.is_ok());
        assert_eq!(quantita_stimata.unwrap(), 250);
    }

    #[test]
    fn test_quantita_stimata_err() {
        let passaggi = [50, 75, 100];

        let quantita_stimata = calculate_quantita_stimata(&passaggi);

        assert!(quantita_stimata.is_err());
        assert!(quantita_stimata.is_err_and(|e| e == -1));
    }
}
