use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::web_utils::web_content::WebContent;

#[function_component(Projects)]
pub fn projects() -> Html {
    let web_content = WebContent::new();
    let featured_projects = web_content.get_featured_projects();
    html! {
        <section class="section projects" id="projects">
            <div class="container">
                <div class="section-box">
                    <div>
                        <h3 class="section-title">
                            <span class="n-section-title">{"03."}</span> { &featured_projects.title }
                        </h3>
                    </div>
                    { for featured_projects.projects.iter().enumerate().map(|(index, project)| {
                        let mut project_class = "project-info-left";
                        let mut text_class = "text-left";
                        let mut justify_content_class = "justify-content-start";
                        let mut img_class_1 = "d-none";
                        let mut img_class_2 = "";
                        if index % 2 == 0 {
                            project_class = "project-info-right";
                            text_class = "text-right";
                            justify_content_class = "justify-content-end";
                            img_class_1 = "";
                            img_class_2 = "d-none";
                        }
                        html! {
                            <div class="d-flex project-container">
                                <div class={format!("{} {}", img_class_1, "img_carousel")} >
                                    <div class="img-feature-project-box">
                                        <div class="img-container" style="width: auto;">
                                            <img class="img-feature-project rounded" src={project.img.clone()} alt={project.img.clone()} width="600px" />
                                        </div>
                                    </div>
                                </div>
                                <div class={format!("{} {}", project_class, "h-100")} style="background-image" url={project.img.clone()}>
                                    <div class="project-box">
                                        <h4 class={format!("{} {}", text_class, "feature-project")}>{ &featured_projects.label }</h4>
                                        <h5 class={format!("{} {}", text_class, "project-name")}>{&project.title}</h5>
                                        <div class="project-description-box">
                                            { project.description.clone() }
                                        </div>
                                        <ul class={format!("{} {}", justify_content_class, "project-skills")}>
                                            { for project.technologies.iter().map(|technology| html! {
                                                <li>
                                                    <span class="underline technology ml-3">{technology}</span>
                                                </li>
                                            })}
                                        </ul>
                                        <div class={format!("{} {}", text_class, "project-links")}>
                                            <a style="color: inherit" href={project.gh_link.clone()} target="_blank"><Icon class="ml-3 icon" icon_id={IconId::BootstrapGithub}/></a>
                                            <a  style="color: inherit" href={project.demo_link.clone()}  target="_blank"><Icon class="ml-3 icon" icon_id={IconId::OcticonsLinkExternal16}/></a>
                                        </div>
                                    </div>
                                </div>
                                <div class={format!("{} {}", img_class_2, "img_carousel")} >
                                    <div class="img-feature-project-box">
                                        <div class="img-container" style="width: auto;">
                                            <img class="img-feature-project rounded" src={project.img.clone()} alt={project.img.clone()} width="600px" />
                                        </div>
                                    </div>
                                </div>
                            </div>
                     }})}
                </div>
            </div>
        </section>
    }
}
