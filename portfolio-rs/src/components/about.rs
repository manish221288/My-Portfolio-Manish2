use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="section visible">
            <h2 class="section-title">"About Me"</h2>
            <div class="about-grid">
                <div class="card">
                    <p>"I am an accomplished Computational Biophysicist and Scientific Manager with a proven track record of applying AI-first approaches to provide technical and strategic direction in drug design programs. I have over 8 years of combined professional and post-doctoral experience translating fundamental structural-dynamic knowledge into AI-accelerated candidate molecules."</p>
                    <br />
                    <p>"My expertise lies in bridging physics-based simulation and cutting-edge machine learning (ML/DL) workflows to drive small molecule programs from hit finding through lead optimization and candidate selection."</p>
                    <br />
                    <p>"Currently, I lead cross-functional R&D efforts at Nuvo AI (Meril Lifescience India), establishing generative AI-driven SBDD pipelines, mentoring high-performing teams, and championing end-to-end drug discovery programs against oncologic, infectious, and rare diseases."</p>
                </div>
                <div class="contact-info">
                    <h3>"Contact Information"</h3>
                    <div class="contact-item">
                        <span>"📧"</span>
                        <span>"manish221288@gmail.com"</span>
                    </div>
                    <div class="contact-item">
                        <span>"📱"</span>
                        <span>"+91-9841135618"</span>
                    </div>
                    <div class="contact-item">
                        <span>"🎓"</span>
                        <span>"Google Scholar: h-index 12"</span>
                    </div>
                    <div class="contact-item">
                        <span>"📄"</span>
                        <span>"24 Publications, 507 Citations"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
