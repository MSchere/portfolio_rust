use yew::prelude::*;
use yew_bootstrap::util::{include_cdn_icons, include_inline, include_cdn_js};

use crate::components::header::Header;
use crate::components::banner::Banner;
use crate::components::about::About;
use crate::components::jobs::Jobs;
use crate::components::projects::Projects;
use crate::components::other_projects::OtherProjects;
use crate::components::contact::Contact;
use crate::components::footer::Footer;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            {include_inline()}
            {include_cdn_icons()}
            <Header />
            <Banner />
            <About />
            <Jobs />
            <Projects />
            <OtherProjects />
            <Contact />
            <Footer />
            {include_cdn_js()}
        </div>
    }
}