use embassy_rp::gpio::{Level, Output};
use embassy_rp::Peripherals;
use embassy_rp::peripherals::{SPI1, PIN_10, PIN_11, PIN_12};

pub struct Pins {
    // LED
    pub user_led: Output<'static>,
    // Display
    // -- SPI
    pub spi_no: SPI1,
    pub spi_sck: PIN_10,
    pub spi_tx: PIN_11,
    pub spi_rx: PIN_12, // Not used
    // -- control
    pub lcd_dc: Output<'static>,    // Data/Command
    pub lcd_cs: Output<'static>,    // Chip Select
    pub lcd_bl: Output<'static>,    // Backlight
    pub lcd_rst: Output<'static>,   // Reset
}

pub fn init(p: Peripherals) -> Pins {
    Pins {
        // LED
        user_led: Output::new(p.PIN_25, Level::Low),
        // Display
        // -- SPI
        spi_no: p.SPI1,
        spi_sck: p.PIN_10,
        spi_tx: p.PIN_11,
        spi_rx: p.PIN_12,   // Not used
        // -- control
        lcd_dc: Output::new(p.PIN_13, Level::Low),      // Data/Command
        lcd_cs: Output::new(p.PIN_14, Level::High),     // Chip Select
        lcd_bl: Output::new(p.PIN_15, Level::High),     // Backlight
        lcd_rst: Output::new(p.PIN_16, Level::High),    // Reset
    }
}