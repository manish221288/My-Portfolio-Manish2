use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            <div class="container">
                <div class="nav-container">
                    <ul class="nav-links">
                        <li><a href="#about" class="nav-link active">"About"</a></li>
                        <li><a href="#experience" class="nav-link">"Experience"</a></li>
                        <li><a href="#expertise" class="nav-link">"Expertise"</a></li>
                        <li><a href="#publications" class="nav-link">"Publications"</a></li>
                        <li><a href="#skills" class="nav-link">"Skills"</a></li>
                    </ul>
                </div>
            </div>
        </nav>
    }
}
