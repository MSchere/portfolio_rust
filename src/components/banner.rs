use yew::prelude::*;
use crate::web_utils::web_content::WebContent;
use crate::web_utils::html_parser::HtmlParser;

#[function_component(Banner)]

pub fn banner() -> Html {
    let web_content = WebContent::new();
    let banner = web_content.get_banner();
    let html_parser = HtmlParser{};
    html! {
        <section class="section banner">
            <div class="container">
                <div class="section-box-banner">
                    <div class="content">
                        <div>
                            <h1>{&banner.pretitle}</h1>
                        </div>
                        <div class="banner-title">
                            <h2>{"Manuel R. Schere"}</h2>
                            <h3>{"Blockchain Developer"}</h3>
                        </div>
                        <div class="banner-description">
                            {html_parser.parse_string_vector_to_inner_html(&banner.description, "mt-4")}
                        </div>
                    </div>
                    <div class="div-btn-banner">
                        <a href="mailto:manu_schere@proton.me" target="_black" class="main-btn">{&banner.action_btn}</a>
                    </div>
                </div>
            </div>
        </section>
    }
}