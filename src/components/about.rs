use yew::prelude::*;
use crate::web_utils::web_content::WebContent;
use crate::web_utils::html_parser::HtmlParser;


#[function_component(About)]
pub fn about() -> Html {
    let web_content = WebContent::new();
    let about = web_content.get_about_me();
    let html_parser = HtmlParser{};
    html! {
        <section class="section about" id="about">
            <div class="container">
                <div class="section-box">
                    <div class="about-title">
                        <h3 class="section-title">
                            <span class="n-section-title">{"01."}</span> { &about.title }
                        </h3>
                    </div>
                    <div class="row">
                        <div class="col-12 col-md-6 mb-4 mb-md-0">
                            <div class="about-description">
                                { html_parser.parse_string_vector_to_inner_html(&about.paragraphs, "") }
                                <ul class="skills-list">
                                    { for about.skills.iter().map(|skill| html! {
                                    <li class="skill-element"><span class="underline">{skill}</span></li>
                                    })}
                                </ul>
                            </div>
                        </div>
                        <div class="col-12 col-md-6 mt-4 mt-md-0 text-center">
                            <div class="about-img-container">
                                <img class="rounded" width="300" height="300" src="assets/images/me.jpg" alt="Manuel R. Schere"/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
        }
}
