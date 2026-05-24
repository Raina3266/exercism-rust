use std::result;

enum Series {
    One,
    Two,
    Three,
    Four,
    Five,
}

impl Series {
    fn parse(books: &[u32]) -> Vec<Series> {
        let mut result = vec![];
        for book in books {
            let book = match book {
                1 => Series::One,
                2 => Series::Two,
                3 => Series::Three,
                4 => Series::Four,
                5 => Series::Five,
                _ => unreachable!(),
            };
            result.push(book);
        }
        result
    }
    fn discounted_price(set: &[Series]) -> f32 {
        match set.len() {
            1 => 8.0,
            2 => 15.2,
            3 => 21.6,
            4 => 25.6,
            5 => 30.0,
            _ => unreachable!(),
        }
    }
}

pub fn lowest_price(books: &[u32]) -> u32 {
    todo!("Find the lowest price of the bookbasket with books {books:?}")
}
