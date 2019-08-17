#[doc = "Reader of register PRSTVAL2"]
pub type R = crate::R<u32, super::PRSTVAL2>;
#[doc = "Reader of field `HSPSDCLKFREQVAL`"]
pub type HSPSDCLKFREQVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `HSPCLKGENVAL`"]
pub type HSPCLKGENVAL_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `HSPDRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSPDRVSTVAL_A {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl crate::ToBits<u8> for HSPDRVSTVAL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            HSPDRVSTVAL_A::TYPEB => 0,
            HSPDRVSTVAL_A::TYPEA => 1,
            HSPDRVSTVAL_A::TYPEC => 2,
            HSPDRVSTVAL_A::TYPED => 3,
        }
    }
}
#[doc = "Reader of field `HSPDRVSTVAL`"]
pub type HSPDRVSTVAL_R = crate::R<u8, HSPDRVSTVAL_A>;
impl HSPDRVSTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSPDRVSTVAL_A {
        match self.bits {
            0 => HSPDRVSTVAL_A::TYPEB,
            1 => HSPDRVSTVAL_A::TYPEA,
            2 => HSPDRVSTVAL_A::TYPEC,
            3 => HSPDRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == HSPDRVSTVAL_A::TYPED
    }
}
#[doc = "Reader of field `SDR12SDCLKFREQVAL`"]
pub type SDR12SDCLKFREQVAL_R = crate::R<u16, u16>;
#[doc = "Reader of field `SDR12CLKGENVAL`"]
pub type SDR12CLKGENVAL_R = crate::R<bool, bool>;
#[doc = "Possible values of the field `SDR12DRVSTVAL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDR12DRVSTVAL_A {
    #[doc = "Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "Driver Type A is selected"]
    TYPEA,
    #[doc = "Driver Type C is selected"]
    TYPEC,
    #[doc = "Driver Type D is selected"]
    TYPED,
}
impl crate::ToBits<u8> for SDR12DRVSTVAL_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SDR12DRVSTVAL_A::TYPEB => 0,
            SDR12DRVSTVAL_A::TYPEA => 1,
            SDR12DRVSTVAL_A::TYPEC => 2,
            SDR12DRVSTVAL_A::TYPED => 3,
        }
    }
}
#[doc = "Reader of field `SDR12DRVSTVAL`"]
pub type SDR12DRVSTVAL_R = crate::R<u8, SDR12DRVSTVAL_A>;
impl SDR12DRVSTVAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDR12DRVSTVAL_A {
        match self.bits {
            0 => SDR12DRVSTVAL_A::TYPEB,
            1 => SDR12DRVSTVAL_A::TYPEA,
            2 => SDR12DRVSTVAL_A::TYPEC,
            3 => SDR12DRVSTVAL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == SDR12DRVSTVAL_A::TYPED
    }
}
impl R {
    #[doc = "Bits 0:9 - SD_CLK Frequency Select Value for High Speed"]
    #[inline(always)]
    pub fn hspsdclkfreqval(&self) -> HSPSDCLKFREQVAL_R {
        HSPSDCLKFREQVAL_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Clock Generator Select Value for High Speed"]
    #[inline(always)]
    pub fn hspclkgenval(&self) -> HSPCLKGENVAL_R {
        HSPCLKGENVAL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Driver Strength Select Value for High Speed"]
    #[inline(always)]
    pub fn hspdrvstval(&self) -> HSPDRVSTVAL_R {
        HSPDRVSTVAL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25 - SD_CLK Frequency Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12sdclkfreqval(&self) -> SDR12SDCLKFREQVAL_R {
        SDR12SDCLKFREQVAL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Clock Generator Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12clkgenval(&self) -> SDR12CLKGENVAL_R {
        SDR12CLKGENVAL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Driver Strength Select Value for SDR12"]
    #[inline(always)]
    pub fn sdr12drvstval(&self) -> SDR12DRVSTVAL_R {
        SDR12DRVSTVAL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
