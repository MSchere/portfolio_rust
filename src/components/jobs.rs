use yew::prelude::*;
use crate::web_utils::web_content::WebContent;
use crate::web_utils::html_parser::HtmlParser;

#[function_component(Jobs)]
pub fn jobs() -> Html {
    let web_content = WebContent::new();
    let html_parser = HtmlParser{};
    let experience = web_content.get_experience();
    let job_index = use_state(|| 0);
    
    html! {
        <section class="section jobs" id="jobs">
            <div class="container">
                <div class="section-box">
                    <div class="about-title mb-5">
                        <h3 class="e-font section-title">
                            <span class="code-font n-section-title">{"02."}</span> {&experience.title}
                        </h3>
                    </div>
                    <div class="jobs-tabs">
                        <ul class="nav-tabs jobs-tabs">
                            { for experience.jobs.iter().enumerate().map(|(index, job)| {
                                let onclick = {
                                    let job_index = job_index.clone();
                                    Callback::from(move |_| job_index.set(index))
                                };
                                html! {
                                    <li class="job-tab" {onclick}>
                                        <a>{&job.tab}</a>
                                    </li>
                                }
                            })}
                        </ul>
                        <div class="job-content">
                            <h4 class="title-tab-content">{&experience.jobs[*job_index].title}{" | "}<span class="company-tab-content">{&experience.jobs[*job_index].tab}</span></h4>
                            <h5 class="job-time">{&experience.jobs[*job_index].date}</h5>
                            { html_parser.parse_string_vector_to_inner_html(&experience.jobs[*job_index].description, "job-description") }
                            <span></span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
