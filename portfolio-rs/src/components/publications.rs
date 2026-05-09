use leptos::prelude::*;
use crate::data::get_publications;

#[component]
pub fn Publications() -> impl IntoView {
    let pubs = get_publications();

    view! {
        <section id="publications" class="section visible">
            <div class="container">
                <h2 class="section-title">"Selected Publications"</h2>
                <div class="card">
                    {pubs.into_iter().map(|p| {
                        view! {
                            <div class="publication">
                                <h4>{p.title}</h4>
                                <p class="journal">{p.journal} <span class="impact-factor">"IF: " {p.impact_factor}</span></p>
                                <p style="color: var(--text-muted); margin-top: 0.5rem;">{p.year} " | " {p.role}</p>
                                
                                <div style="margin-top: 1.5rem; background: rgba(0,0,0,0.2); padding: 1rem; border-radius: 8px; border: 1px solid rgba(255,255,255,0.05);">
                                    {p.abstract_img.clone().map(|img| {
                                        let img_url = img.clone();
                                        view! {
                                            <>
                                                <p style="color: var(--accent-cyan); font-size: 0.9rem; margin-bottom: 0.5rem; font-weight: 600;">"Graphical Abstract"</p>
                                                <a href=img_url target="_blank" title="Click to view full size">
                                                    <img src=img alt="Graphical Abstract" class="abstract-thumb" />
                                                </a>
                                            </>
                                        }
                                    })}
                                    <a href=p.doi.clone() target="_blank" style="display: inline-block; padding: 0.5rem 1rem; background: rgba(0, 212, 255, 0.1); color: var(--accent-cyan); border: 1px solid var(--accent-cyan); border-radius: 5px; font-size: 0.9rem; transition: all 0.3s ease;">"Access Paper (DOI)"</a>
                                </div>
                            </div>
                        }
                    }).collect_view()}

                    <div class="stat-highlight">
                        <div><strong>"Total: "</strong> <span>"24 publications"</span></div>
                        <div><strong>"Citations: "</strong> <span>"507"</span></div>
                        <div><strong>"h-index: "</strong> <span>"12"</span></div>
                    </div>
                </div>
            </div>
        </section>
    }
}
