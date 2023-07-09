use stylist::{yew::styled_component, Style};
use yew::{Html, html};

const CSS: &str = grass::include!("src/components/header/Header.scss");

#[styled_component(Header)]
pub fn header() -> Html {
    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
            <header>
                <img src="assets/imgs/logo.gif" alt="logo"/>
                <h1>{"Yew.rs Template"}</h1>
                <button>
                    <a href="https://github.com/Vincent-the-gamer/yew-template" target="_blank">{"GitHub"}</a>
                </button>
            </header>
        </div>
    }
}
