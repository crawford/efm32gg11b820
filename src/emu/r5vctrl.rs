#[doc = "Reader of register R5VCTRL"]
pub type R = crate::R<u32, super::R5VCTRL>;
#[doc = "Writer for register R5VCTRL"]
pub type W = crate::W<u32, super::R5VCTRL>;
#[doc = "Register R5VCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::R5VCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `EM4WUEN`"]
pub type EM4WUEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4WUEN`"]
pub struct EM4WUEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WUEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `IMONEN`"]
pub type IMONEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IMONEN`"]
pub struct IMONEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IMONEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "5V Input Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUTMODE_A {
    #[doc = "0: Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    AUTO,
    #[doc = "1: Force VBUS pin as the regulator input"]
    VBUS,
    #[doc = "2: Force VREGI pin as the regulator input"]
    VREGI,
}
impl From<INPUTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUTMODE_A) -> Self {
        match variant {
            INPUTMODE_A::AUTO => 0,
            INPUTMODE_A::VBUS => 1,
            INPUTMODE_A::VREGI => 2,
        }
    }
}
#[doc = "Reader of field `INPUTMODE`"]
pub type INPUTMODE_R = crate::R<u8, INPUTMODE_A>;
impl INPUTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUTMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUTMODE_A::AUTO),
            1 => Val(INPUTMODE_A::VBUS),
            2 => Val(INPUTMODE_A::VREGI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == INPUTMODE_A::AUTO
    }
    #[doc = "Checks if the value of the field is `VBUS`"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == INPUTMODE_A::VBUS
    }
    #[doc = "Checks if the value of the field is `VREGI`"]
    #[inline(always)]
    pub fn is_vregi(&self) -> bool {
        *self == INPUTMODE_A::VREGI
    }
}
#[doc = "Write proxy for field `INPUTMODE`"]
pub struct INPUTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUTMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut W {
        self.variant(INPUTMODE_A::AUTO)
    }
    #[doc = "Force VBUS pin as the regulator input"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut W {
        self.variant(INPUTMODE_A::VBUS)
    }
    #[doc = "Force VREGI pin as the regulator input"]
    #[inline(always)]
    pub fn vregi(self) -> &'a mut W {
        self.variant(INPUTMODE_A::VREGI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline(always)]
    pub fn imonen(&self) -> IMONEN_R {
        IMONEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline(always)]
    pub fn inputmode(&self) -> INPUTMODE_R {
        INPUTMODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> EM4WUEN_W {
        EM4WUEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline(always)]
    pub fn imonen(&mut self) -> IMONEN_W {
        IMONEN_W { w: self }
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline(always)]
    pub fn inputmode(&mut self) -> INPUTMODE_W {
        INPUTMODE_W { w: self }
    }
}
