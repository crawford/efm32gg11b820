#[doc = "Reader of register HFPERPRESC"]
pub type R = crate::R<u32, super::HFPERPRESC>;
#[doc = "Writer for register HFPERPRESC"]
pub type W = crate::W<u32, super::HFPERPRESC>;
#[doc = "Register HFPERPRESC `reset()`'s with value 0"]
impl crate::ResetValue for super::HFPERPRESC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "HFPERCLK Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION,
}
impl From<PRESC_A> for u16 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        match variant {
            PRESC_A::NODIVISION => 0,
        }
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u16, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 8)) | (((value as u32) & 0x01ff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:16 - HFPERCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:16 - HFPERCLK Prescaler"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
}
