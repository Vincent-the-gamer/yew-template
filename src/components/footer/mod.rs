use stylist::{Style, yew::styled_component};
use yew::{Html, html};

const CSS: &str = grass::include!("src/components/footer/Footer.scss");

#[styled_component(Footer)]
pub fn footer() -> Html {

    let stylesheet = Style::new(CSS).unwrap();

    html!{
        <div class={stylesheet}>
          <footer>
            <h3>{"v1.0.1 | Vincent-the-gamer @2023-present"}</h3>
          </footer>
        </div>
    }
}