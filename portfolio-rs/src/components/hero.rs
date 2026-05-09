use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <header>
            <div class="container">
                <div class="hero-content">
                    <h1>"Dr. Manish " <span>"Kesherwani"</span></h1>
                    <div class="subtitle">"Scientific Manager & Lead Computational Chemist"</div>
                    <p>"8+ Years Professional Experience | R&D Manager at Nuvo AI"</p>
                </div>
            </div>
        </header>
    }
}
