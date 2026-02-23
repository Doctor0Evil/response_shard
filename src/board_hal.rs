use embedded_hal::adc::OneShot;
use embedded_hal::digital::v2::OutputPin;

pub trait NanoswarmBoard {
    type Adc;
    type DutyPin: OutputPin;

    fn adc(&mut self) -> &mut Self::Adc;
    fn duty_pin(&mut self) -> &mut Self::DutyPin;

    fn read_tdi(&mut self) -> f32;
    fn read_mbi(&mut self) -> f32;
    fn read_eis(&mut self) -> f32;
    fn read_rad_index(&mut self) -> f32;

    fn apply_duty(&mut self, duty: f32);
}
