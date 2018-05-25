#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTVAL6 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct SDR104SDCLKFREQVALR {
    bits: u16,
}
impl SDR104SDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR104CLKGENVALR {
    bits: bool,
}
impl SDR104CLKGENVALR {
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
#[doc = "Possible values of the field `SDR104DRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104DRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl SDR104DRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDR104DRVSTVALR::TYPEB => 0,
            SDR104DRVSTVALR::TYPEA => 1,
            SDR104DRVSTVALR::TYPEC => 2,
            SDR104DRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDR104DRVSTVALR {
        match value {
            0 => SDR104DRVSTVALR::TYPEB,
            1 => SDR104DRVSTVALR::TYPEA,
            2 => SDR104DRVSTVALR::TYPEC,
            3 => SDR104DRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == SDR104DRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == SDR104DRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == SDR104DRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == SDR104DRVSTVALR::TYPED
    }
}
#[doc = r" Value of the field"]
pub struct DDR50SDCLKFREQVALR {
    bits: u16,
}
impl DDR50SDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DDR50CLKGENVALR {
    bits: bool,
}
impl DDR50CLKGENVALR {
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
#[doc = "Possible values of the field `DDR50DRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR50DRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl DDR50DRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DDR50DRVSTVALR::TYPEB => 0,
            DDR50DRVSTVALR::TYPEA => 1,
            DDR50DRVSTVALR::TYPEC => 2,
            DDR50DRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DDR50DRVSTVALR {
        match value {
            0 => DDR50DRVSTVALR::TYPEB,
            1 => DDR50DRVSTVALR::TYPEA,
            2 => DDR50DRVSTVALR::TYPEC,
            3 => DDR50DRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == DDR50DRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == DDR50DRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == DDR50DRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == DDR50DRVSTVALR::TYPED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR104"]
    #[inline]
    pub fn sdr104sdclkfreqval(&self) -> SDR104SDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SDR104SDCLKFREQVALR { bits }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR104"]
    #[inline]
    pub fn sdr104clkgenval(&self) -> SDR104CLKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR104CLKGENVALR { bits }
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR104"]
    #[inline]
    pub fn sdr104drvstval(&self) -> SDR104DRVSTVALR {
        SDR104DRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for DDR50"]
    #[inline]
    pub fn ddr50sdclkfreqval(&self) -> DDR50SDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DDR50SDCLKFREQVALR { bits }
    }
    #[doc = "Bit 26 - Clock Generator Select Value for DDR50"]
    #[inline]
    pub fn ddr50clkgenval(&self) -> DDR50CLKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDR50CLKGENVALR { bits }
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for DDR50"]
    #[inline]
    pub fn ddr50drvstval(&self) -> DDR50DRVSTVALR {
        DDR50DRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
