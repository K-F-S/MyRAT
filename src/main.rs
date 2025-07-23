#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Timer, Delay};
use embassy_rp::spi::{Spi, Config};
use embassy_rp::init as rp_init;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
    pixelcolor::Rgb565,
    image::{Image, ImageRaw},
};
use display_interface_spi::SPIInterface;
use st7789::{ST7789, Orientation};

use {defmt_rtt as _, panic_probe as _};

// pin setting
mod config {
    pub mod pins;
}
use config::pins::init as pin_init;

// 画像データを raw バイナリとして埋め込む
const MOUSE_IMAGE_RAW: &[u8] = include_bytes!("images/mouse_blue_100.raw");

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("main start");

    let p = rp_init(Default::default()); // Init pico
    let mut pins = pin_init(p);

    // Init SPI
    let spi = Spi::new_blocking(
        pins.spi_no,
        pins.spi_sck,
        pins.spi_tx,
        pins.spi_rx,
        Config::default(),
    );

    // LCDインターフェースを生成
    let di = SPIInterface::new(spi, pins.lcd_dc, pins.lcd_cs);

    // ST7789 ドライバを生成
    let mut display = ST7789::new(
        di,
        Some(pins.lcd_rst),
        Some(pins.lcd_bl),
        320,
        170,
    );
    let mut delay = Delay;
    display.init(&mut delay).unwrap();
    display.set_orientation(Orientation::Landscape).unwrap();   // 横向きに設定
    display.clear(Rgb565::BLACK).unwrap();  // 画面を黒でクリア

    // テキストを描画
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    // Text::new("Hello Mouse!", Point::new(100, 100), style)
    //     .draw(&mut display)
    //     .unwrap();

    // 画像をLCDに表示
    let img_raw = ImageRaw::<Rgb565>::new(MOUSE_IMAGE_RAW, 100);
    let img = Image::new(&img_raw, Point::new(50, 80));
    img.draw(&mut display).unwrap();

    loop {
        pins.user_led.set_high(); // 点灯
        Timer::after_secs(1).await; // 1秒待機
        pins.user_led.set_low(); // 消灯
        Timer::after_secs(1).await; // 1秒待機
    }
}
