
#[derive(Clone, Debug, Default, Hash, Eq, PartialEq)]
pub struct Board {
    board: u32
}

impl Board {
    pub fn get_field(&self, field: usize) -> u32 {
        debug_assert!(field < 9);
        self.board >> (field * 3) & 0b111
    }

    fn set_field(&mut self, field: usize, val: u32) {
        debug_assert!(val < 8);
        debug_assert!(field < 9);
        let val = (val ^ self.get_field(field)) << (field * 3);
        self.board ^= val;
    }

    pub fn print(&self) {
        println!("=====");
        for i in 0..9 {
            print!("{}",self.get_field(i));
            if i % 3 == 2 {
                println!();
            }
        }
    }

    pub fn turn(&mut self, new_field: usize) {
        debug_assert!(self.get_field(new_field) == 0);
        for i in 0..9 {
            let a = self.get_field(i);
            self.set_field(i, a.saturating_sub(1));
        }
        self.set_field(new_field, 6);
    }

    pub fn is_won(&self) -> bool {
        let mut binary = 0;
        for i in 0..9 {
            let field = self.get_field(i);
            if field == 2 || field == 4 || field == 6 {
                binary ^= 1 << i;
            }
        }

        binary == 0b111000000 || binary == 0b000111000 || binary == 0b000000111 || binary == 0b100100100 ||
            binary == 0b010010010 || binary == 0b001001001 || binary == 0b100010001 || binary == 0b001010100
    }
}