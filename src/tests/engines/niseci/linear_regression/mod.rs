use crate::engines::niseci::linear_regression::{calculate_quantita_with_regression, gradient_descent_iterate, Point};


#[test]
fn test_linear_regression() {
    let records = [
        Point::new(1, 100),
        Point::new(2, 75),
        Point::new(3, 50),
    ];

    let result = gradient_descent_iterate(&records);
    assert!(result.is_ok());

    let (m_final, b_final) = result.unwrap();
    assert_eq!(m_final, -25.0);
    assert_eq!(b_final, 125.0);
}

#[test]
fn test_quantita_stimata() {
    let passaggi = [
        Point::new(1, 100),
        Point::new(2, 75),
        Point::new(3, 50),
    ];

    let quantita_stimata = calculate_quantita_with_regression(&passaggi);

    assert!(quantita_stimata.is_ok());
    assert_eq!(quantita_stimata.unwrap(), 5);
}

#[test]
fn test_quantita_stimata_2() {
    let passaggi = [
        Point::new(70, 70),
        Point::new(130, 60),
        Point::new(150, 20),
        Point::new(160, 10),
    ];

    let quantita_stimata = calculate_quantita_with_regression(&passaggi);

    assert!(quantita_stimata.is_ok());
    assert_eq!(quantita_stimata.unwrap(), 190);
}

#[test]
fn test_quantita_stimata_m_positive() {
    let passaggi = [
        Point::new(1, 50),
        Point::new(2, 75),
        Point::new(3, 100),
    ];
    let quantita_stimata = calculate_quantita_with_regression(&passaggi);

    assert!(quantita_stimata.is_ok());
    assert_eq!(quantita_stimata.unwrap(), 225);
}

