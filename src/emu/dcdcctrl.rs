#[doc = "Reader of register DCDCCTRL"]
pub type R = crate::R<u32, super::DCDCCTRL>;
#[doc = "Writer for register DCDCCTRL"]
pub type W = crate::W<u32, super::DCDCCTRL>;
#[doc = "Register DCDCCTRL `reset()`'s with value 0x33"]
impl crate::ResetValue for super::DCDCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x33
    }
}
#[doc = "Regulator Mode\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCDCMODE_A {
    #[doc = "0: DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    BYPASS = 0,
    #[doc = "1: DCDC regulator is operating in low noise mode."]
    LOWNOISE = 1,
    #[doc = "2: DCDC regulator is operating in low power mode."]
    LOWPOWER = 2,
    #[doc = "3: DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    OFF = 3,
}
impl From<DCDCMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDCMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCDCMODE`"]
pub type DCDCMODE_R = crate::R<u8, DCDCMODE_A>;
impl DCDCMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDCMODE_A {
        match self.bits {
            0 => DCDCMODE_A::BYPASS,
            1 => DCDCMODE_A::LOWNOISE,
            2 => DCDCMODE_A::LOWPOWER,
            3 => DCDCMODE_A::OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == DCDCMODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `LOWNOISE`"]
    #[inline(always)]
    pub fn is_lownoise(&self) -> bool {
        *self == DCDCMODE_A::LOWNOISE
    }
    #[doc = "Checks if the value of the field is `LOWPOWER`"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == DCDCMODE_A::LOWPOWER
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DCDCMODE_A::OFF
    }
}
#[doc = "Write proxy for field `DCDCMODE`"]
pub struct DCDCMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDCMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "DCDC regulator is operating in bypass mode. Prior to configuring DCDCMODE=BYPASS, software must set EMU_DCDCCLIMCTRL.BYPLIMEN=1 to prevent excessive current between VREGVDD and DVDD supplies."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(DCDCMODE_A::BYPASS)
    }
    #[doc = "DCDC regulator is operating in low noise mode."]
    #[inline(always)]
    pub fn lownoise(self) -> &'a mut W {
        self.variant(DCDCMODE_A::LOWNOISE)
    }
    #[doc = "DCDC regulator is operating in low power mode."]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut W {
        self.variant(DCDCMODE_A::LOWPOWER)
    }
    #[doc = "DCDC regulator is off and the bypass switch is off. Note: DVDD must be supplied externally"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(DCDCMODE_A::OFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DCDCMODEEM23`"]
pub type DCDCMODEEM23_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDCMODEEM23`"]
pub struct DCDCMODEEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCMODEEM23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DCDCMODEEM4`"]
pub type DCDCMODEEM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCDCMODEEM4`"]
pub struct DCDCMODEEM4_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCMODEEM4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    pub fn dcdcmode(&self) -> DCDCMODE_R {
        DCDCMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    pub fn dcdcmodeem23(&self) -> DCDCMODEEM23_R {
        DCDCMODEEM23_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    pub fn dcdcmodeem4(&self) -> DCDCMODEEM4_R {
        DCDCMODEEM4_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Regulator Mode"]
    #[inline(always)]
    pub fn dcdcmode(&mut self) -> DCDCMODE_W {
        DCDCMODE_W { w: self }
    }
    #[doc = "Bit 4 - DCDC Mode EM23"]
    #[inline(always)]
    pub fn dcdcmodeem23(&mut self) -> DCDCMODEEM23_W {
        DCDCMODEEM23_W { w: self }
    }
    #[doc = "Bit 5 - DCDC Mode EM4H"]
    #[inline(always)]
    pub fn dcdcmodeem4(&mut self) -> DCDCMODEEM4_W {
        DCDCMODEEM4_W { w: self }
    }
}
