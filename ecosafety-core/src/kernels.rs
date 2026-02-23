pub fn to_r_linear(x: f64, bands: &CorridorBands) -> RiskCoord {
    assert!(bands.safe < bands.hard && bands.hard <= 1.0);
    let r = if x <= bands.safe {
        0.0
    } else if x >= bands.hard {
        1.0
    } else {
        (x - bands.safe) / (bands.hard - bands.safe)
    };
    RiskCoord { r, sigma: 0.0, bands: *bands }
}
