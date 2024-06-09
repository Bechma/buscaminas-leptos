use leptos::*;

use crate::models::Board;

mod models;

#[component]
pub fn Minesweeper() -> impl IntoView {
    let (h, w) = (5, 5);
    let board = create_rw_signal(Board::new(h, w, 3));

    view! {
        <div class="flex flex-col">
            {move || board.with(|b| view!{
                <span>{format!("Game ended: {}", b.ended())}</span>
                <span>{format!("Remaining: {}", b.get_remaining())}</span>
                <span>{format!("Won!: {}", b.get_remaining() == 0 && !b.ended())}</span>
            })}
            <div>
                {move || board.with(|b| (0..h).map(|x| {
                    view! {
                        <div class="flex">
                            {
                                (0..w).map(|y| {
                                    let point = b.get_point(x, y);
                                    let cell = point.show_cell();
                                    let is_showing = point.is_showing();
                                    view! {
                                        <div on:click=move |_| {
                                            if !is_showing {
                                                board.update(|new_b| new_b.handle_click(x, y));
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
