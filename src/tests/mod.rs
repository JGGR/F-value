#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::{RIFERIMENTO_NISECI_HEADER, CAMPIONAMENTO_NISECI_HEADER, translate_error_message, check_riferimento_niseci_reader, check_campionamento_niseci_reader};


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
}
