#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTVAL2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct HSPSDCLKFREQVALR {
    bits: u16,
}
impl HSPSDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HSPCLKGENVALR {
    bits: bool,
}
impl HSPCLKGENVALR {
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
#[doc = "Possible values of the field `HSPDRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPDRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl HSPDRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            HSPDRVSTVALR::TYPEB => 0,
            HSPDRVSTVALR::TYPEA => 1,
            HSPDRVSTVALR::TYPEC => 2,
            HSPDRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> HSPDRVSTVALR {
        match value {
            0 => HSPDRVSTVALR::TYPEB,
            1 => HSPDRVSTVALR::TYPEA,
            2 => HSPDRVSTVALR::TYPEC,
            3 => HSPDRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == HSPDRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == HSPDRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == HSPDRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == HSPDRVSTVALR::TYPED
    }
}
#[doc = r" Value of the field"]
pub struct SDR12SDCLKFREQVALR {
    bits: u16,
}
impl SDR12SDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SDR12CLKGENVALR {
    bits: bool,
}
impl SDR12CLKGENVALR {
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
#[doc = "Possible values of the field `SDR12DRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR12DRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl SDR12DRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SDR12DRVSTVALR::TYPEB => 0,
            SDR12DRVSTVALR::TYPEA => 1,
            SDR12DRVSTVALR::TYPEC => 2,
            SDR12DRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SDR12DRVSTVALR {
        match value {
            0 => SDR12DRVSTVALR::TYPEB,
            1 => SDR12DRVSTVALR::TYPEA,
            2 => SDR12DRVSTVALR::TYPEC,
            3 => SDR12DRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == SDR12DRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == SDR12DRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == SDR12DRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == SDR12DRVSTVALR::TYPED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for High Speed"]
    #[inline]
    pub fn hspsdclkfreqval(&self) -> HSPSDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        HSPSDCLKFREQVALR { bits }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for High Speed"]
    #[inline]
    pub fn hspclkgenval(&self) -> HSPCLKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HSPCLKGENVALR { bits }
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for High Speed"]
    #[inline]
    pub fn hspdrvstval(&self) -> HSPDRVSTVALR {
        HSPDRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR12"]
    #[inline]
    pub fn sdr12sdclkfreqval(&self) -> SDR12SDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SDR12SDCLKFREQVALR { bits }
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR12"]
    #[inline]
    pub fn sdr12clkgenval(&self) -> SDR12CLKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDR12CLKGENVALR { bits }
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR12"]
    #[inline]
    pub fn sdr12drvstval(&self) -> SDR12DRVSTVALR {
        SDR12DRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
