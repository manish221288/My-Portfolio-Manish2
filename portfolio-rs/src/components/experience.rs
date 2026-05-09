use leptos::prelude::*;

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <section id="experience" class="section visible">
            <h2 class="section-title">"Professional Experience"</h2>
            <div class="timeline">
                <div class="timeline-item">
                    <h3>"R&D Manager"</h3>
                    <p><strong style="color: var(--text-dark);">"Nuvo AI (Meril Lifescience India)"</strong> " | March 2025 - Present"</p>
                    <p>"Providing strategic oversight on AI-based small molecule and antibody programs. Fostering a culture of innovation, mentoring scientific teams, and integrating AI/ML workflows into classical computational chemistry pipelines to execute drug design efficiently."</p>
                </div>
                <div class="timeline-item">
                    <h3>"Post-Doctoral Researcher"</h3>
                    <p><strong style="color: var(--text-dark);">"Institute of Transformative Biomolecules (ITbM), Nagoya University, Japan"</strong> " | 2019-2025"</p>
                    <p>"Pioneered integrative pipelines coupling protein-ligand interaction fingerprints with unsupervised ML for ultra-large virtual screening. Executed generative SBDD and protein design pipelines, and quantified solvation thermodynamics to drive hit-to-lead execution."</p>
                </div>
                <div class="timeline-item">
                    <h3>"Ph.D. Researcher"</h3>
                    <p><strong style="color: var(--text-dark);">"CAS in Crystallography & Biophysics, University of Madras"</strong> " | 2013-2019"</p>
                    <p>"Thesis: 'Molecular modelling and Molecular dynamics simulations of some biological macromolecules'"</p>
                </div>
                <div class="timeline-item">
                    <h3>"Researcher"</h3>
                    <p><strong style="color: var(--text-dark);">"Indo-Japan Collaborative Project"</strong> " | 2011-2013"</p>
                    <p>"Research at CAS in Crystallography & Biophysics, University of Madras"</p>
                </div>
            </div>
        </section>
    }
}
