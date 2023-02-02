use super::Weapon;

const MAGAZINE_BULLET_NUM: u32 = 8;
const TOTAL_BULLET_NUM: u32 = 32;

pub struct Sheriff {
    current_magazine_bullet_num: u32,
    total_bullet_num: u32,
}

impl Sheriff {
    pub fn print_magazine(self) {
        print!("{}", self.show_magazine());
    }
}

impl Default for Sheriff {
    fn default() -> Self {
        Self {
            current_magazine_bullet_num: MAGAZINE_BULLET_NUM,
            total_bullet_num: TOTAL_BULLET_NUM,
        }
    }
}

impl Weapon for Sheriff {
    fn new() -> Sheriff {
        Sheriff::default()
    }

    fn get_atk() -> u32 {
        145
    }

    fn reload(&mut self) -> &mut Self {
        self.current_magazine_bullet_num = MAGAZINE_BULLET_NUM;
        self
    }

    fn fire(&mut self) -> &mut Self {
        if self.current_magazine_bullet_num == 0 {
            self.reload();
        }
        self.current_magazine_bullet_num -= 1;
        self.total_bullet_num -= 1;
        self
    }

    fn sub_fire(&mut self) -> &mut Self {
        self
    }

    fn show_magazine(self) -> String {
        format!(
            "{}/{}",
            self.current_magazine_bullet_num,
            self.total_bullet_num - self.current_magazine_bullet_num,
        )
    }
}
