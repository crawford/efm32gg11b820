#[doc = "Reader of register R5VADCCTRL"]
pub type R = crate::R<u32, super::R5VADCCTRL>;
#[doc = "Writer for register R5VADCCTRL"]
pub type W = crate::W<u32, super::R5VADCCTRL>;
#[doc = "Register R5VADCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::R5VADCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENAMUX`"]
pub type ENAMUX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAMUX`"]
pub struct ENAMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAMUX_W<'a> {
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
#[doc = "ADC Mux Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMUXSEL_A {
    #[doc = "0: VBUS divided by 10"]
    VBUSDIV10 = 0,
    #[doc = "1: VREGI divided by 10"]
    VREGIDIV10 = 1,
    #[doc = "2: VREGO divided by 6"]
    VREGODIV6 = 2,
    #[doc = "3: VREGI current monitor"]
    VREGIIMON = 3,
    #[doc = "4: VBUS current monitor"]
    VBUSIMON = 4,
}
impl From<AMUXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: AMUXSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AMUXSEL`"]
pub type AMUXSEL_R = crate::R<u8, AMUXSEL_A>;
impl AMUXSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, AMUXSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(AMUXSEL_A::VBUSDIV10),
            1 => Val(AMUXSEL_A::VREGIDIV10),
            2 => Val(AMUXSEL_A::VREGODIV6),
            3 => Val(AMUXSEL_A::VREGIIMON),
            4 => Val(AMUXSEL_A::VBUSIMON),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VBUSDIV10`"]
    #[inline(always)]
    pub fn is_vbusdiv10(&self) -> bool {
        *self == AMUXSEL_A::VBUSDIV10
    }
    #[doc = "Checks if the value of the field is `VREGIDIV10`"]
    #[inline(always)]
    pub fn is_vregidiv10(&self) -> bool {
        *self == AMUXSEL_A::VREGIDIV10
    }
    #[doc = "Checks if the value of the field is `VREGODIV6`"]
    #[inline(always)]
    pub fn is_vregodiv6(&self) -> bool {
        *self == AMUXSEL_A::VREGODIV6
    }
    #[doc = "Checks if the value of the field is `VREGIIMON`"]
    #[inline(always)]
    pub fn is_vregiimon(&self) -> bool {
        *self == AMUXSEL_A::VREGIIMON
    }
    #[doc = "Checks if the value of the field is `VBUSIMON`"]
    #[inline(always)]
    pub fn is_vbusimon(&self) -> bool {
        *self == AMUXSEL_A::VBUSIMON
    }
}
#[doc = "Write proxy for field `AMUXSEL`"]
pub struct AMUXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> AMUXSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMUXSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "VBUS divided by 10"]
    #[inline(always)]
    pub fn vbusdiv10(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VBUSDIV10)
    }
    #[doc = "VREGI divided by 10"]
    #[inline(always)]
    pub fn vregidiv10(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VREGIDIV10)
    }
    #[doc = "VREGO divided by 6"]
    #[inline(always)]
    pub fn vregodiv6(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VREGODIV6)
    }
    #[doc = "VREGI current monitor"]
    #[inline(always)]
    pub fn vregiimon(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VREGIIMON)
    }
    #[doc = "VBUS current monitor"]
    #[inline(always)]
    pub fn vbusimon(self) -> &'a mut W {
        self.variant(AMUXSEL_A::VBUSIMON)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&self) -> ENAMUX_R {
        ENAMUX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&self) -> AMUXSEL_R {
        AMUXSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the 5V Subsystem ADC MUX"]
    #[inline(always)]
    pub fn enamux(&mut self) -> ENAMUX_W {
        ENAMUX_W { w: self }
    }
    #[doc = "Bits 12:15 - ADC Mux Selection"]
    #[inline(always)]
    pub fn amuxsel(&mut self) -> AMUXSEL_W {
        AMUXSEL_W { w: self }
    }
}
