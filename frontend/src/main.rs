use yew::prelude::*;
use common::{db, models};

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| Model {
        value: 0
    });

    let onclick = {
        let counter = counter.clone();

        move |_| {
            counter.set(Model {
                value: counter.value + 1
            });
        }

        // Callback::from(move |_| {
        //     state.set(Model {
        //         value: state.value + 1
        //     })
        // })
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ counter.value }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

