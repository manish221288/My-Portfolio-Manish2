use leptos::prelude::*;
use crate::data::get_expertise;

#[component]
pub fn Expertise() -> impl IntoView {
    let expertise = get_expertise();

    view! {
        <section id="expertise" class="section visible">
            <div class="container">
                <h2 class="section-title">"Target Expertise & Research Focus"</h2>
                <p style="text-align: center; max-width: 800px; margin: 0 auto 3rem auto; color: var(--text-muted); font-size: 1.1rem;">
                    "Our Molecular Design Team has deep expertise in complex mechanisms, from membrane transporters to RNA regulation."
                </p>

                <div class="project-grid">
                    {expertise.into_iter().map(|card| {
                        view! {
                            <div class="card" style=format!("border-top: 4px solid {};", card.color)>
                                <h3 style=format!("color: {}; margin-bottom: 1rem; font-size: 1.4rem;", card.color)>{card.title}</h3>
                                <p style="color: var(--text-muted);">{card.description}</p>
                                <ul style="list-style: none; margin-top: 1rem;">
                                    {card.items.into_iter().map(|item| {
                                        view! {
                                            <li style="color: var(--text-muted); margin-bottom: 0.5rem;">"• " {item}</li>
                                        }
                                    }).collect_view()}
                                </ul>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </section>
    }
}
