use super::*;

#[test]
fn inserts_valid_vector() {
    let mut db = VectorDb::new(3);

    let result = db.insert(1, vec![0.1, 0.2, 0.3]);

    assert!(result.is_ok());
    assert_eq!(db.vectors.get(&1), Some(&vec![0.1, 0.2, 0.3]));
}

#[test]
fn rejects_wrong_dimension() {
    let mut db = VectorDb::new(3);
    let result = db.insert(1, vec![0.1, 0.2]);

    assert!(matches!(
        result,
        Err(DbError::DimensionMismatch {
            expected: 3,
            actual: 2,
        })
    ));
    assert!(db.vectors.is_empty());
}
