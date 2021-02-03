//! I2S (inter-IC Sound) communication using SPI peripherals

use crate::spi;

// I2S pins are mostly the same as the corresponding SPI pins:
// MOSI -> SD
// NSS -> WS (the current SPI code doesn't define NSS pins)
// SCK -> CK
// The master clock output is separate.

/// A pin that can be used as SD (serial data)
pub trait PinSd<SPI> {}
/// A pin that can be used as WS (word select, left/right clock)
pub trait PinWs<SPI> {}
/// A pin that can be used as CK (bit clock)
pub trait PinCk<SPI> {}
/// A pin that can be used as MCK (master clock output)
pub trait PinMck<SPI> {}

/// Each MOSI pin can also be used as SD
impl<P, SPI> PinSd<SPI> for P where P: spi::PinMosi<SPI> {}
/// Each SCK pin can also be used as CK
impl<P, SPI> PinCk<SPI> for P where P: spi::PinSck<SPI> {}

/// A placeholder for when the MCLK pin is not needed
pub struct NoMasterClock;

mod sealed {
    pub trait Sealed {}
}

/// A set of pins configured for I2S communication: (WS, CK, MCLK, SD)
pub trait Pins<SPI> {}

impl<SPI, PWS, PCK, PMCLK, PSD> Pins<SPI> for (PWS, PCK, PMCLK, PSD)
where
    PWS: PinWs<SPI>,
    PCK: PinCk<SPI>,
    PMCLK: PinMck<SPI>,
    PSD: PinSd<SPI>,
{
}

/// Master clock (MCK) pins
mod mck_pins {
    macro_rules! pin_mck {
        ($($PER:ident => $pin:ident<$af:ident>,)+) => {
            $(
                impl crate::i2s::sealed::Sealed for $pin<crate::gpio::Alternate<$af>> {}
                impl crate::i2s::PinMck<$PER> for $pin<crate::gpio::Alternate<$af>> {}
            )+
        };
    }

    mod common {
        use crate::gpio::{gpioc::PC6, AF5};
        use crate::pac::SPI2;
        // All STM32F4 models support PC6<AF5> for SPI2/I2S2
        pin_mck! { SPI2 => PC6<AF5>, }
    }

    #[cfg(any(
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pa3_pa6_pb10 {
        use crate::gpio::{
            gpioa::{PA3, PA6},
            gpiob::PB10,
            AF5, AF6,
        };
        use crate::pac::{SPI2, SPI3};
        pin_mck! {
            SPI2 => PA3<AF5>,
            SPI2 => PA6<AF6>,
            SPI3 => PB10<AF6>,
        }
    }

    #[cfg(any(
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    mod pc4_af5 {
        use crate::gpio::{gpioc::PC4, AF5};
        use crate::pac::SPI1;
        pin_mck! { SPI1 => PC4<AF5>, }
    }

    // On all models except the STM32F410, PC7<AF6> is the master clock output from I2S3.
    #[cfg(any(
        feature = "stm32f401",
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    mod i2s3_pc7_af6 {
        use crate::gpio::{gpioc::PC7, AF6};
        use crate::pac::SPI3;
        pin_mck! { SPI3 => PC7<AF6>, }
    }

    // On the STM32F410, PC7<AF6> is the master clock output from I2S1 instead of I2S3.
    // Also, PB10<AF6> is the master clock output from I2S1 instead of I2S3.
    #[cfg(feature = "stm32f410")]
    mod i2s1_pc7_af6 {
        use crate::gpio::{gpiob::PB10, gpioc::PC7, AF6};
        use crate::pac::SPI1;
        pin_mck! {
            SPI1 => PC7<AF6>,
            SPI1 => PB10<AF6>,
        }
    }
}

/// Word select (WS) pins
mod ws_pins {
    macro_rules! pin_ws {
        ($($PER:ident => $pin:ident<$af:ident>,)+) => {
            $(
                impl crate::i2s::sealed::Sealed for $pin<crate::gpio::Alternate<$af>> {}
                impl crate::i2s::PinWs<$PER> for $pin<crate::gpio::Alternate<$af>> {}
            )+
        };
    }

    mod common {
        use crate::gpio::{
            gpiob::{PB12, PB9},
            AF5,
        };
        use crate::pac::SPI2;
        // All STM32F4 models support these pins
        pin_ws! {
            SPI2 => PB9<AF5>,
            SPI2 => PB12<AF5>,
        }
    }

    /// Pins available on all models except the STM32F410
    #[cfg(any(
        feature = "stm32f401",
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f423",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f446",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    mod not_f410 {
        use crate::gpio::{
            gpioa::{PA15, PA4},
            AF6,
        };
        use crate::pac::SPI3;
        pin_ws! {
            SPI3 => PA4<AF6>,
            SPI3 => PA15<AF6>,
        }
    }
    #[cfg(any(
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
        feature = "stm32f446",
    ))]
    mod pa4_af5_pa15_af5 {
        use crate::gpio::{
            gpioa::{PA15, PA4},
            AF5,
        };
        use crate::pac::SPI1;
        pin_ws! {
            SPI1 => PA4<AF5>,
            SPI1 => PA15<AF5>,
        }
    }
    #[cfg(any(
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pb12_pe4_pe11 {
        use crate::gpio::{
            gpiob::PB12,
            gpioe::{PE11, PE4},
            AF5, AF6,
        };
        use crate::pac::{SPI4, SPI5};
        pin_ws! {
            SPI4 => PB12<AF6>,
            SPI4 => PE4<AF5>,
            SPI4 => PE11<AF5>,
            SPI5 => PE4<AF6>,
            SPI5 => PE11<AF6>,
        }
    }

    #[cfg(any(feature = "stm32f413", feature = "stm32f423"))]
    mod pa11 {
        use crate::gpio::{gpioa::PA11, AF5};
        use crate::pac::SPI2;
        pin_ws! { SPI2 => PA11<AF5>, }
    }

    #[cfg(any(
        feature = "stm32f410",
        feature = "stm32f411",
        feature = "stm32f412",
        feature = "stm32f413",
        feature = "stm32f423",
    ))]
    mod pb1 {
        use crate::gpio::{gpiob::PB1, AF6};
        use crate::pac::SPI5;
        pin_ws! { SPI5 => PB1<AF6>, }
    }

    #[cfg(feature = "stm32f446")]
    mod pb4_pd1 {
        use crate::gpio::{gpiob::PB4, gpiod::PD1, AF7};
        use crate::pac::SPI2;
        pin_ws! {
            SPI2 => PB4<AF7>,
            SPI2 => PD1<AF7>,
        }
    }

    #[cfg(any(
        feature = "stm32f405",
        feature = "stm32f407",
        feature = "stm32f415",
        feature = "stm32f417",
        feature = "stm32f427",
        feature = "stm32f429",
        feature = "stm32f437",
        feature = "stm32f439",
        feature = "stm32f469",
        feature = "stm32f479",
    ))]
    mod pi0 {
        use crate::gpio::{gpioi::PI0, AF5};
        use crate::pac::SPI2;
        pin_ws! { SPI2 => PI0<AF5>, }
    }
}

/// An SPI peripheral that can be used in I2S mode
pub trait Enable: sealed::Sealed {
    /// Enables the peripheral by setting the corresponding enable bit in an RCC register
    fn enable();
}

// All STM32F4 models use the same bits in APB1ENR, APB2ENR, APB1RSTR, and APB2RSTR to enable
// and reset the SPI peripherals.
// SPI1: APB2 bit 12
// SPI2: APB1 bit 14
// SPI3: APB1 bit 15
// SPI4: APB2 bit 13
// SPI5: APB2 bit 20

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
    feature = "stm32f446",
))]
mod spi1 {
    use super::sealed::Sealed;
    use super::{Enable, NoMasterClock, PinMck};
    use crate::bb;
    use crate::pac::{RCC, SPI1};
    impl Sealed for SPI1 {}
    impl Enable for SPI1 {
        fn enable() {
            unsafe {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                let rcc = &(*RCC::ptr());
                const SPI_BIT: u8 = 12;
                // Enable clock, enable reset, clear, reset
                bb::set(&rcc.apb2enr, SPI_BIT);
                bb::set(&rcc.apb2rstr, SPI_BIT);
                bb::clear(&rcc.apb2rstr, SPI_BIT);
            }
        }
    }
    impl PinMck<SPI1> for NoMasterClock {}
}

// All STM32F4 models support SPI2/I2S2
mod spi2 {
    use super::sealed::Sealed;
    use super::{Enable, NoMasterClock, PinMck};
    use crate::bb;
    use crate::pac::{RCC, SPI2};
    impl Sealed for SPI2 {}
    impl Enable for SPI2 {
        fn enable() {
            unsafe {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                let rcc = &(*RCC::ptr());
                const SPI_BIT: u8 = 14;
                // Enable clock, enable reset, clear, reset
                bb::set(&rcc.apb1enr, SPI_BIT);
                bb::set(&rcc.apb1rstr, SPI_BIT);
                bb::clear(&rcc.apb1rstr, SPI_BIT);
            }
        }
    }
    impl PinMck<SPI2> for NoMasterClock {}
}

// All STM32F4 models except STM32F410 support SPI3/I2S3
#[cfg(any(
    feature = "stm32f401",
    feature = "stm32f405",
    feature = "stm32f407",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f415",
    feature = "stm32f417",
    feature = "stm32f423",
    feature = "stm32f427",
    feature = "stm32f429",
    feature = "stm32f437",
    feature = "stm32f439",
    feature = "stm32f446",
    feature = "stm32f469",
    feature = "stm32f479",
))]
mod spi3 {
    use super::sealed::Sealed;
    use super::{Enable, NoMasterClock, PinMck};
    use crate::bb;
    use crate::pac::{RCC, SPI3};
    impl Sealed for SPI3 {}
    impl Enable for SPI3 {
        fn enable() {
            unsafe {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                let rcc = &(*RCC::ptr());
                const SPI_BIT: u8 = 15;
                // Enable clock, enable reset, clear, reset
                bb::set(&rcc.apb1enr, SPI_BIT);
                bb::set(&rcc.apb1rstr, SPI_BIT);
                bb::clear(&rcc.apb1rstr, SPI_BIT);
            }
        }
    }
    impl PinMck<SPI3> for NoMasterClock {}
}

#[cfg(any(
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
mod spi4 {
    use super::sealed::Sealed;
    use super::{Enable, NoMasterClock, PinMck};
    use crate::bb;
    use crate::pac::{RCC, SPI4};
    impl Sealed for SPI4 {}
    impl Enable for SPI4 {
        fn enable() {
            unsafe {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                let rcc = &(*RCC::ptr());
                const SPI_BIT: u8 = 13;
                // Enable clock, enable reset, clear, reset
                bb::set(&rcc.apb2enr, SPI_BIT);
                bb::set(&rcc.apb2rstr, SPI_BIT);
                bb::clear(&rcc.apb2rstr, SPI_BIT);
            }
        }
    }
    impl PinMck<SPI4> for NoMasterClock {}
}

#[cfg(any(
    feature = "stm32f410",
    feature = "stm32f411",
    feature = "stm32f412",
    feature = "stm32f413",
    feature = "stm32f423",
))]
mod spi5 {
    use super::sealed::Sealed;
    use super::{Enable, NoMasterClock, PinMck};
    use crate::bb;
    use crate::pac::{RCC, SPI5};
    impl Sealed for SPI5 {}
    impl Enable for SPI5 {
        fn enable() {
            unsafe {
                // NOTE(unsafe) this reference will only be used for atomic writes with no side effects.
                let rcc = &(*RCC::ptr());
                const SPI_BIT: u8 = 20;
                // Enable clock, enable reset, clear, reset
                bb::set(&rcc.apb2enr, SPI_BIT);
                bb::set(&rcc.apb2rstr, SPI_BIT);
                bb::clear(&rcc.apb2rstr, SPI_BIT);
            }
        }
    }
    impl PinMck<SPI5> for NoMasterClock {}
}
