#[doc = "Reader of register PRSTVAL4"]
pub type R = crate::R<u32, super::PRSTVAL4>;
#[doc = "Reader of field `SDR25SDCLKFREQVAL`"]
pub type SDR25SDCLKFREQVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `SDR25CLKGENVAL`"]
pub type SDR25CLKGENVAL_R = crate::R<bool, bool>;
#[doc = "Driver Strength Select Value for SDR25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDR25DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<SDR25DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR25DRVSTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDR25DRVSTVAL`"]
pub type SDR25DRVSTVAL_R = crate::R<u8, SDR25DRVSTVAL_A>;
impl SDR25DRVSTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR25DRVSTVAL_A {
        match self.bits {
            0 => SDR25DRVSTVAL_A::TYPEB,
            1 => SDR25DRVSTVAL_A::TYPEA,
            2 => SDR25DRVSTVAL_A::TYPEC,
            3 => SDR25DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR25DRVSTVAL_A::TYPED
    }
}
#[doc = "Reader of field `SDR50SDCLKFREQVAL`"]
pub type SDR50SDCLKFREQVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `SDR50CLCKGENVAL`"]
pub type SDR50CLCKGENVAL_R = crate::R<bool, bool>;
#[doc = "Driver Strength Select Value for SDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SDR50DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB = 0,
    #[doc = "1: Driver Type A is selected"]
    TYPEA = 1,
    #[doc = "2: Driver Type C is selected"]
    TYPEC = 2,
    #[doc = "3: Driver Type D is selected"]
    TYPED = 3,
}
impl From<SDR50DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR50DRVSTVAL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SDR50DRVSTVAL`"]
pub type SDR50DRVSTVAL_R = crate::R<u8, SDR50DRVSTVAL_A>;
impl SDR50DRVSTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR50DRVSTVAL_A {
        match self.bits {
            0 => SDR50DRVSTVAL_A::TYPEB,
            1 => SDR50DRVSTVAL_A::TYPEA,
            2 => SDR50DRVSTVAL_A::TYPEC,
            3 => SDR50DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR50DRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25sdclkfreqval(&self) -> SDR25SDCLKFREQVAL_R {
        SDR25SDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25clkgenval(&self) -> SDR25CLKGENVAL_R {
        SDR25CLKGENVAL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR25"]
    #[inline(always)]
    pub fn sdr25drvstval(&self) -> SDR25DRVSTVAL_R {
        SDR25DRVSTVAL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50sdclkfreqval(&self) -> SDR50SDCLKFREQVAL_R {
        SDR50SDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50clckgenval(&self) -> SDR50CLCKGENVAL_R {
        SDR50CLCKGENVAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR50"]
    #[inline(always)]
    pub fn sdr50drvstval(&self) -> SDR50DRVSTVAL_R {
        SDR50DRVSTVAL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
