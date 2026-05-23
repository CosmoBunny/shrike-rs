use embedded_hal_async::spi::SpiBus;

use embedded_hal::digital::OutputPin;

#[cfg(not(feature = "embassy"))]
use embedded_hal_async::delay::DelayNs;

use crate::error::FlashError;

/// Async Flash, This trait do flash asynchronously
pub trait AsyncFlash {
    fn flash(&mut self, bin: &[u8]) -> impl core::future::Future<Output = Result<(), FlashError>>;
}

// -------------------------- NORMAL PART -----------------------------------

#[cfg(not(feature = "embassy"))]
pub struct UniversalAsyncSPIflash<
    'a,
    SPI: SpiBus,
    EN: OutputPin,
    PWR: OutputPin,
    SS: OutputPin,
    D: DelayNs,
> {
    spi: SPI,
    pwr: PWR,
    en: EN,
    ss: SS,
    delay: &'a mut D,
}

#[cfg(not(feature = "embassy"))]
impl<'a, SPI, EN, PWR, SS, D> UniversalAsyncSPIflash<'a, SPI, EN, PWR, SS, D>
where
    SPI: SpiBus,
    EN: OutputPin,
    PWR: OutputPin,
    SS: OutputPin,
    D: DelayNs,
{
    pub fn new(spi: SPI, en: EN, pwr: PWR, ss: SS, delay: &'a mut D) -> Self {
        Self {
            spi,
            pwr,
            en,
            ss,
            delay,
        }
    }
    fn set_low_ss(&mut self) -> Result<(), FlashError> {
        self.ss.set_low().map_err(|_| FlashError::SetLowError)
    }
    fn set_high_ss(&mut self) -> Result<(), FlashError> {
        self.ss.set_high().map_err(|_| FlashError::SetHighError)
    }
    fn set_low_pwr(&mut self) -> Result<(), FlashError> {
        self.pwr.set_low().map_err(|_| FlashError::SetLowError)
    }
    fn set_high_pwr(&mut self) -> Result<(), FlashError> {
        self.pwr.set_high().map_err(|_| FlashError::SetHighError)
    }
    fn set_low_en(&mut self) -> Result<(), FlashError> {
        self.en.set_low().map_err(|_| FlashError::SetLowError)
    }
    fn set_high_en(&mut self) -> Result<(), FlashError> {
        self.en.set_high().map_err(|_| FlashError::SetHighError)
    }
}

#[cfg(not(feature = "embassy"))]
impl<'a, SPI: SpiBus, EN: OutputPin, PWR: OutputPin, SS: OutputPin, D: DelayNs> AsyncFlash
    for UniversalAsyncSPIflash<'a, SPI, EN, PWR, SS, D>
{
    async fn flash(&mut self, bin: &[u8]) -> Result<(), FlashError> {
        // Reset for moment
        self.set_low_pwr().await?;
        self.set_high_en().await?;
        self.delay.delay_ms(500).await;

        // Power Up FPGA
        self.set_low_ss().await?;
        self.set_low_en().await?;
        self.set_low_pwr().await?;
        self.delay.delay_ms(100).await;
        self.set_high_en().await?;
        self.set_high_pwr().await?;
        self.delay.delay_ms(100).await;

        // Start SPI Interface
        self.set_high_ss().await?;
        self.delay.delay_ms(2).await;
        self.set_low_ss().await?;

        // Tranfer the data in 4096 chunks wise.
        for chunk in bin.chunks(4096) {
            self.spi
                .write(chunk)
                .await
                .map_err(|_| FlashError::SpiWriteError)?;
        }

        self.set_high_ss().await?;
        self.delay.delay_ms(100).await;

        Ok(())
    }
}

// -------------------------- Embassy PART -----------------------------------

#[cfg(feature = "embassy")]
pub struct UniversalAsyncSPIflash<SPI: SpiBus, EN: OutputPin, PWR: OutputPin, SS: OutputPin> {
    spi: SPI,
    pwr: PWR,
    en: EN,
    ss: SS,
}

#[cfg(feature = "embassy")]
impl<SPI, EN, PWR, SS> UniversalAsyncSPIflash<SPI, EN, PWR, SS>
where
    SPI: SpiBus,
    EN: OutputPin,
    PWR: OutputPin,
    SS: OutputPin,
{
    pub fn new(spi: SPI, en: EN, pwr: PWR, ss: SS) -> Self {
        Self { spi, pwr, en, ss }
    }
    async fn set_low_ss(&mut self) -> Result<(), FlashError> {
        self.ss.set_low().map_err(|_| FlashError::SetLowError)?;
        Ok(())
    }
    async fn set_high_ss(&mut self) -> Result<(), FlashError> {
        self.ss.set_high().map_err(|_| FlashError::SetHighError)?;
        Ok(())
    }
    async fn set_low_pwr(&mut self) -> Result<(), FlashError> {
        self.pwr.set_low().map_err(|_| FlashError::SetLowError)?;
        Ok(())
    }
    async fn set_high_pwr(&mut self) -> Result<(), FlashError> {
        self.pwr.set_high().map_err(|_| FlashError::SetHighError)?;
        Ok(())
    }
    async fn set_low_en(&mut self) -> Result<(), FlashError> {
        self.en.set_low().map_err(|_| FlashError::SetLowError)?;
        Ok(())
    }
    async fn set_high_en(&mut self) -> Result<(), FlashError> {
        self.en.set_high().map_err(|_| FlashError::SetHighError)?;
        Ok(())
    }
}

#[cfg(feature = "embassy")]
impl<SPI: SpiBus, EN: OutputPin, PWR: OutputPin, SS: OutputPin> AsyncFlash
    for UniversalAsyncSPIflash<SPI, EN, PWR, SS>
{
    async fn flash(&mut self, bin: &[u8]) -> Result<(), FlashError> {
        // Reset for moment

        use embassy_time::Timer;
        self.set_low_pwr().await?;
        self.set_high_en().await?;
        Timer::after_millis(500).await;

        // Power Up FPGA
        self.set_low_ss().await?;
        self.set_low_en().await?;
        self.set_low_pwr().await?;
        Timer::after_millis(100).await;
        self.set_high_en().await?;
        self.set_high_pwr().await?;
        Timer::after_millis(100).await;

        // Start SPI Interface
        self.set_high_ss().await?;
        Timer::after_millis(2).await;
        self.set_low_ss().await?;

        // Tranfer the data in 4096 chunks wise.
        for chunk in bin.chunks(4096) {
            self.spi
                .write(chunk)
                .await
                .map_err(|_| FlashError::SpiWriteError)?;
        }

        self.set_high_ss().await?;
        Timer::after_millis(100).await;

        Ok(())
    }
}
