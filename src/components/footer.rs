use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="footer">
            <div>
                <ul class="footer-left-bar d-none d-md-block">
                    <li>
                        <a href="https://t.me/scheren76" target="_blank">
                            <Icon class="icon" icon_id={IconId::BootstrapTelegram}/>
                        </a>
                    </li>
                    <li>
                        <a href="https://www.instagram.com/manu_rdsc" target="_blank">
                            <Icon class="icon" icon_id={IconId::BootstrapInstagram}/>
                        </a>
                    </li>
                    <li>
                        <a href="https://twitter.com/manu_schere" target="_blank">
                            <Icon class="icon" icon_id={IconId::BootstrapTwitter}/>
                        </a>
                    </li>
                    <li>
                        <a href="https://github.com/MSchere" target="_blank">
                            <Icon class="icon" icon_id={IconId::BootstrapGithub}/>
                        </a>
                    </li>
                    <li>
                        <a href="https://www.linkedin.com/in/manu-schere/" target="_blank">
                            <Icon class="icon" icon_id={IconId::BootstrapLinkedin}/>
                        </a>
                    </li>
                </ul>
                <div class="footer-right-bar d-none d-md-block">
                    <a href="mailto:manu_schere@proton.me" target="_blank">{"manu_schere@proton.me"}</a>
                </div>
            </div>

            <div class="footer-credits text-center" >
                <a href="https://github.com/MSchere/portfolio" target="_blank" rel="nofollow noopener noreferrer">
                    <div>{"Built with Yew.rs by Manuel R. Schere"}</div>
                </a>
                <a href="https://github.com/bchiang7/v4" target="_blank" rel="nofollow noopener noreferrer">
                    <div class="mt-2">{"Designed by Brittany Chiang"}</div>
                </a>
            </div>
        </div>
    }
}
