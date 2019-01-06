#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::PRSTVAL0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct INITSDCLKFREQVALR {
    bits: u16,
}
impl INITSDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INITCLCKGENVALR {
    bits: bool,
}
impl INITCLCKGENVALR {
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
#[doc = "Possible values of the field `INITDRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl INITDRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INITDRVSTVALR::TYPEB => 0,
            INITDRVSTVALR::TYPEA => 1,
            INITDRVSTVALR::TYPEC => 2,
            INITDRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INITDRVSTVALR {
        match value {
            0 => INITDRVSTVALR::TYPEB,
            1 => INITDRVSTVALR::TYPEA,
            2 => INITDRVSTVALR::TYPEC,
            3 => INITDRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == INITDRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == INITDRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == INITDRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == INITDRVSTVALR::TYPED
    }
}
#[doc = r" Value of the field"]
pub struct DSPSDCLKFREQVALR {
    bits: u16,
}
impl DSPSDCLKFREQVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DSPCLKGENVALR {
    bits: bool,
}
impl DSPCLKGENVALR {
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
#[doc = "Possible values of the field `DSPDRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSPDRVSTVALR {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl DSPDRVSTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DSPDRVSTVALR::TYPEB => 0,
            DSPDRVSTVALR::TYPEA => 1,
            DSPDRVSTVALR::TYPEC => 2,
            DSPDRVSTVALR::TYPED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DSPDRVSTVALR {
        match value {
            0 => DSPDRVSTVALR::TYPEB,
            1 => DSPDRVSTVALR::TYPEA,
            2 => DSPDRVSTVALR::TYPEC,
            3 => DSPDRVSTVALR::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline]
    pub fn is_typeb(&self) -> bool {
        *self == DSPDRVSTVALR::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline]
    pub fn is_typea(&self) -> bool {
        *self == DSPDRVSTVALR::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline]
    pub fn is_typec(&self) -> bool {
        *self == DSPDRVSTVALR::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline]
    pub fn is_typed(&self) -> bool {
        *self == DSPDRVSTVALR::TYPED
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for Initialization"]
    #[inline]
    pub fn initsdclkfreqval(&self) -> INITSDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        INITSDCLKFREQVALR { bits }
    }
    #[doc = "Bit 10 - Clock Generator Select Value for Initialization"]
    #[inline]
    pub fn initclckgenval(&self) -> INITCLCKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INITCLCKGENVALR { bits }
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for Initialization"]
    #[inline]
    pub fn initdrvstval(&self) -> INITDRVSTVALR {
        INITDRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for Default Speed"]
    #[inline]
    pub fn dspsdclkfreqval(&self) -> DSPSDCLKFREQVALR {
        let bits = {
            const MASK: u16 = 1023;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        DSPSDCLKFREQVALR { bits }
    }
    #[doc = "Bit 26 - Clock Generator Select Value for Default Speed"]
    #[inline]
    pub fn dspclkgenval(&self) -> DSPCLKGENVALR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DSPCLKGENVALR { bits }
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for Default Speed"]
    #[inline]
    pub fn dspdrvstval(&self) -> DSPDRVSTVALR {
        DSPDRVSTVALR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
