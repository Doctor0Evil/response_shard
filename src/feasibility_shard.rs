use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FeasibilityShard {
    pub shardid: String,
    pub projectid: String,
    pub city: String,
    pub country: String,
    pub climate_zone: String,
    pub lat: f64,
    pub lon: f64,
    pub eval_year: i32,

    pub tech_feas_score: f64,
    pub ecoimpact_score: f64,
    pub economic_score: f64,
    pub risk_score: f64,
    pub socialurban_score: f64,
    pub knowledgefactor: f64,
    pub riskofharm: f64,

    pub capex_per_m_linear_eur: f64,
    pub opex_energy_kwh_m2y: f64,
    pub opex_maintenance_eur_y: f64,
    pub water_price_eur_m3: f64,
    pub wastewater_tariff_eur_m3: f64,

    pub payback_best_years: f64,
    pub payback_median_years: f64,
    pub payback_worst_years: f64,

    pub water_saving_frac: f64,
    pub hvac_energy_saving_frac: f64,
    pub nutrient_recovery_frac: f64,
    pub food_yield_kg_m2y: f64,
    pub bipv_generation_kwh_m2y: f64,

    pub greywater_flow_m3_d: f64,
    pub reuse_fraction: f64,
    pub mp_pollutant_corridor: f64,
    pub hb_rating: f64,
    pub oc_impact_safety: f64,

    pub legal_eu2020_741_ok: bool,
    pub legal_whg_abwv_ok: bool,
    pub legal_greywater_local_ok: bool,

    pub corridors_count: i32,
    pub gate_predicates_count: i32,
    pub invariants_verified_count: i32,

    pub espd_route: String,
    pub notes: String,
}
