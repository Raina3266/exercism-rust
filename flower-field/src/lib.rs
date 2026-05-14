struct Flower {
    data: Vec<bool>,
    width: usize,
}

impl Flower {
    fn parse(garden: &[&str]) -> Self {
        let data: Vec<bool> = garden
            .iter()
            .flat_map(|str| {
                str.chars().map(|char| match char {
                    '·' => false,
                    '*' => true,
                    _ => unreachable!(),
                })
            })
            .collect();
        Self { data, width: 5 }
    }
    fn get(&self, index: i32) -> Option<bool> {
        if index < 0 {
            None
        } else {
            Some(self.data[index as usize])
        }
    }
    fn count(&mut self, index: i32) -
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    todo!(
        "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    );
}
