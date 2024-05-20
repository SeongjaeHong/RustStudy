use gigasecond;
use gigasecond::dt;

fn main() {
    let start_date = dt(1979, 7, 8, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(1980, 7, 12, 0, 0, 0));
    println!("- - - - - - - - - - - - -");
    let start_date = dt(1975, 7, 8, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(1976, 7, 12, 0, 0, 0));
    println!("- - - - - - - - - - - - -");
    let start_date = dt(1976, 7, 8, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(1977, 7, 13, 0, 0, 0));
}

#[test]
fn date() {
    let start_date = dt(2011, 4, 25, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(2043, 1, 1, 1, 46, 40));
}
#[test]
#[ignore]
fn another_date() {
    let start_date = dt(1977, 6, 13, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(2009, 2, 19, 1, 46, 40));
}
#[test]
#[ignore]
fn third_date() {
    let start_date = dt(1959, 7, 19, 0, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(1991, 3, 27, 1, 46, 40));
}
#[test]
#[ignore]
fn datetime() {
    let start_date = dt(2015, 1, 24, 22, 0, 0);
    assert_eq!(gigasecond::after(start_date), dt(2046, 10, 2, 23, 46, 40));
}
#[test]
#[ignore]
fn another_datetime() {
    let start_date = dt(2015, 1, 24, 23, 59, 59);
    assert_eq!(gigasecond::after(start_date), dt(2046, 10, 3, 1, 46, 39));
}
