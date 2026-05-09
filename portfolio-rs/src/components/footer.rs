use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <div class="container">
                <p>"© 2026 Dr. Manish Kesherwani. All rights reserved."</p>
                <p style="margin-top: 0.5rem; font-size: 0.9rem; color: var(--text-muted);">
                    "Built with " <span style="color: #dea584;">"Rust"</span> " & " <span style="color: var(--accent-cyan);">"WebAssembly"</span>
                </p>
            </div>
        </footer>
    }
}
