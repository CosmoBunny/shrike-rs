#[cfg(not(feature = "embassy"))]
use crate::{error::FlashError, universal::UniversalSPIflash};

#[cfg(not(feature = "embassy"))]
use rp235x_hal::{
    Clock, Spi,
    clocks::PeripheralClock,
    fugit::RateExtU32,
    gpio::{
        FunctionNull, Pin, PullDown,
        bank0::{Gpio0, Gpio1, Gpio2, Gpio3, Gpio12, Gpio13},
    },
};

#[cfg(not(feature = "embassy"))]
use rp235x_hal::pac::{RESETS, SPI0};

#[cfg(not(feature = "embassy"))]
use embedded_hal::delay::DelayNs;

#[cfg(not(feature = "embassy"))]
pub fn init_rp2350_normal<D: DelayNs>(
    clock: &PeripheralClock,
    resets: &mut RESETS,
    spi: SPI0,
    en: Pin<Gpio13, FunctionNull, PullDown>,
    pwr: Pin<Gpio12, FunctionNull, PullDown>,
    ss: Pin<Gpio1, FunctionNull, PullDown>,
    mosi: Pin<Gpio3, FunctionNull, PullDown>,
    miso: Pin<Gpio0, FunctionNull, PullDown>,
    sclk: Pin<Gpio2, FunctionNull, PullDown>,
    delay: &mut D,
    bin: &[u8],
) -> Result<(), FlashError> {
    use crate::universal::Flash;
    let mosi = mosi.into_function::<rp235x_hal::gpio::FunctionSpi>();
    let miso = miso.into_function::<rp235x_hal::gpio::FunctionSpi>();
    let sclk = sclk.into_function::<rp235x_hal::gpio::FunctionSpi>();

    let en = en.into_function::<rp235x_hal::gpio::FunctionSioOutput>();
    let ss = ss.into_function::<rp235x_hal::gpio::FunctionSioOutput>();
    let pwr = pwr.into_function::<rp235x_hal::gpio::FunctionSioOutput>();

    let spi = Spi::<_, _, _, 8>::new(spi, (mosi, miso, sclk)).init(
        resets,
        1_600_000.Hz(),
        clock.freq(),
        embedded_hal::spi::MODE_0,
    );

    let mut flash = UniversalSPIflash::new(spi, en, pwr, ss, delay);
    flash.flash(bin)
}

#[cfg(feature = "embassy")]
pub mod async_rp {
    use embassy_rp::{
        Peri, bind_interrupts, dma,
        gpio::{Level, Output},
        peripherals::{DMA_CH0, DMA_CH1, PIN_0, PIN_1, PIN_2, PIN_3, PIN_12, PIN_13, SPI0},
        spi::{Config, Spi},
    };

    use crate::{
        async_universal::UniversalAsyncSPIflash, error::FlashError, universal::UniversalSPIflash,
    };

    pub async fn init_rp2350_blocking_async(
        spi: Peri<'static, SPI0>,
        en: Peri<'static, PIN_13>,
        pwr: Peri<'static, PIN_12>,
        ss: Peri<'static, PIN_1>,
        mosi: Peri<'static, PIN_3>,
        miso: Peri<'static, PIN_0>,
        sclk: Peri<'static, PIN_2>,
        bin: &[u8],
    ) -> Result<(), FlashError> {
        use crate::async_universal::AsyncFlash;

        let mut config = Config::default();

        config.frequency = 1_600_000;

        let spi = Spi::new_blocking(spi, sclk, mosi, miso, config);

        let en = Output::new(en, Level::Low);
        let pwr = Output::new(pwr, Level::Low);
        let ss = Output::new(ss, Level::Low);

        let mut flash = UniversalSPIflash::new(spi, en, pwr, ss);
        flash.flash(bin).await
    }

    pub async fn init_rp2350_all_async(
        spi: Peri<'static, SPI0>,
        en: Peri<'static, PIN_13>,
        pwr: Peri<'static, PIN_12>,
        ss: Peri<'static, PIN_1>,
        mosi: Peri<'static, PIN_3>,
        miso: Peri<'static, PIN_0>,
        sclk: Peri<'static, PIN_2>,
        tx_dma: Peri<'_, DMA_CH0>,
        rx_dma: Peri<'_, DMA_CH1>,
        bin: &[u8],
    ) -> Result<(), FlashError> {
        use crate::async_universal::AsyncFlash;

        let mut config = Config::default();

        config.frequency = 1_600_000;

        let spi = Spi::new(spi, sclk, mosi, miso, tx_dma, rx_dma, Irqs, config);

        let en = Output::new(en, Level::Low);
        let pwr = Output::new(pwr, Level::Low);
        let ss = Output::new(ss, Level::Low);

        let mut flash = UniversalAsyncSPIflash::new(spi, en, pwr, ss);
        flash.flash(bin).await
    }

    use embassy_executor::task;
    bind_interrupts!(struct Irqs {
        DMA_IRQ_0 => dma::InterruptHandler<DMA_CH0>, dma::InterruptHandler<DMA_CH1>;
    });

    #[task]
    pub async fn spawn_rp2350_all_async(
        spi: Peri<'static, SPI0>,
        en: Peri<'static, PIN_13>,
        pwr: Peri<'static, PIN_12>,
        ss: Peri<'static, PIN_1>,
        mosi: Peri<'static, PIN_3>,
        miso: Peri<'static, PIN_0>,
        sclk: Peri<'static, PIN_2>,
        tx_dma: Peri<'static, DMA_CH0>,
        rx_dma: Peri<'static, DMA_CH1>,
        bin: &'static [u8],
        error: fn(FlashError),
    ) {
        use crate::async_universal::AsyncFlash;

        let mut config = Config::default();

        config.frequency = 1_600_000;

        let spi = Spi::new(spi, sclk, mosi, miso, tx_dma, rx_dma, Irqs, config);

        let en = Output::new(en, Level::Low);
        let pwr = Output::new(pwr, Level::Low);
        let ss = Output::new(ss, Level::Low);

        let mut flash = UniversalAsyncSPIflash::new(spi, en, pwr, ss);
        flash.flash(bin).await.map_err(|err| error(err)).ok();
    }
}
