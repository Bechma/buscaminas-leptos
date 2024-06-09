use leptos::*;

use crate::models::Board;

mod models;

#[component]
pub fn Minesweeper() -> impl IntoView {
    let (initial_height, initial_width, initial_mines) = (5, 5, 3);
    let board = create_rw_signal(Board::new(initial_width, initial_height, initial_mines));

    view! {
        <div class="flex flex-col">
        <div class="flex gap-x-2">
            <label for="height">Height</label>
            <input on:change=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap();
                board.update(|b| b.change_y(value));
            } id="height" type="number" value={initial_height} min=3 max=20 />
        </div>
        <div class="flex gap-x-2">
            <label for="width">Width</label>
            <input on:change=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap();
                board.update(|b| b.change_x(value));
            } id="width" type="number" value={initial_width} min=3 max=20 />
        </div>
        <div class="flex gap-x-2">
            <label for="number_of_mines">Number of mines</label>
            <input on:change=move |ev| {
                let value = event_target_value(&ev).parse::<usize>().unwrap();
                board.update(|b| b.change_mines(value));
            } id="number_of_mines" type="number" value={initial_mines} min=1 max=100 />
        </div>
        <div>
            <button class="bg-gray-200" on:click=move |_| {
                board.update(|new_b| *new_b = Board::new(new_b.x(), new_b.y(), new_b.number_of_mines()));
            }>Reset</button>
        </div>
            {move || board.with(|b| view!{
                <span>{format!("Game ended: {}", b.ended())}</span>
                <span>{format!("Remaining: {}", b.get_remaining())}</span>
                <span>{format!("Flags: {}", b.count_flags())}</span>
                <span>{format!("Won!: {}", b.get_remaining() == 0)}</span>
            })}
            <div>
                {move || board.with(|b| (0..b.y()).map(|y| {
                    view! {
                        <div class="flex">
                            {
                                (0..b.x()).map(|x| {
                                    let point = b.get_point(x, y);
                                    let cell = point.show_cell();
                                    let is_showing = point.show();
                                    view! {
                                        <div on:click=move |ev| {
                                                ev.prevent_default();
                                                if !is_showing {
                                                    board.update(|new_b| new_b.handle_click(x, y));
                                                }
                                            }
                                            on:contextmenu=move |ev| {
                                                ev.prevent_default();
                                                if !is_showing {
                                                        board.update(|new_b| new_b.put_flag(x, y));
                                                }
                                            }
                                            class="bg-gray-400 w-8 text-center border-solid border-2">
                                            {cell}
                                        </div>
                                    }
                                }).collect_view()
                            }
                        </div>
                    }
                }).collect_view())}
            </div>
        </div>
    }
}
