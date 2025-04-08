//! A Yew-based search bar component to be placed in the navigator component.
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::forms::Datalist;

#[function_component(SearchBar)]
pub fn search_bar() -> Html {
    let focus_state = use_state(|| false);
    let node = use_node_ref();

    let focus_state_clone = focus_state.clone();
    use_click_away(node.clone(), move |_: Event| {
        focus_state_clone.set(false);
    });

    let onclick = {
        let focus_state = focus_state.clone();
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            e.prevent_default();
            focus_state.set(true)
        })
    };

    let classes = format!("search{}", if *focus_state { " focus" } else { "" });

    html! {
        <div ref={node} id="search-bar" class={classes} onclick={onclick}>
            <Datalist<SearchableStruct, false> number_of_candidates={10} label="Search" show_label={false} show_load_more={false} />
        </div>
    }
}
