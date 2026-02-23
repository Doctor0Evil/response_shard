// Core ecosafety types are assumed to live in a shared crate, e.g. `ecosafety_core`.
// These correspond to the RiskCoord, CorridorBands, Residual, and CorridorDecision
// structures and contracts described in the ecosafety spine.[file:1][file:9]

use ecosafety_core::{
    KerScore,
    KerBandTarget,
    KerGate,
    KerMeta,
    CorridorBands,
    Residual,
};

/// Enumerates the eight 2026 ecosafety research lines.[file:9]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResearchLineId {
    EcosafetyGrammarCore,
    PhoenixMarCyboquaticModules,
    EcotechnologyWetlandsBiofilms,
    BiodegradableSoftRoboticNodes,
    CircularHardwareRefurbishment,
    FlowvacBiodegradableSubstrates,
    AirGlobeExhaustKernels,
    PhoenixSitingLogisticsSoulsafety,
}

/// Per-line static K/E/R scores (research-only band) plus target band.[file:9]
#[derive(Debug, Clone)]
pub struct ResearchLineKer {
    pub id: ResearchLineId,
    pub name: &'static str,
    pub ker_current: KerScore,
    pub ker_target: KerScore,
}

/// Container for all eight Phoenix research lines for 2026.[file:9]
#[derive(Debug, Clone)]
pub struct PhoenixResearchLines2026v1 {
    pub epoch: &'static str,
    pub meta: KerMeta,
    pub lines: Vec<ResearchLineKer>,
}

impl PhoenixResearchLines2026v1 {
    /// Construct the canonical 2026 research-lines set (research-only lane). [file:9]
    pub fn new() -> Self {
        use ResearchLineId::*;

        // Shared meta and gates for 2026 research-only work.[file:1][file:9]
        let meta = KerMeta {
            epoch: "2026_research_only",
            ker_band_target: KerBandTarget {
                // Grammar + MAR + AirGlobe aspirations.[file:9]
                k_nominal: 0.95,
                e_nominal: 0.93,
                r_nominal: 0.10,
            },
            ker_gate_production: KerGate {
                // Hard gate for future production lanes (not yet applied here).[file:1]
                k_min: 0.90,
                e_min: 0.90,
                r_max: 0.13,
            },
        };

        let lines = vec![
            ResearchLineKer {
                id: EcosafetyGrammarCore,
                name: "Ecosafety grammar core (rx, Vt, corridorpresent, safestep)",
                ker_current: KerScore { k: 0.94, e: 0.90, r: 0.12 },
                ker_target:  KerScore { k: 0.97, e: 0.93, r: 0.10 },
            },
            ResearchLineKer {
                id: PhoenixMarCyboquaticModules,
                name: "Phoenix-class MAR cyboquatic modules",
                ker_current: KerScore { k: 0.93, e: 0.92, r: 0.14 },
                ker_target:  KerScore { k: 0.95, e: 0.93, r: 0.10 },
            },
            ResearchLineKer {
                id: EcotechnologyWetlandsBiofilms,
                name: "Ecotechnology wetlands / biofilms coupling",
                ker_current: KerScore { k: 0.90, e: 0.91, r: 0.15 },
                ker_target:  KerScore { k: 0.94, e: 0.92, r: 0.12 },
            },
            ResearchLineKer {
                id: BiodegradableSoftRoboticNodes,
                name: "Biodegradable soft-robotic cyboquatic nodes",
                ker_current: KerScore { k: 0.88, e: 0.87, r: 0.18 },
                ker_target:  KerScore { k: 0.93, e: 0.89, r: 0.13 },
            },
            ResearchLineKer {
                id: CircularHardwareRefurbishment,
                name: "Circular hardware & refurbishment loops",
                ker_current: KerScore { k: 0.89, e: 0.88, r: 0.16 },
                ker_target:  KerScore { k: 0.93, e: 0.90, r: 0.13 },
            },
            ResearchLineKer {
                id: FlowvacBiodegradableSubstrates,
                name: "Flowvac biodegradable substrate synthesis",
                ker_current: KerScore { k: 0.88, e: 0.87, r: 0.18 },
                ker_target:  KerScore { k: 0.92, e: 0.89, r: 0.14 },
            },
            ResearchLineKer {
                id: AirGlobeExhaustKernels,
                name: "AirGlobe / exhaust ecosafety kernels (ESPD, Dt, KTSF)",
                ker_current: KerScore { k: 0.96, e: 0.92, r: 0.11 },
                ker_target:  KerScore { k: 0.98, e: 0.94, r: 0.09 },
            },
            ResearchLineKer {
                id: PhoenixSitingLogisticsSoulsafety,
                name: "Phoenix siting, logistics & soulsafety planning kernels",
                ker_current: KerScore { k: 0.93, e: 0.91, r: 0.13 },
                ker_target:  KerScore { k: 0.96, e: 0.93, r: 0.11 },
            },
        ];

        PhoenixResearchLines2026v1 {
            epoch: "2026",
            meta,
            lines,
        }
    }

    /// Filter the lines that already satisfy the production K/E/R gate.[file:1]
    pub fn lines_meeting_production_gate(&self) -> Vec<&ResearchLineKer> {
        self.lines
            .iter()
            .filter(|line| {
                let g = &self.meta.ker_gate_production;
                line.ker_current.k >= g.k_min
                    && line.ker_current.e >= g.e_min
                    && line.ker_current.r <= g.r_max
            })
            .collect()
    }
}
