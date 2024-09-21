use leptos::{component, view, IntoView, ReadSignal, RwSignal, SignalGet, SignalGetUntracked, SignalSet, SignalUpdate, SignalWith};
use std::collections::HashSet;
use std::rc::Rc;

pub struct Board {
    inner: Vec<Vec<Rc<Point>>>,
    end: RwSignal<bool>,
    first_click: RwSignal<bool>,
    number_of_mines: usize,
    remaining: RwSignal<usize>,
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
        let total_cells = x * y;
        let (remaining, number_of_mines) = if total_cells <= number_of_mines {
            (1, total_cells - 1)
        } else {
            (total_cells - number_of_mines, number_of_mines)
        };
        Self {
            end: RwSignal::new(false),
            first_click: RwSignal::new(true),
            inner: (0..y).map(|_| (0..x).map(|_| Rc::new(Point::default())).collect()).collect(),
            number_of_mines,
            remaining: RwSignal::new(remaining),
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
        self.remaining.get()
    }

    pub fn ended(&self) -> bool {
        self.end.get()
    }

    pub fn get_point_view(&self, x: usize, y: usize, board: ReadSignal<Self>) -> impl IntoView {
        PointView(PointViewProps {
            point: self.inner[y][x].clone(),
            x,
            y,
            board,
        })
    }

    fn ground_mines(&self, first_click: (usize, usize)) {
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
            let Some(elem) = self.inner[mine.1].get(mine.0) else {
                continue;
            };
            elem.mine.set(true);
            for d in DIRECTIONS {
                match (mine.0.checked_add_signed(d.0), mine.1.checked_add_signed(d.1)) {
                    (Some(point_x), Some(point_y)) if self.point_in_limit(point_x, point_y) => {
                        log::debug!("parsing: ({point_y}, {point_x})");
                        self.inner[point_y][point_x].number.update(|n| *n += 1);
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

    fn end_game(&self) {
        self.end.set(true);
        for i in &self.inner {
            for j in i {
                j.show.set(true);
            }
        }
    }

    pub fn handle_click(&self, x: usize, y: usize) {
        if self.first_click.get_untracked() {
            self.first_click.set(false);
            self.ground_mines((x, y));
        }
        let Some(elem) = self.inner[y].get(x) else {
            return;
        };
        if elem.show.get_untracked() || self.end.get() || elem.flag.get_untracked() {
            return;
        }
        elem.show.set(true);
        elem.clicked.set(true);
        let mine = elem.mine.get_untracked();
        if mine {
            self.end_game();
        } else {
            self.remaining.update(|r| *r -= 1);
            if self.remaining.get_untracked() == 0 {
                self.end_game();
            }
        }
        if elem.number.get_untracked() == 0 && !mine {
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

    pub fn put_flag(&self, x: usize, y: usize) {
        if let Some(elem) = self.inner[y].get(x) {
            if !elem.show.get() {
                elem.flag.update(|x| *x = !*x);
            }
        }
    }

    pub fn count_flags(&self) -> usize {
        self.inner.iter().map(|x| x.iter().filter(|y| y.flag.get()).count()).sum()
    }
}

#[derive(Default, Clone, Debug)]
pub(crate) struct Point {
    mine: RwSignal<bool>,
    show: RwSignal<bool>,
    flag: RwSignal<bool>,
    clicked: RwSignal<bool>,
    number: RwSignal<usize>,
}

impl Point {
    fn show_cell(&self) -> String {
        if self.flag.get() {
            if self.show.get() && !self.mine.get() {
                "❌".to_string()
            } else {
                "🚩".to_string()
            }
        } else if self.show.get() {
            if self.mine.get() {
                if self.clicked.get() {
                    "💥".to_string()
                } else {
                    "💣".to_string()
                }
            } else {
                self.number.get().to_string()
            }
        } else {
            "-".to_string()
        }
    }

    /*
    fn to_view(&self, x: usize, y: usize, board: ReadSignal<Board>) -> impl IntoView {
        let cell = self.show_cell();
        let is_showing = || self.show.get();
        let point_class = move || format!("bg-gray-200 w-8 text-center border-solid border-2 m-0.5 font-bold {}", if is_showing() {
            match self.number.get() {
                0 => if self.mine.get() { "" } else { "text-transparent" },
                1 => "text-blue-500",
                2 => "text-green-500",
                3 => "text-red-800",
                4 => "text-blue-950",
                5 => "text-orange-500",
                6 => "text-cyan-800",
                7 => "text-black",
                8 => "text-stone-500",
                _ => unreachable!("you can't have more than 8 mines around you"),
            }
        } else {
            "drop-shadow"
        });
        view! {
            <div on:click={move |ev| {
                ev.prevent_default();
                if !is_showing() {
                    board.with(|b| b.handle_click(x, y));
                }
            }}
            on:contextmenu={move |ev| {
            ev.prevent_default();
            if !is_showing() {
                board.with(|b| b.put_flag(x, y));
            }
        }} class={point_class}>
                {cell}
            </div>
        }
    }
     */
}

#[component]
fn PointView(point: Rc<Point>, x: usize, y: usize, board: ReadSignal<Board>) -> impl IntoView {
    let show = point.show;
    let number = point.number;
    let mine = point.mine;
    let is_showing = move || show.get();
    let point_class = move || format!("bg-gray-200 w-8 text-center border-solid border-2 m-0.5 font-bold {}", if is_showing() {
        match number.get() {
            0 => if mine.get() { "" } else { "text-transparent" },
            1 => "text-blue-500",
            2 => "text-green-500",
            3 => "text-red-800",
            4 => "text-blue-950",
            5 => "text-orange-500",
            6 => "text-cyan-800",
            7 => "text-black",
            8 => "text-stone-500",
            n => unreachable!("you can't have {} in {},{} more than 8 mines around you", n, x, y),
        }
    } else {
        "drop-shadow"
    });
    view! {
            <div on:click={move |ev| {
                ev.prevent_default();
                if !is_showing() {
                    board.with(|b| b.handle_click(x, y));
                }
            }}
            on:contextmenu={move |ev| {
            ev.prevent_default();
            if !is_showing() {
                board.with(|b| b.put_flag(x, y));
            }
        }} class={point_class}>
                {move || point.show_cell()}
            </div>
        }
}