use std::io::Cursor;
use crate::{translate_error_message, check_riferimento_niseci_reader, RIFERIMENTO_NISECI_HEADER};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_type() {
        let csv_data = format!(
            "{}\
    Cervo;Cervus elaphus;1234;Italia;abc;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
    ",
            RIFERIMENTO_NISECI_HEADER
        );
        let reader = Cursor::new(csv_data);
        let result = check_riferimento_niseci_reader(reader);

        assert!(result.is_err());
        let errors = result.err().unwrap();
        assert_eq!(errors.len(), 1); // One invalid record
        let translated_error = super::translate_error_message(&errors[0].to_string());
        assert!(translated_error.contains("tipo non valido"));
    }

    #[test]
    fn test_empty_csv() {
        let csv_data = RIFERIMENTO_NISECI_HEADER.to_string(); // Only header, no data
        let reader = Cursor::new(csv_data);
        let result = check_riferimento_niseci_reader(reader);

        assert!(result.is_ok());
        let records = result.unwrap();
        assert_eq!(records.len(), 0); // No records
    }

}
