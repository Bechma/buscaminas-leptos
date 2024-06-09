use std::collections::HashSet;

pub struct Board {
    inner: Vec<Vec<Point>>,
    end: bool,
    first_click: bool,
    number_of_mines: usize,
    remaining: usize,
    x: usize,
    y: usize,
}

static DIRECTIONS: [(isize, isize); 8] = [
    (1, -1), (1, 0), (1, 1),
    (0, -1), /*(0, 0)*/ (0, 1),
    (-1, -1), (-1, 0), (-1, 1),
];


impl Board {
    pub fn new(x: usize, y: usize, number_of_mines: usize) -> Self {
        Self {
            end: false,
            first_click: true,
            inner: vec![vec![Point::default(); x]; y],
            number_of_mines,
            remaining: (x * y) - number_of_mines,
            x,
            y,
        }
    }

    pub fn change_x(&mut self, x: usize) {
        *self = Self::new(x, self.y, self.number_of_mines);
    }

    pub fn change_y(&mut self, y: usize) {
        *self = Self::new(self.x, y, self.number_of_mines);
    }

    pub fn change_mines(&mut self, mines: usize) {
        *self = Self::new(self.x, self.y, mines);
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn number_of_mines(&self) -> usize {
        self.number_of_mines
    }

    pub fn get_remaining(&self) -> usize {
        self.remaining
    }

    pub fn ended(&self) -> bool {
        self.end
    }

    pub fn get_point(&self, x: usize, y: usize) -> &Point {
        &self.inner[y][x]
    }

    fn ground_mines(&mut self, first_click: (usize, usize)) {
        let mut mines_position = HashSet::new();
        mines_position.insert(first_click);
        let number_of_mines = self.number_of_mines + 1; // +1 because of the first_click
        while mines_position.len() < number_of_mines {
            let mut random = [0; 2];
            getrandom::getrandom(&mut random).unwrap();
            mines_position.insert(((random[0] as usize) % self.x, (random[1] as usize) % self.y));
        }
        mines_position.remove(&first_click);
        for mine in mines_position.into_iter() {
            log::debug!("mine: ({}, {})", mine.1, mine.0);
            let Some(elem) = self.inner[mine.1].get_mut(mine.0) else {
                continue;
            };
            elem.mine = true;
            for d in DIRECTIONS {
                match (mine.0.checked_add_signed(d.0), mine.1.checked_add_signed(d.1)) {
                    (Some(point_x), Some(point_y)) if self.point_in_limit(point_x, point_y) => {
                        log::debug!("parsing: ({point_y}, {point_x})");
                        self.inner[point_y][point_x].number += 1;
                    }
                    (_, _) => {
                        // If there's an overflow, ignore that direction
                    }
                }
            }
        }
    }

    #[inline]
    fn point_in_limit(&self, x: usize, y: usize) -> bool {
        x < self.x && y < self.y
    }

    fn end_game(&mut self) {
        self.end = true;
        for i in &mut self.inner {
            for j in i {
                j.show = true;
                j.flag = false;
            }
        }
    }

    pub fn handle_click(&mut self, x: usize, y: usize) {
        if self.first_click {
            self.first_click = false;
            self.ground_mines((x, y));
        }
        let mine;
        let number;

        if let Some(elem) = self.inner[y].get_mut(x) {
            if elem.show || self.end || elem.flag {
                return;
            }
            elem.show = true;
            elem.clicked = true;
            (mine, number) = (elem.mine, elem.number);
        } else {
            return;
        };
        if mine {
            self.end_game();
        } else {
            self.remaining -= 1;
            if self.remaining == 0 {
                self.end_game();
            }
        }
        if number == 0 && !mine {
            for d in DIRECTIONS {
                match (x.checked_add_signed(d.0), y.checked_add_signed(d.1)) {
                    (Some(point_x), Some(point_y)) if self.point_in_limit(point_x, point_y) => {
                        self.handle_click(point_x, point_y);
                    }
                    (_, _) => {
                        // If there's an overflow, ignore that direction
                    }
                }
            }
        }
    }

    pub fn put_flag(&mut self, x: usize, y: usize) {
        if let Some(elem) = self.inner[y].get_mut(x) {
            if !elem.show {
                elem.flag = !elem.flag;
            }
        }
    }

    pub fn count_flags(&self) -> usize {
        self.inner.iter().map(|x| x.iter().filter(|y| y.flag).count()).sum()
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Point {
    mine: bool,
    show: bool,
    flag: bool,
    clicked: bool,
    number: usize,
}

impl Point {
    pub(crate) fn show_cell(&self) -> String {
        if self.flag {
            "🚩".to_string()
        } else if self.show {
            if self.mine {
                if self.clicked {
                    "💥".to_string()
                } else {
                    "💣".to_string()
                }
            } else {
                self.number.to_string()
            }
        } else {
            "-".to_string()
        }
    }

    pub(crate) fn show(&self) -> bool {
        self.show
    }
}