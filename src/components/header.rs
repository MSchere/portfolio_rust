use crate::web_utils::web_content::WebContent;
use web_sys::MouseEvent;
use yew::prelude::*;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let web_content = WebContent::new();
        let header = web_content.get_header();
        html! {
            <header class="navbar nav-shadow on-top">
                <div class="container">
                    <div class="nav-container">
                        <a href="#" class="nav-logo">
                            <img src="assets/images/logo.png" height="25" />
                        </a>
                        <nav class="nav-menu">
                            <a href="#about" class="nav-item"><span class="nav-number">{"01. "}</span><span class="underline nav-text">{&header.item1}</span></a>
                            <a href="#jobs" class="nav-item"><span class="nav-number">{"02. "}</span><span class="underline nav-text">{&header.item2}</span></a>
                            <a href="#projects" class="nav-item"><span class="nav-number">{"03. "}</span><span class="underline nav-text">{&header.item3}</span></a>
                            <a href="#contact" class="nav-item"><span class="nav-number">{"04. "}</span><span class="underline nav-text">{&header.item4}</span></a>
                        </nav>
                        <a onclick={download_file("assets/cv/Europass_EN.pdf")}class="main-btn cv-btn">{&header.cv_btn}</a>
                    </div>
                    <div class="menu-wrapper">
                        <div class="hamburger-menu"></div>
                    </div>
                    <div class="menu-responsive">
                        <aside class="on-top">
                            <nav class="nav-menu">
                                <a href="#about" class="nav-item"><span class="nav-number">{"01. "}</span><span class="underline nav-text">{&header.item1}</span></a>
                                <a href="#jobs" class="nav-item"><span class="nav-number">{"02. "}</span><span class="underline nav-text">{&header.item2}</span></a>
                                <a href="#projects" class="nav-item"><span class="nav-number">{"03. "}</span><span class="underline nav-text">{&header.item3}</span></a>
                                <a href="#contact" class="nav-item"><span class="nav-number">{"04. "}</span><span class="underline nav-text">{&header.item4}</span></a>
                            </nav>
                        </aside>
                    </div>
                </div>
            </header>
        }
    }
}

fn download_file(path: &str) -> Callback<MouseEvent> {
    let cloned_path = path.to_string();
    Callback::from(move |_| {
        let window = web_sys::window().unwrap();
        window.open_with_url(&cloned_path).unwrap();
    })
}
