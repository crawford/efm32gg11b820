#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTVAL4 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct SDR25SDCLKFREQVALR {
    bits: u16,
}
impl SDR25SDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR25CLKGENVALR {
    bits: bool,
}
impl SDR25CLKGENVALR {
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
#[doc = "Possible values of the field `SDR25DRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR25DRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl SDR25DRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDR25DRVSTVALR::TYPEB => 0,
            SDR25DRVSTVALR::TYPEA => 1,
            SDR25DRVSTVALR::TYPEC => 2,
            SDR25DRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDR25DRVSTVALR {
        match value {
            0 => SDR25DRVSTVALR::TYPEB,
            1 => SDR25DRVSTVALR::TYPEA,
            2 => SDR25DRVSTVALR::TYPEC,
            3 => SDR25DRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == SDR25DRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == SDR25DRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == SDR25DRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == SDR25DRVSTVALR::TYPED
    }
}
#[doc = r" Value of the field"]
pub struct SDR50SDCLKFREQVALR {
    bits: u16,
}
impl SDR50SDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR50CLCKGENVALR {
    bits: bool,
}
impl SDR50CLCKGENVALR {
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
#[doc = "Possible values of the field `SDR50DRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR50DRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl SDR50DRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDR50DRVSTVALR::TYPEB => 0,
            SDR50DRVSTVALR::TYPEA => 1,
            SDR50DRVSTVALR::TYPEC => 2,
            SDR50DRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDR50DRVSTVALR {
        match value {
            0 => SDR50DRVSTVALR::TYPEB,
            1 => SDR50DRVSTVALR::TYPEA,
            2 => SDR50DRVSTVALR::TYPEC,
            3 => SDR50DRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == SDR50DRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == SDR50DRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == SDR50DRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == SDR50DRVSTVALR::TYPED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR25"]
    #[inline]
    pub fn sdr25sdclkfreqval(&self) -> SDR25SDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SDR25SDCLKFREQVALR { bits }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR25"]
    #[inline]
    pub fn sdr25clkgenval(&self) -> SDR25CLKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR25CLKGENVALR { bits }
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR25"]
    #[inline]
    pub fn sdr25drvstval(&self) -> SDR25DRVSTVALR {
        SDR25DRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR50"]
    #[inline]
    pub fn sdr50sdclkfreqval(&self) -> SDR50SDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SDR50SDCLKFREQVALR { bits }
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR50"]
    #[inline]
    pub fn sdr50clckgenval(&self) -> SDR50CLCKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR50CLCKGENVALR { bits }
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR50"]
    #[inline]
    pub fn sdr50drvstval(&self) -> SDR50DRVSTVALR {
        SDR50DRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
