#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct VMONRDYR {
    bits: bool,
}
impl VMONRDYR {
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
pub struct VMONAVDDR {
    bits: bool,
}
impl VMONAVDDR {
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
pub struct VMONALTAVDDR {
    bits: bool,
}
impl VMONALTAVDDR {
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
pub struct VMONDVDDR {
    bits: bool,
}
impl VMONDVDDR {
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
pub struct VMONIO0R {
    bits: bool,
}
impl VMONIO0R {
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
pub struct VMONIO1R {
    bits: bool,
}
impl VMONIO1R {
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
pub struct VMONBUVDDR {
    bits: bool,
}
impl VMONBUVDDR {
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
pub struct BURDYR {
    bits: bool,
}
impl BURDYR {
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
#[doc = "Possible values of the field `VSCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VSCALER {
    #[doc = "Voltage Scale Level 2"]
    VSCALE2,
    #[doc = "Voltage Scale Level 0"]
    VSCALE0,
    #[doc = "RESV"]
    RESV,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl VSCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            VSCALER::VSCALE2 => 0,
            VSCALER::VSCALE0 => 2,
            VSCALER::RESV => 3,
            VSCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> VSCALER {
        match value {
            0 => VSCALER::VSCALE2,
            2 => VSCALER::VSCALE0,
            3 => VSCALER::RESV,
            i => VSCALER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline]
    pub fn is_vscale2(&self) -> bool {
        *self == VSCALER::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline]
    pub fn is_vscale0(&self) -> bool {
        *self == VSCALER::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline]
    pub fn is_resv(&self) -> bool {
        *self == VSCALER::RESV
    }
}
#[doc = r" Value of the field"]
pub struct VSCALEBUSYR {
    bits: bool,
}
impl VSCALEBUSYR {
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
pub struct EM4IORETR {
    bits: bool,
}
impl EM4IORETR {
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
pub struct TEMPACTIVER {
    bits: bool,
}
impl TEMPACTIVER {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - VMON Ready"]
    #[inline]
    pub fn vmonrdy(&self) -> VMONRDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONRDYR { bits }
    }
    #[doc = "Bit 1 - VMON AVDD Channel"]
    #[inline]
    pub fn vmonavdd(&self) -> VMONAVDDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONAVDDR { bits }
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel"]
    #[inline]
    pub fn vmonaltavdd(&self) -> VMONALTAVDDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONALTAVDDR { bits }
    }
    #[doc = "Bit 3 - VMON DVDD Channel"]
    #[inline]
    pub fn vmondvdd(&self) -> VMONDVDDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONDVDDR { bits }
    }
    #[doc = "Bit 4 - VMON IOVDD0 Channel"]
    #[inline]
    pub fn vmonio0(&self) -> VMONIO0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONIO0R { bits }
    }
    #[doc = "Bit 5 - VMON IOVDD1 Channel"]
    #[inline]
    pub fn vmonio1(&self) -> VMONIO1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONIO1R { bits }
    }
    #[doc = "Bit 7 - VMON BUVDD Channel"]
    #[inline]
    pub fn vmonbuvdd(&self) -> VMONBUVDDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VMONBUVDDR { bits }
    }
    #[doc = "Bit 12 - Backup Mode Ready"]
    #[inline]
    pub fn burdy(&self) -> BURDYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BURDYR { bits }
    }
    #[doc = "Bits 16:17 - Current Voltage Scale Value"]
    #[inline]
    pub fn vscale(&self) -> VSCALER {
        VSCALER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - System is Busy Scaling Voltage"]
    #[inline]
    pub fn vscalebusy(&self) -> VSCALEBUSYR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VSCALEBUSYR { bits }
    }
    #[doc = "Bit 20 - IO Retention Status"]
    #[inline]
    pub fn em4ioret(&self) -> EM4IORETR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM4IORETR { bits }
    }
    #[doc = "Bit 26 - Temperature Measurement Active"]
    #[inline]
    pub fn tempactive(&self) -> TEMPACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TEMPACTIVER { bits }
    }
}
