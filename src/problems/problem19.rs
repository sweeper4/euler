pub fn problem19() {
    let mut count = 0;
    let mut year = 1901;
    let mut month = 1;
    let mut day = 2;
    while year < 2001 {
        day = (day + num_days_in_month(year, month)) % 7;
        year += month / 12;
        month = (month % 12) + 1;
        if day == 0 {
            count += 1;
        }
    }
    println!("There are {} sundays on the first of the month between 1 Jan 1901 and 31 Dec 2000", count);
}

fn num_days_in_month(year:u32, month:u32) -> u32 {
    match month {
        9 => return 30,
        4 => return 30,
        6 => return 30,
        11 => return 30,
        2 => if year % 4 == 0 && year % 100 != 0 || year % 400 == 0 {
            return 29;
        } else {
            return 28;
        },
        _ => return 31
    }
}