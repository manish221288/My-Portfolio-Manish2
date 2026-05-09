use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Publication {
    pub title: String,
    pub journal: String,
    pub impact_factor: String,
    pub year: String,
    pub role: String,
    pub doi: String,
    pub abstract_img: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpertiseCard {
    pub title: String,
    pub description: String,
    pub items: Vec<String>,
    pub color: String,
}

pub fn get_expertise() -> Vec<ExpertiseCard> {
    vec![
        ExpertiseCard {
            title: "Membrane Proteins & Transporters".to_string(),
            description: "Elucidating dynamic coupling and substrate binding mechanisms in challenging targets.".to_string(),
            items: vec![
                "Multidrug resistance NorM transporter".to_string(),
                "Membrane integrated phosphatase PgpB".to_string(),
            ],
            color: "var(--accent-purple)".to_string(),
        },
        ExpertiseCard {
            title: "Circadian Clock Regulation".to_string(),
            description: "Designing and optimizing specific modulators for sleep/wake cycles and metabolic syndromes.".to_string(),
            items: vec![
                "Cryptochrome (CRY1, CRY2) Activators & Antagonists".to_string(),
                "Anti-glioblastoma efficacy modeling".to_string(),
            ],
            color: "var(--accent-cyan)".to_string(),
        },
        ExpertiseCard {
            title: "Complex Enzymes & Riboswitches".to_string(),
            description: "Applying Markov State Modeling and hit-finding to understand allostery and regulation in diverse metabolic targets.".to_string(),
            items: vec![
                "Novel NAMPT inhibitor identification".to_string(),
                "TPP/thiM Riboswitch conformational dynamics".to_string(),
                "Fumarate hydratase and UBIAD1 prenyltransferase".to_string(),
            ],
            color: "#FFD700".to_string(),
        },
        ExpertiseCard {
            title: "Peptide Modeling & Biologics".to_string(),
            description: "Advanced structural modeling of large biomolecules, host-pathogen interactions, and serum proteins.".to_string(),
            items: vec![
                "Dengue virus 2 protease inhibitors".to_string(),
                "Human serum albumin (HSA) interactions".to_string(),
            ],
            color: "#FF4D4D".to_string(),
        },
        ExpertiseCard {
            title: "Generative SBDD Pipelines".to_string(),
            description: "Championing end-to-end AI-first pipelines that successfully meet critical go/no-go milestones.".to_string(),
            items: vec![
                "Hit-to-Lead execution".to_string(),
                "Concept to wet-lab candidate nomination".to_string(),
            ],
            color: "var(--accent-cyan)".to_string(),
        },
        ExpertiseCard {
            title: "Solvation Thermodynamics".to_string(),
            description: "Quantifying solvation thermodynamics using inhomogeneous-fluid theory for lead optimization.".to_string(),
            items: vec![
                "Water network analysis".to_string(),
                "Binding affinity modulation".to_string(),
            ],
            color: "#FFD700".to_string(),
        },
    ]
}

pub fn get_publications() -> Vec<Publication> {
    vec![
        Publication {
            title: "Local peptide signalling induces stomatal closure under drought stress".to_string(),
            journal: "Nature Communications".to_string(),
            impact_factor: "15.7".to_string(),
            year: "2025".to_string(),
            role: "Co-author".to_string(),
            doi: "https://doi.org/10.1038/s41467-025-66392-6".to_string(),
            abstract_img: Some("Final/CLE5P.png".to_string()),
        },
        Publication {
            title: "Single-Cell Fluorescence Analysis of Lipid Droplet Compositional Dynamics during Triacylglycerol Catabolism".to_string(),
            journal: "Journal of the American Chemical Society".to_string(),
            impact_factor: "15.7".to_string(),
            year: "2025".to_string(),
            role: "Co-author".to_string(),
            doi: "https://doi.org/10.1021/jacs.5c11742".to_string(),
            abstract_img: None,
        },
        Publication {
            title: "Stereochemistry-Dependent Labeling of Organelles with a Near-Infrared-Emissive Phosphorus-Bridged Rhodamine Dye in Live-Cell Imaging".to_string(),
            journal: "Angewandte Chemie".to_string(),
            impact_factor: "16.6".to_string(),
            year: "2024".to_string(),
            role: "Co-author".to_string(),
            doi: "https://doi.org/10.1002/anie.202400711".to_string(),
            abstract_img: Some("Final/Aggregation_PORBT_dye.png".to_string()),
        },
        Publication {
            title: "CRY2 isoform selectivity of circadian clock modulator with anti-glioblastoma efficacy".to_string(),
            journal: "Proceedings of National Academy of Sciences".to_string(),
            impact_factor: "11.20".to_string(),
            year: "2022".to_string(),
            role: "Second author".to_string(),
            doi: "https://doi.org/10.1073/pnas.2203936119".to_string(),
            abstract_img: Some("Final/CRY2_SHP177.png".to_string()),
        },
        Publication {
            title: "Conformational Dynamics of thiM Riboswitch to understand Gene Regulation Mechanism using Markov State Modeling and Residual Fluctuation Network Approach".to_string(),
            journal: "Journal of Chemical Information and Modeling".to_string(),
            impact_factor: "3.76".to_string(),
            year: "2018".to_string(),
            role: "First author".to_string(),
            doi: "https://doi.org/10.1021/acs.jcim.8b00155".to_string(),
            abstract_img: Some("Final/TPP_riboswitch.png".to_string()),
        },
        Publication {
            title: "Molecular insights into substrate binding mechanism of undecaprenyl pyrophosphate with membrane integrated phosphatidyl glycerophosphate phosphatase B (PgpB) using molecular dynamics simulation approach".to_string(),
            journal: "Journal of Biomolecular Structure and Dynamics".to_string(),
            impact_factor: "3.12".to_string(),
            year: "2018".to_string(),
            role: "First author".to_string(),
            doi: "https://doi.org/10.1080/07391102.2018.1455502".to_string(),
            abstract_img: Some("Final/PgpB.png".to_string()),
        },
        Publication {
            title: "Identification of novel Nicotinamide Phosphoribosyltransferase (NAMPT) inhibitors using computational approaches".to_string(),
            journal: "Journal of Biomolecular Structure & Dynamics".to_string(),
            impact_factor: "3.12".to_string(),
            year: "2018".to_string(),
            role: "First author".to_string(),
            doi: "https://doi.org/10.1080/07391102.2017.1322004".to_string(),
            abstract_img: Some("Final/NAMPT.png".to_string()),
        },
        Publication {
            title: "Identification of novel natural inhibitor for NorM – a multidrug and toxic compound extrusion transporter – an insilico molecular modeling and simulation studies".to_string(),
            journal: "Journal of Biomolecular Structure & Dynamics".to_string(),
            impact_factor: "3.12".to_string(),
            year: "2017".to_string(),
            role: "First author".to_string(),
            doi: "https://doi.org/10.1080/07391102.2015.1132391".to_string(),
            abstract_img: Some("Final/NorM.png".to_string()),
        },
    ]
}
