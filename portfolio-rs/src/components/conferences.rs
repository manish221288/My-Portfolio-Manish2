use leptos::prelude::*;

#[component]
pub fn Conferences() -> impl IntoView {
    view! {
        <section id="conferences" class="section visible">
            <div class="container">
                <h2 class="section-title">"Global Reach & Scientific Presentations"</h2>
                <p style="text-align: center; max-width: 800px; margin: 0 auto 3rem auto; color: var(--text-muted); font-size: 1.1rem;">
                    "Actively contributing to the global scientific community through major international conferences and symposiums."
                </p>

                <div class="project-grid">
                    <div class="card" style="background: rgba(28, 37, 65, 0.4); border-left: 4px solid var(--accent-cyan);">
                        <h3 style="color: #FFFFFF; font-size: 1.1rem; margin-bottom: 0.75rem;">"Biophysical Society of Japan"</h3>
                        <p style="font-size: 0.95rem; color: var(--text-muted);">
                            "Presented research on the molecular mechanisms underpinning the aggregation of 1-Halo POR dyes within membranes using advanced MD simulations at IUPAB2024, Kyoto International Conference Center."
                        </p>
                    </div>

                    <div class="card" style="background: rgba(28, 37, 65, 0.4); border-left: 4px solid var(--accent-purple);">
                        <h3 style="color: #FFFFFF; font-size: 1.1rem; margin-bottom: 0.75rem;">"Asian Crystallographic Association"</h3>
                        <p style="font-size: 0.95rem; color: var(--text-muted);">
                            "Presented in-silico molecular modeling and simulation studies on novel natural inhibitors for the multidrug and toxic compound extrusion transporter NorM at Science City, Kolkata."
                        </p>
                    </div>

                    <div class="card" style="background: rgba(28, 37, 65, 0.4); border-left: 4px solid #FFD700;">
                        <h3 style="color: #FFFFFF; font-size: 1.1rem; margin-bottom: 0.75rem;">"CSP-ITbM Joint Workshop"</h3>
                        <p style="font-size: 0.95rem; color: var(--text-muted);">
                            "Presented findings on the identification of novel leads against CRY proteins for therapeutics in Circadian Disorders at RIKEN Wako, Japan."
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
