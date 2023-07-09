use stylist::{yew::styled_component, Style};
use yew::{Html, html, use_state, Callback};

const CSS: &str = grass::include!("src/components/counter/Counter.scss");

#[styled_component(Counter)]
pub fn counter() -> Html {
    let stylesheet = Style::new(CSS).unwrap();
    let count = use_state(|| 0);
    let count_clone = count.clone();
    let add = Callback::from(move |_| count_clone.set(*count_clone + 1));

    html!{
        <div class={stylesheet}>
            <p>
                <button onclick={add}>{"Count is: "}{*count}</button>
            </p>
        </div>
    }
}