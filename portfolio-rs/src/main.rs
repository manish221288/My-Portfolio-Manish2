mod components;
mod data;

use leptos::prelude::*;
use components::nav::Nav;
use components::hero::Hero;
use components::about::About;
use components::experience::Experience;
use components::expertise::Expertise;
use components::publications::Publications;
use components::skills::Skills;
use components::footer::Footer;

#[component]
fn App() -> impl IntoView {
    view! {
        <Nav />
        <Hero />
        <main>
            <div class="container">
                <About />
                <Experience />
                <Expertise />
                <Publications />
                <Skills />
            </div>
        </main>
        <Footer />
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
