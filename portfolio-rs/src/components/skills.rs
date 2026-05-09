use leptos::prelude::*;

#[component]
pub fn Skills() -> impl IntoView {
    view! {
        <section id="skills" class="section visible">
            <div class="container">
                <h2 class="section-title">"Core Competencies"</h2>
                <div class="skills-grid">
                    <div class="card">
                        <div class="skill-category">
                            <h3>"Computational Strategy & Leadership"</h3>
                            <span class="skill-item">"Drug Discovery Pipeline Leadership"</span>
                            <span class="skill-item">"Mentorship & Team Building"</span>
                            <span class="skill-item">"Cross-functional Collaboration"</span>
                        </div>
                    </div>
                    <div class="card">
                        <div class="skill-category">
                            <h3>"Advanced AI & Machine Learning"</h3>
                            <span class="skill-item">"Generative AI (Diffusion, LLMs)"</span>
                            <span class="skill-item">"Geometric Deep Learning"</span>
                            <span class="skill-item">"Predictive Analytics"</span>
                        </div>
                    </div>
                    <div class="card">
                        <div class="skill-category">
                            <h3>"Molecular Modeling & Dynamics"</h3>
                            <span class="skill-item">"Classical & MLP MD Simulations"</span>
                            <span class="skill-item">"Solute transport in membrane bilayer"</span>
                            <span class="skill-item">"GROMACS, AMBER, CHARMM, OpenMM"</span>
                            <span class="skill-item">"Markov State Modeling"</span>
                            <span class="skill-item">"Free Energy Calculations"</span>
                        </div>
                    </div>
                    <div class="card">
                        <div class="skill-category">
                            <h3>"Physics-based CADD"</h3>
                            <span class="skill-item">"Structure-based Drug Design"</span>
                            <span class="skill-item">"Pharmacophore modeling"</span>
                            <span class="skill-item">"Lead optimization"</span>
                            <span class="skill-item">"ADMET"</span>
                        </div>
                    </div>
                    <div class="card">
                        <div class="skill-category">
                            <h3>"Structural Modeling"</h3>
                            <span class="skill-item">"Homology Modeling"</span>
                            <span class="skill-item">"Integrated Modeling (Cryo-EM, SAXS, NMR, HDX-MS)"</span>
                            <span class="skill-item">"Antibody Design"</span>
                            <span class="skill-item">"PPI Inhibitors"</span>
                            <span class="skill-item">"AlphaFold & Rosetta"</span>
                        </div>
                    </div>
                    <div class="card">
                        <div class="skill-category">
                            <h3>"Scripting & Cheminformatics"</h3>
                            <span class="skill-item">"Python (RDKit, PyTorch)"</span>
                            <span class="skill-item">"Rust & WebAssembly"</span>
                            <span class="skill-item">"Pipeline Automation"</span>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
