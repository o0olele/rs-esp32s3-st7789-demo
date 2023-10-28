use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{mono_font::{MonoTextStyle, ascii::FONT_10X20}, pixelcolor::Rgb565, text::Text, prelude::*, Drawable};
use esp_idf_svc::hal::{spi, gpio, prelude::*};
use anyhow::Result;

fn main() -> Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    test_lcd(
        peripherals.spi2,
        pins.gpio13.into(),
        pins.gpio15.into(),
        pins.gpio16.into(),
        pins.gpio18.into(),
        pins.gpio17.into(),
        pins.gpio14.into(),
    )?;

    Ok(())
}

fn test_lcd(
    spi: spi::SPI2,
    backlight: gpio::AnyOutputPin,
    dc: gpio::AnyOutputPin,
    rst: gpio::AnyOutputPin,
    sclk: gpio::AnyOutputPin,
    sdo: gpio::AnyOutputPin,
    cs: gpio::AnyOutputPin,
) -> Result<()> {
    let driver = spi::SpiDeviceDriver::new_single(
        spi,
        sclk,
        sdo,
        Option::<gpio::AnyIOPin>::None,
        Some(cs),
        &spi::SpiDriverConfig::new().dma(spi::Dma::Disabled),
        &spi::SpiConfig::new()
            .baudrate(20_000_000.into())
            .data_mode(esp_idf_svc::hal::spi::config::MODE_0),
    )?;

    let dc = gpio::PinDriver::output(dc)?;
    let rst = gpio::PinDriver::output(rst)?;
    let mut backlight = gpio::PinDriver::output(backlight)?;

    let di = SPIInterfaceNoCS::new(driver, dc);

    let mut delay = esp_idf_svc::hal::delay::Ets;
    let mut display = mipidsi::Builder::st7789(di)
        .with_display_size(240, 280)
        .with_orientation(mipidsi::Orientation::Landscape(true))
        .init(&mut delay, Some(rst))
        .unwrap();

    // Text
    let text = "Hello World ^_^;";
    let text_x = 120;
    let text_y = 280 / 2;
    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);

    // Alternating color
    let colors = [Rgb565::RED, Rgb565::GREEN, Rgb565::BLUE];

    // Clear the display initially
    display.clear(colors[0]).unwrap();

    // Turn on backlight
    let _ = backlight.set_high();

    // Draw text
    Text::new(text, Point::new(text_x, text_y), text_style)
        .draw(&mut display)
        .unwrap();
    println!("Draw Hello from Rust!");
    // text_x = if right.x <= 0 { 240 } else { text_x - char_w };

    // Turn off backlight and clear the display
    // let _ = backlight.set_low();
    // display.clear(Rgb565::BLACK).unwrap();

    Ok(())
}