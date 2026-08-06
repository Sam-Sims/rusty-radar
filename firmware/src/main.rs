use std::{thread};
use embassy_time::{with_deadline, Duration, Instant};
use anyhow::{anyhow, Error, Result};
use cst816s::{
    command::{Gesture, IrqCtl},
    Cst816s,
};
use esp_idf_svc::hal::{
    delay::Delay,
    gpio::{self, PinDriver, Pull},
    i2c,
    peripherals::Peripherals,
    spi::{
        self,
        config::{Config, Mode, Phase, Polarity},
        SpiDeviceDriver,
    },
    task::block_on,
    units::MegaHertz,
};
use gc9a01::{mode::BasicMode, prelude::*, Gc9a01, SPIDisplayInterface};
use rusty_radar_graphics::{Aircraft, RadarScale};
use rusty_radar_simulator::aircraft::SimCraft;

type BasicRadarDisplay = Gc9a01<
    SPIInterface<spi::SpiSingleDeviceDriver<'static>, PinDriver<'static, gpio::Output>>,
    DisplayResolution240x240,
    BasicMode,
>;

// pin out reference: https://docs.waveshare.com/assets/images/ESP32-S3-Touch-LCD-1.28-pin-4c11432ab715535bf20f334d46ebefbf.webp
fn initalise_display(
    spi2: spi::SPI2<'static>,
    lcd_clk: gpio::Gpio10<'static>,
    lcd_mosi: gpio::Gpio11<'static>,
    lcd_cs: gpio::Gpio9<'static>,
    lcd_dc: gpio::Gpio8<'static>,
    lcd_rst_driver: &mut PinDriver<'static, gpio::Output>,
    lcd_bl_driver: &mut PinDriver<'static, gpio::Output>,
    delay: &mut Delay,
) -> Result<BasicRadarDisplay> {
    lcd_bl_driver.set_low()?;

    let lcd_dc_driver = PinDriver::output(lcd_dc)?;

    let spi_driver = spi::SpiDriver::new(
        spi2,
        lcd_clk,
        lcd_mosi,
        None::<gpio::AnyIOPin>,
        &spi::SpiDriverConfig::new(),
    )?;

    let spi_config = Config::new()
        .baudrate(MegaHertz(40).into())
        .data_mode(Mode {
            polarity: Polarity::IdleLow,
            phase: Phase::CaptureOnFirstTransition,
        });

    let spi_device = SpiDeviceDriver::new(spi_driver, Some(lcd_cs), &spi_config)?;
    let interface = SPIDisplayInterface::new(spi_device, lcd_dc_driver);

    let mut display_device = Gc9a01::new(
        interface,
        DisplayResolution240x240,
        DisplayRotation::Rotate180,
    );
    display_device.reset(lcd_rst_driver, delay)?;
    display_device
        .init(delay)
        .map_err(|error| anyhow!("error initalising display: {error:?}"))?;
    display_device
        .clear()
        .map_err(|error| anyhow!("error clearing display: {error:?}"))?;
    lcd_bl_driver.set_high()?;
    Ok(display_device)
}

// pin out reference: https://docs.waveshare.com/assets/images/ESP32-S3-Touch-LCD-1.28-pin-4c11432ab715535bf20f334d46ebefbf.webp
fn initalise_touch(
    i2c0: i2c::I2C0<'static>,
    tp_sda: gpio::Gpio6<'static>,
    tp_scl: gpio::Gpio7<'static>,
    tp_rst_driver: &mut PinDriver<'static, gpio::Output>,
    delay: &mut Delay,
) -> Result<Cst816s<i2c::I2cDriver<'static>, Delay>> {
    let i2c = i2c::I2cDriver::new(i2c0, tp_sda, tp_scl, &i2c::I2cConfig::new())?;

    let mut touch_device = Cst816s::new(i2c, Delay::new_default());
    touch_device.reset(tp_rst_driver, delay)?;

    let mut irq_ctl = IrqCtl(0);
    irq_ctl.set_en_motion(true);

    touch_device.write_irq_ctl(irq_ctl)?;

    Ok(touch_device)
}

fn the_real_main() -> Result<(), Error> {
    let peripherals = Peripherals::take()?;

    // buffered graphics needs a larger than default stack size
    thread::Builder::new()
        .name("display".into())
        .stack_size(7000 + (160 * 1024) + 4096)
        .spawn(move || -> Result<(), Error> {
            let pins = peripherals.pins;
            let mut delay = Delay::new_default();

            let mut lcd_bl = PinDriver::output(pins.gpio2)?;
            let mut lcd_rst = PinDriver::output(pins.gpio14)?;
            let mut display_device = initalise_display(
                peripherals.spi2,
                pins.gpio10,
                pins.gpio11,
                pins.gpio9,
                pins.gpio8,
                &mut lcd_rst,
                &mut lcd_bl,
                &mut delay,
            )?
            .into_buffered_graphics();

            let mut tp_rst_driver = PinDriver::output(pins.gpio13)?;
            let mut tp_int_driver = PinDriver::input(pins.gpio5, Pull::Up)?;
            let mut touch_device = initalise_touch(
                peripherals.i2c0,
                pins.gpio6,
                pins.gpio7,
                &mut tp_rst_driver,
                &mut delay,
            )?;

            // mock some planes
            let mut simulated_planes = [
                SimCraft::new(),
                SimCraft::new(),
                SimCraft::new(),
                SimCraft::new(),
                SimCraft::new(),
            ];

            let mut scale = RadarScale::Km20;
            let mut redraw =
                |scale: RadarScale, aircraft: &[Aircraft<'_>]| -> Result<(), Error> {
                    rusty_radar_graphics::draw_frame(&mut display_device, scale).map_err(|error| anyhow!("{error:?}"))?;
                    rusty_radar_graphics::draw_planes(&mut display_device, aircraft).map_err(|error| anyhow!("{error:?}"))?;
                    display_device.flush().map_err(|error| anyhow!("{error:?}"))?;
                    Ok(())
                };

            let update_timeout = Duration::from_secs(2);
            let mut next_update = Instant::now() + update_timeout;

            let aircraft = simulated_planes.each_ref().map(SimCraft::to_aircraft);
            redraw(scale, &aircraft)?;

            // event handler
            loop {
                match block_on(with_deadline(
                    next_update,
                    tp_int_driver.wait_for_rising_edge(),
                )) {
                    // touch screen event
                    Ok(future) => {
                        future?;

                        let event = touch_device.read_event()?;
                        log::info!("touch event: {event:?}");

                        if event.gesture_type == Gesture::SingleClick {
                            scale = scale.next();

                            let aircraft = simulated_planes.each_ref().map(SimCraft::to_aircraft);
                            redraw(scale, &aircraft)?;
                        }
                    }
                    // timeout
                    Err(_) => {
                        for craft in &mut simulated_planes {
                            if !craft.update() {
                                *craft = SimCraft::new();
                            }
                        }

                        let aircraft = simulated_planes.each_ref().map(SimCraft::to_aircraft);
                        redraw(scale, &aircraft)?;

                        next_update = Instant::now() + update_timeout;
                    }
                }

            }
        })?
        .join()
        .map_err(|_| anyhow!("display thread panic"))??;

    Ok(())
}

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!!!!!");

    if let Err(error) = the_real_main() {
        log::error!("firmware panic: {error:?}");
    }
}
