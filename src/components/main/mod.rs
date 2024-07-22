use stylist::{yew::styled_component, Style};
use yew::{Html, html};

use crate::components::counter::Counter;

const CSS: &str = grass::include!("src/components/main/Main.scss");

#[styled_component(Main)]
pub fn main_page() -> Html{
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
            <main>
                <p class="img-holder">
                   <img src="assets/imgs/logo.gif" alt="logo"/>
                </p>
                <h1>{"Yew.rs Template by Vincent"}</h1>
                <h4>{"Do you want to build a webpage without writing any JavaScript or TypeScript?"}</h4>
                <h2 class="margin-top-20">{"Clone my repo:"}</h2>
                <p class="shell-style">{"git clone https://github.com/Vincent-the-gamer/yew-template.git"}</p>
                <Counter/>
            </main>
        </div>
    }
}