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

    pub fn get_remaining(&self) -> usize {
        self.remaining
    }

    pub fn ended(&self) -> bool {
        self.end
    }

    pub fn get_point(&self, x: usize, y: usize) -> &Point {
        &self.inner[x][y]
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
            log::info!("{mine:?}");
            let Some(elem) = self.inner[mine.0].get_mut(mine.1) else {
                continue;
            };
            elem.mine = true;
            for d in DIRECTIONS {
                match (mine.0.checked_add_signed(d.0), mine.1.checked_add_signed(d.1)) {
                    (Some(point_x), Some(point_y)) if self.point_in_limit(point_x, point_y) => {
                        self.inner[point_x][point_y].number += 1;
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

    pub fn handle_click(&mut self, x: usize, y: usize) {
        if self.first_click {
            self.first_click = false;
            self.ground_mines((x, y));
        }
        let Some(elem) = self.inner[x].get_mut(y) else {
            return;
        };
        if elem.show || self.end {
            return;
        }
        elem.show = true;
        if elem.mine {
            self.end = true;
        } else {
            self.remaining -= 1;
        }
        if elem.number == 0 && !elem.mine {
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
}

#[derive(Default, Clone)]
pub(crate) struct Point {
    mine: bool,
    show: bool,
    number: usize,
}

impl Point {
    pub(crate) fn show_cell(&self) -> String {
        if self.show {
            if self.mine {
                "💣".to_string()
            } else {
                self.number.to_string()
            }
        } else {
            "-".to_string()
        }
    }

    pub(crate) fn is_showing(&self) -> bool {
        self.show
    }
}