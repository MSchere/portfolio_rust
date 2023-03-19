mod components;
mod web_utils;
use crate::components::home::Home;

fn main() {
    yew::Renderer::<Home>::new().render();
}
