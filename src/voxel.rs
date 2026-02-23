use crate::types::{CorridorBands, RiskCoord, Residual};

#[derive(Clone, Copy, Debug)]
pub enum LifeForm {
    Honeybee,
    Aquatic,
    Human,
    Pet,
    Insect,
    None,
}

#[derive(Clone, Copy, Debug)]
pub struct SafetyEnvelope {
    pub lifeform: LifeForm,
    pub max_tdi: f32,
    pub min_mbi: f32,
    pub max_eis: f32,
    pub max_rad_index: f32,
    pub max_residence: u32,
}

impl SafetyEnvelope {
    pub fn is_safe(
        &self,
        tdi: f32,
        mbi: f32,
        eis: f32,
        rad: f32,
        residences: u32,
    ) -> bool {
        tdi <= self.max_tdi
            && mbi >= self.min_mbi
            && eis <= self.max_eis
            && rad <= self.max_rad_index
            && residences <= self.max_residence
    }
}

#[derive(Clone, Debug)]
pub struct Lifeforce5DVoxel {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub tdi: f32,
    pub mbi: f32,
    pub eis: f32,
    pub rad_index: f32,
    pub residence: u32,
    pub envelope: SafetyEnvelope,
}

impl Lifeforce5DVoxel {
    pub fn to_risk_coords(
        &self,
        bands_tdi: &CorridorBands,
        bands_mbi: &CorridorBands,
        bands_eis: &CorridorBands,
        bands_rad: &CorridorBands,
    ) -> Residual {
        fn norm(x: f32, b: &CorridorBands, good_is_low: bool) -> f32 {
            if good_is_low {
                if x <= b.safe {
                    0.0
                } else if x >= b.hard {
                    1.0
                } else {
                    (x - b.safe) / (b.hard - b.safe)
                }
            } else {
                if x >= b.safe {
                    0.0
                } else if x <= b.hard {
                    1.0
                } else {
                    (b.safe - x) / (b.safe - b.hard)
                }
            }
        }

        let mut rx = heapless::Vec::<RiskCoord, 16>::new();

        rx.push(RiskCoord {
            value: norm(self.tdi, bands_tdi, true),
            bands: bands_tdi.clone(),
            sigma: 0.0,
        })
        .ok();
        rx.push(RiskCoord {
            value: norm(self.mbi, bands_mbi, false),
            bands: bands_mbi.clone(),
            sigma: 0.0,
        })
        .ok();
        rx.push(RiskCoord {
            value: norm(self.eis, bands_eis, true),
            bands: bands_eis.clone(),
            sigma: 0.0,
        })
        .ok();
        rx.push(RiskCoord {
            value: norm(self.rad_index, bands_rad, true),
            bands: bands_rad.clone(),
            sigma: 0.0,
        })
        .ok();

        let mut res = Residual { vt: 0.0, rx };
        res.recompute();
        res
    }
}
