fn count_sundays() -> u32 {
    let mut count = 0;
    let mut day_of_week = 2; // January 1, 1901 was a Tuesday (0 - Sunday, 1 - Monday, ..., 6 - Saturday)

    for year in 1901..=2000 {
        for month in 1..=12 {
            let is_sunday = day_of_week == 0;
            if is_sunday {
                count += 1;
            }

            // Determine the number of days in the current month
            let days_in_month = match month {
                4 | 6 | 9 | 11 => 30, // April, June, September, November
                2 => {
                    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                        29 // Leap year
                    } else {
                        28 // Non-leap year
                    }
                }
                _ => 31, // All other months
            };

            // Update the day of the week for the next month
            day_of_week = (day_of_week + days_in_month) % 7;
        }
    }

    count
}

fn main() {
    let sundays = count_sundays();
    println!("Number of Sundays that fell on the first of the month during the twentieth century: {}", sundays);
}

