#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `VBUSENAP`"]
pub type VBUSENAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBUSENAP`"]
pub struct VBUSENAP_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUSENAP_W<'a> {
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
#[doc = "Reader of field `SELFPOWERED`"]
pub type SELFPOWERED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELFPOWERED`"]
pub struct SELFPOWERED_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFPOWERED_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Low Energy Mode Oscillator Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEMOSCCTRL_A {
    #[doc = "0: Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE = 0,
    #[doc = "1: The USBC clock is gated when Low Energy Mode is active."]
    GATE = 1,
}
impl From<LEMOSCCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEMOSCCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LEMOSCCTRL`"]
pub type LEMOSCCTRL_R = crate::R<u8, LEMOSCCTRL_A>;
impl LEMOSCCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LEMOSCCTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LEMOSCCTRL_A::NONE),
            1 => Val(LEMOSCCTRL_A::GATE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == LEMOSCCTRL_A::NONE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == LEMOSCCTRL_A::GATE
    }
}
#[doc = "Write proxy for field `LEMOSCCTRL`"]
pub struct LEMOSCCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMOSCCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEMOSCCTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::NONE)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::GATE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `LEMPHYCTRL`"]
pub type LEMPHYCTRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEMPHYCTRL`"]
pub struct LEMPHYCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMPHYCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `LEMIDLEEN`"]
pub type LEMIDLEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEMIDLEEN`"]
pub struct LEMIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMIDLEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `IDCDEN`"]
pub type IDCDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDCDEN`"]
pub struct IDCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `OTGCLKCDIS`"]
pub type OTGCLKCDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGCLKCDIS`"]
pub struct OTGCLKCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGCLKCDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `OTGIDINDIS`"]
pub type OTGIDINDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGIDINDIS`"]
pub struct OTGIDINDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGIDINDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `OTGPHYCTRLDIS`"]
pub type OTGPHYCTRLDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OTGPHYCTRLDIS`"]
pub struct OTGPHYCTRLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OTGPHYCTRLDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Data Contact Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DCDEN_A {
    #[doc = "0: DCD is disabled."]
    DISABLED = 0,
    #[doc = "2: Only DCD timeout will be initiated."]
    TIMEOUT = 2,
    #[doc = "3: Full DCD operation (physical contact and timeout) will be initiated."]
    ENABLED = 3,
}
impl From<DCDEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DCDEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DCDEN`"]
pub type DCDEN_R = crate::R<u8, DCDEN_A>;
impl DCDEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DCDEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DCDEN_A::DISABLED),
            2 => Val(DCDEN_A::TIMEOUT),
            3 => Val(DCDEN_A::ENABLED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DCDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == DCDEN_A::TIMEOUT
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DCDEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `DCDEN`"]
pub struct DCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDEN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DCD is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDEN_A::DISABLED)
    }
    #[doc = "Only DCD timeout will be initiated."]
    #[inline(always)]
    pub fn timeout(self) -> &'a mut W {
        self.variant(DCDEN_A::TIMEOUT)
    }
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDEN_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `PDEN`"]
pub type PDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDEN`"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `SDEN`"]
pub type SDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDEN`"]
pub struct SDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    pub fn vbusenap(&self) -> VBUSENAP_R {
        VBUSENAP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline(always)]
    pub fn selfpowered(&self) -> SELFPOWERED_R {
        SELFPOWERED_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LEMOSCCTRL_R {
        LEMOSCCTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LEMPHYCTRL_R {
        LEMPHYCTRL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LEMIDLEEN_R {
        LEMIDLEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline(always)]
    pub fn idcden(&self) -> IDCDEN_R {
        IDCDEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline(always)]
    pub fn otgclkcdis(&self) -> OTGCLKCDIS_R {
        OTGCLKCDIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline(always)]
    pub fn otgidindis(&self) -> OTGIDINDIS_R {
        OTGIDINDIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline(always)]
    pub fn otgphyctrldis(&self) -> OTGPHYCTRLDIS_R {
        OTGPHYCTRLDIS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline(always)]
    pub fn vbusenap(&mut self) -> VBUSENAP_W {
        VBUSENAP_W { w: self }
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline(always)]
    pub fn selfpowered(&mut self) -> SELFPOWERED_W {
        SELFPOWERED_W { w: self }
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&mut self) -> LEMOSCCTRL_W {
        LEMOSCCTRL_W { w: self }
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&mut self) -> LEMPHYCTRL_W {
        LEMPHYCTRL_W { w: self }
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&mut self) -> LEMIDLEEN_W {
        LEMIDLEEN_W { w: self }
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline(always)]
    pub fn idcden(&mut self) -> IDCDEN_W {
        IDCDEN_W { w: self }
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline(always)]
    pub fn otgclkcdis(&mut self) -> OTGCLKCDIS_W {
        OTGCLKCDIS_W { w: self }
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline(always)]
    pub fn otgidindis(&mut self) -> OTGIDINDIS_W {
        OTGIDINDIS_W { w: self }
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline(always)]
    pub fn otgphyctrldis(&mut self) -> OTGPHYCTRLDIS_W {
        OTGPHYCTRLDIS_W { w: self }
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W {
        DCDEN_W { w: self }
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W {
        SDEN_W { w: self }
    }
}
