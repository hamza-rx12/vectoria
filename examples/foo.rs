use vectoria::VectorDb;

fn main() {
    let dimension = 3;
    let mut  db = VectorDb::new(dimension);
    let _result = db.insert(12, vec![1.0,1.3,3.7]);
    let _result = db.insert(13, vec![0.1,0.3,0.7]);

    println!("{}", db);
}
