use leptos::*;

use crate::models::Board;

mod models;

#[component]
pub fn Minesweeper() -> impl IntoView {
    let (initial_height, initial_width, initial_mines) = (10, 10, 10);
    let board = create_rw_signal(Board::new(initial_width, initial_height, initial_mines));

    view! {
        <div class="flex flex-col items-center gap-y-4">
            <div class="flex flex-col items-center bg-gray-200 rounded p-2">
                <h1 class="text-lg font-bold">Settings</h1>
                <div class="flex gap-x-2">
                    <div class="flex gap-x-2 bg-red-300">
                        <label for="height">Height</label>
                        <input on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<usize>().unwrap();
                            board.update(|b| b.change_y(value));
                        } id="height" type="number" value={initial_height}
                        prop:value=move || board.with(|b| b.y())
                        min=3 max=100 />
                    </div>
                    <div class="flex gap-x-2 bg-blue-300">
                        <label for="width">Width</label>
                        <input on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<usize>().unwrap();
                            board.update(|b| b.change_x(value));
                        } id="width" type="number" value={initial_width}
                        prop:value=move || board.with(|b| b.x())
                        min=3 max=100 />
                    </div>
                    <div class="flex gap-x-2 bg-pink-300">
                        <label for="number_of_mines">Number of mines</label>
                        <input on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<usize>().unwrap();
                            board.update(|b| b.change_mines(value));
                        } id="number_of_mines" type="number" value={initial_mines}
                        prop:value=move || board.with(|b| b.number_of_mines())
                        min=1 max=100 />
                    </div>
                    <div>
                        <button class="bg-gray-400 rounded-lg px-1 hover:bg-gray-500" on:click=move |_| {
                            board.update(|new_b| *new_b = Board::new(new_b.x(), new_b.y(), new_b.number_of_mines()));
                        }>Reset</button>
                    </div>
                </div>
            </div>
            <div class="flex flex-col items-center bg-gray-200 rounded p-2">
                <h1 class="text-lg font-bold">Game status</h1>
                <div class="flex gap-x-4">
                    {move || board.with(|b| {
                        let remaining = b.get_remaining();
                        let message = if b.ended() {
                            if remaining == 0 {
                                "You won! congratulations 🙌🎉🙌🎉🎉".to_string()
                            } else {
                                "You hit a mine! LoOoSeErr".to_string()
                            }
                        } else {
                            format!("Remaining cells: {} ||| Bombs flagged: {}", remaining, b.count_flags())
                        };
                        view! {
                            <span>{message}</span>
                        }
                    })}
                </div>
            </div>
            <div>
                {move || board.with(|b| (0..b.y()).map(|y| {
                    view! {
                        <div class="flex">
                            {
                                (0..b.x()).map(|x| {
                                    b.get_point_view(x, y, board)
                                }).collect_view()
                            }
                        </div>
                    }
                }).collect_view())}
            </div>
        </div>
    }
}
