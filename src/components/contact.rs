use yew::prelude::*;
use crate::web_utils::web_content::WebContent;

#[function_component(Contact)]
pub fn contact() -> Html {
    let web_content = WebContent::new();
    let contact = web_content.get_contact();
    html! {
        <section class="section contact" id="contact">
            <div class="container">
                <div class="section-box">
                    <div class="mb-4">
                        <h3 class="contact-pre-title big-subtitle">{"05. "}{&contact.pretitle}</h3>
                        <h4 class="e-font contact-title big-title">
                            {&contact.title}
                        </h4>
                    </div>
                    <p>
                        {&contact.content}
                    </p>
                    <div class="mt-5">
                        <a href="mailto:manu_schere@proton.me" target="_blank" class="main-btn">
                            {&contact.btn}
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}