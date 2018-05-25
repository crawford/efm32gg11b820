#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CAPAB0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TMOUTCLKFREQR {
    bits: u8,
}
impl TMOUTCLKFREQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TMOUTCLKUNITR {
    bits: bool,
}
impl TMOUTCLKUNITR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BASECLKFREQSDR {
    bits: u8,
}
impl BASECLKFREQSDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MAXBLOCKLENR {
    bits: u8,
}
impl MAXBLOCKLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct EXTMEDIABUSSUPR {
    bits: bool,
}
impl EXTMEDIABUSSUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ADMA2SUPR {
    bits: bool,
}
impl ADMA2SUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct HSSUPR {
    bits: bool,
}
impl HSSUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SDMASUPR {
    bits: bool,
}
impl SDMASUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SUSRESSUPR {
    bits: bool,
}
impl SUSRESSUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VOLTSUP3P3VR {
    bits: bool,
}
impl VOLTSUP3P3VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VOLTSUP3P0VR {
    bits: bool,
}
impl VOLTSUP3P0VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct VOLTSUP1P8VR {
    bits: bool,
}
impl VOLTSUP1P8VR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct SYSBUS64BSUPR {
    bits: bool,
}
impl SYSBUS64BSUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ASYNCINTSUPR {
    bits: bool,
}
impl ASYNCINTSUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `IFSLOTTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFSLOTTYPER {
    #[doc = "Removable Card Slot"]
    REMOVABLE,
    #[doc = "Only one non-removable device is conected to a SD bus slot"]
    EMBEDDED,
    #[doc = "Can be set if Host controller supports Shared Bus CTRL register"]
    SHARED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl IFSLOTTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IFSLOTTYPER::REMOVABLE => 0,
            IFSLOTTYPER::EMBEDDED => 1,
            IFSLOTTYPER::SHARED => 2,
            IFSLOTTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IFSLOTTYPER {
        match value {
            0 => IFSLOTTYPER::REMOVABLE,
            1 => IFSLOTTYPER::EMBEDDED,
            2 => IFSLOTTYPER::SHARED,
            i => IFSLOTTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `REMOVABLE`"]
    #[inline]
    pub fn is_removable(&self) -> bool {
        *self == IFSLOTTYPER::REMOVABLE
    }
    #[doc = "Checks if the value of the field is `EMBEDDED`"]
    #[inline]
    pub fn is_embedded(&self) -> bool {
        *self == IFSLOTTYPER::EMBEDDED
    }
    #[doc = "Checks if the value of the field is `SHARED`"]
    #[inline]
    pub fn is_shared(&self) -> bool {
        *self == IFSLOTTYPER::SHARED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Timeout Clock Frequency"]
    #[inline]
    pub fn tmoutclkfreq(&self) -> TMOUTCLKFREQR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMOUTCLKFREQR { bits }
    }
    #[doc = "Bit 7 - Timeout Clock Unit"]
    #[inline]
    pub fn tmoutclkunit(&self) -> TMOUTCLKUNITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TMOUTCLKUNITR { bits }
    }
    #[doc = "Bits 8:15 - Base Clock Frequency for SD_CLK"]
    #[inline]
    pub fn baseclkfreqsd(&self) -> BASECLKFREQSDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BASECLKFREQSDR { bits }
    }
    #[doc = "Bits 16:17 - Maximum Block Length"]
    #[inline]
    pub fn maxblocklen(&self) -> MAXBLOCKLENR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MAXBLOCKLENR { bits }
    }
    #[doc = "Bit 18 - Extended Media Bus Support"]
    #[inline]
    pub fn extmediabussup(&self) -> EXTMEDIABUSSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EXTMEDIABUSSUPR { bits }
    }
    #[doc = "Bit 19 - ADMA2 Support"]
    #[inline]
    pub fn adma2sup(&self) -> ADMA2SUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADMA2SUPR { bits }
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline]
    pub fn hssup(&self) -> HSSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSSUPR { bits }
    }
    #[doc = "Bit 22 - SDMA Support"]
    #[inline]
    pub fn sdmasup(&self) -> SDMASUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDMASUPR { bits }
    }
    #[doc = "Bit 23 - Suspend / Resume Support"]
    #[inline]
    pub fn susressup(&self) -> SUSRESSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SUSRESSUPR { bits }
    }
    #[doc = "Bit 24 - Voltage Support 3.3V"]
    #[inline]
    pub fn voltsup3p3v(&self) -> VOLTSUP3P3VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOLTSUP3P3VR { bits }
    }
    #[doc = "Bit 25 - Voltage Support 3.0V"]
    #[inline]
    pub fn voltsup3p0v(&self) -> VOLTSUP3P0VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOLTSUP3P0VR { bits }
    }
    #[doc = "Bit 26 - Voltage Support 1.8V"]
    #[inline]
    pub fn voltsup1p8v(&self) -> VOLTSUP1P8VR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VOLTSUP1P8VR { bits }
    }
    #[doc = "Bit 28 - System Bus 64-bit Support"]
    #[inline]
    pub fn sysbus64bsup(&self) -> SYSBUS64BSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SYSBUS64BSUPR { bits }
    }
    #[doc = "Bit 29 - Asynchronous Interrupt Support"]
    #[inline]
    pub fn asyncintsup(&self) -> ASYNCINTSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCINTSUPR { bits }
    }
    #[doc = "Bits 30:31 - Interface Card Slot Type"]
    #[inline]
    pub fn ifslottype(&self) -> IFSLOTTYPER {
        IFSLOTTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
