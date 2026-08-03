use esp_idf_svc::hal::{
    delay::Delay,
    gpio::{self, PinDriver},
    units::MegaHertz,
    peripherals::Peripherals,
    spi::{
        self,
        config::{Config, Mode, Phase, Polarity},
        SpiDeviceDriver,
    },
};
use gc9a01::{prelude::*, Gc9a01, SPIDisplayInterface};
use std::time::Duration;

fn the_real_main() {
    let peripherals = Peripherals::take().unwrap();

    // we need quite a big stack size for buffered graphics so spawns a thread
    // with a bigger than default
    std::thread::Builder::new()
        .name("display".into())
        .stack_size(7000 + (160 * 1024) + 4096)
        .spawn(move || {
            let pins = peripherals.pins;
            let mut backlight = PinDriver::output(pins.gpio2).unwrap();
            backlight.set_low().unwrap();

            let mut reset = PinDriver::output(pins.gpio14).unwrap();
            let dc = PinDriver::output(pins.gpio8).unwrap();

            let spi = spi::SpiDriver::new(
                peripherals.spi2,
                pins.gpio10,
                pins.gpio11,
                None::<gpio::AnyIOPin>,
                &spi::SpiDriverConfig::new(),
            )
            .unwrap();

            let spi_config = Config::new()
                .baudrate(MegaHertz(40).into())
                .data_mode(Mode {
                    polarity: Polarity::IdleLow,
                    phase: Phase::CaptureOnFirstTransition,
                });

            let spi_device =
                SpiDeviceDriver::new(spi, Some(pins.gpio9), &spi_config).unwrap();
            let interface = SPIDisplayInterface::new(spi_device, dc);
            let mut delay = Delay::new_default();
            let mut display = Gc9a01::new(
                interface,
                DisplayResolution240x240,
                DisplayRotation::Rotate180,
            )
            .into_buffered_graphics();

            display.reset(&mut reset, &mut delay).unwrap();
            display.init(&mut delay).unwrap();
            display.fill(0xF800);
            display.flush().unwrap();

            backlight.set_high().unwrap();

            loop {
                std::thread::sleep(Duration::from_secs(60));
            }
        })
        .unwrap()
        .join()
        .unwrap();
}

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    the_real_main();
}
