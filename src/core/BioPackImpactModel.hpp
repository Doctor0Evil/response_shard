#pragma once
#include <cmath>
#include <stdexcept>

struct BioPackNodeState {
    double M_baseline_kg;   // baseline plastic mass per tray
    double M_biopack_kg;    // actual biopack mass per tray
    double trays_per_day;   // throughput
    double dt_days;         // time window
};

struct BioPackImpactConfig {
    double hazard_weight;   // w_pack, risk weight vs conventional plastic
    double karma_per_kg;    // Karma units per kg plastic avoided
};

struct BioPackImpactResult {
    double mass_avoided_kg;
    double node_impact_K;
};

inline BioPackImpactResult computeBioPackImpact(const BioPackNodeState& s,
                                                const BioPackImpactConfig& cfg) {
    if (s.dt_days <= 0.0 || s.trays_per_day <= 0.0) {
        throw std::invalid_argument("Invalid time window or throughput.");
    }
    if (s.M_baseline_kg < s.M_biopack_kg) {
        // No mass benefit; clamp at zero
        return {0.0, 0.0};
    }

    const double deltaM_kg_per_tray = s.M_baseline_kg - s.M_biopack_kg;
    const double mass_avoided_kg =
        deltaM_kg_per_tray * s.trays_per_day * s.dt_days;

    const double risk_unit = deltaM_kg_per_tray / s.M_baseline_kg;
    const double K_node = cfg.hazard_weight * risk_unit *
                          mass_avoided_kg * cfg.karma_per_kg;

    return {mass_avoided_kg, K_node};
}
