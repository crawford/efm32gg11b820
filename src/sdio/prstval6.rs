#[doc = "Reader of register PRSTVAL6"]
pub type R = crate::R<u32, super::PRSTVAL6>;
#[doc = "Reader of field `SDR104SDCLKFREQVAL`"]
pub type SDR104SDCLKFREQVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `SDR104CLKGENVAL`"]
pub type SDR104CLKGENVAL_R = crate::R<bool, bool>;
#[doc = "Driver Strength Select Value for SDR104\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR104DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "1: Driver Type A is selected"]
    TYPEA,
    #[doc = "2: Driver Type C is selected"]
    TYPEC,
    #[doc = "3: Driver Type D is selected"]
    TYPED,
}
impl From<SDR104DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: SDR104DRVSTVAL_A) -> Self {
        match variant {
            SDR104DRVSTVAL_A::TYPEB => 0,
            SDR104DRVSTVAL_A::TYPEA => 1,
            SDR104DRVSTVAL_A::TYPEC => 2,
            SDR104DRVSTVAL_A::TYPED => 3,
        }
    }
}
#[doc = "Reader of field `SDR104DRVSTVAL`"]
pub type SDR104DRVSTVAL_R = crate::R<u8, SDR104DRVSTVAL_A>;
impl SDR104DRVSTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR104DRVSTVAL_A {
        match self.bits {
            0 => SDR104DRVSTVAL_A::TYPEB,
            1 => SDR104DRVSTVAL_A::TYPEA,
            2 => SDR104DRVSTVAL_A::TYPEC,
            3 => SDR104DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR104DRVSTVAL_A::TYPED
    }
}
#[doc = "Reader of field `DDR50SDCLKFREQVAL`"]
pub type DDR50SDCLKFREQVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `DDR50CLKGENVAL`"]
pub type DDR50CLKGENVAL_R = crate::R<bool, bool>;
#[doc = "Driver Strength Select Value for DDR50\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDR50DRVSTVAL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "1: Driver Type A is selected"]
    TYPEA,
    #[doc = "2: Driver Type C is selected"]
    TYPEC,
    #[doc = "3: Driver Type D is selected"]
    TYPED,
}
impl From<DDR50DRVSTVAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DDR50DRVSTVAL_A) -> Self {
        match variant {
            DDR50DRVSTVAL_A::TYPEB => 0,
            DDR50DRVSTVAL_A::TYPEA => 1,
            DDR50DRVSTVAL_A::TYPEC => 2,
            DDR50DRVSTVAL_A::TYPED => 3,
        }
    }
}
#[doc = "Reader of field `DDR50DRVSTVAL`"]
pub type DDR50DRVSTVAL_R = crate::R<u8, DDR50DRVSTVAL_A>;
impl DDR50DRVSTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDR50DRVSTVAL_A {
        match self.bits {
            0 => DDR50DRVSTVAL_A::TYPEB,
            1 => DDR50DRVSTVAL_A::TYPEA,
            2 => DDR50DRVSTVAL_A::TYPEC,
            3 => DDR50DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == DDR50DRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104sdclkfreqval(&self) -> SDR104SDCLKFREQVAL_R {
        SDR104SDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104clkgenval(&self) -> SDR104CLKGENVAL_R {
        SDR104CLKGENVAL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for SDR104"]
    #[inline(always)]
    pub fn sdr104drvstval(&self) -> SDR104DRVSTVAL_R {
        SDR104DRVSTVAL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50sdclkfreqval(&self) -> DDR50SDCLKFREQVAL_R {
        DDR50SDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50clkgenval(&self) -> DDR50CLKGENVAL_R {
        DDR50CLKGENVAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for DDR50"]
    #[inline(always)]
    pub fn ddr50drvstval(&self) -> DDR50DRVSTVAL_R {
        DDR50DRVSTVAL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
