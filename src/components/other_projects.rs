use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::web_utils::web_content::WebContent;

#[function_component(OtherProjects)]
pub fn more_projects() -> Html {
    let web_content = WebContent::new();
    let more_projects = web_content.get_other_projects();
    html! {
        <section class="section more-projects">
            <div class="container">
                <div class="section-box">
                    <div class="">
                        <h3 class="section-title">
                            <span class="n-section-title">{"04."}</span> {&more_projects.title}
                        </h3>
                    </div>
                    <div class="mt-5">
                        <div class="row p-0">
                            { for more_projects.projects.iter().map(|project| html! {
                            <div class="col-12 col-md-6 col-lg-6 col-xl-4 project-col">
                                <div class="more-project-box">
                                    <div class="row w-100 text-left m-0 p-0">
                                        <div class="col-6 p-0">
                                            <img src="assets/images/folder.svg" width="40"/>
                                        </div>
                                        <div class="col-6 d-flex p-0 justify-content-end">
                                            <a style="color: inherit" href={project.gh_link.clone()} target="_blank"><Icon class="ml-4 icon" icon_id={IconId::BootstrapGithub}/></a>
                                        </div>
                                        <h5 class="other-project-title mt-4">{&project.title}</h5>
                                        <p class="other-project-description">
                                            {project.description.clone()}
                                        </p>
                                    </div>
                                    <div>
                                        <ul class="more-projects-skills">
                                            { for project.technologies.iter().map(|technology| html! {
                                                <li><span class="underline">{technology}</span></li>
                                            })}
                                        </ul>
                                    </div>
                                </div>
                            </div>
                            })}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
