use csv::Reader;
use std::path::Path;

pub fn read_numbers_from_file<T: AsRef<Path>>(path: T) -> Vec<u32> {
    let mut csv_reader = Reader::from_path(path).unwrap();

    csv_reader
        .records()
        .map(|rec| rec
            .ok()
            .and_then(|r| r.get(0).and_then(|n| n.parse::<u32>().ok())))
        .filter(|n| n.is_some())
        .map(|n| n.unwrap())
        .collect()
}