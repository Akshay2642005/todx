#![allow(clippy::all, unused)]
use dioxus::prelude::*;

#[derive(PartialEq, Eq)]
pub enum FilterState {
    All,
    Completed,
    Active,
}

pub type Todos = im_rc::HashMap<u32, TodoItem>;

#[derive(PartialEq, Debug, Clone)]
pub struct TodoItem {
    pub id: u32,
    pub content: String,
    pub checked: bool,
}

#[component]
pub fn app() -> Element {
    let todos = use_signal(|| im_rc::HashMap::<u32, TodoItem>::default());
    let filter = use_signal(|| FilterState::All);
    let draft = use_signal(|| String::new());
    let todo_id = use_signal(|| 0);

    // Filter the todos based on the filter state
    let mut filtered_todos = todos
        .read()
        .iter()
        .filter(|(_, item)| {
            filter.with(|filter_state| match filter_state {
                FilterState::All => true,
                FilterState::Active => !item.checked,
                FilterState::Completed => item.checked,
            })
        })
        .map(|f| *f.0)
        .collect::<Vec<_>>();
    filtered_todos.sort_unstable();

    let show_clear_completed = todos.read().values().any(|todo| todo.checked);
    let items_left = filtered_todos.len();
    let item_text = match items_left {
        1 => "item",
        _ => "items",
    };

    //ui
    return rsx! {};
}
