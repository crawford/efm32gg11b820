#[doc = "Reader of register BIASCTRL"]
pub type R = crate::R<u32, super::BIASCTRL>;
#[doc = "Writer for register BIASCTRL"]
pub type W = crate::W<u32, super::BIASCTRL>;
#[doc = "Register BIASCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BIASCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `BIASMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASMODE_A {
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE"]
    DONTTOUCH,
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    DUTYCYCLE,
    #[doc = "Bias module always in high accuracy mode"]
    HIGHACC,
}
impl crate::ToBits<u8> for BIASMODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BIASMODE_A::DONTTOUCH => 0,
            BIASMODE_A::DUTYCYCLE => 1,
            BIASMODE_A::HIGHACC => 2,
        }
    }
}
#[doc = "Reader of field `BIASMODE`"]
pub type BIASMODE_R = crate::R<u8, BIASMODE_A>;
impl BIASMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BIASMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BIASMODE_A::DONTTOUCH),
            1 => Val(BIASMODE_A::DUTYCYCLE),
            2 => Val(BIASMODE_A::HIGHACC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DONTTOUCH`"]
    #[inline(always)]
    pub fn is_donttouch(&self) -> bool {
        *self == BIASMODE_A::DONTTOUCH
    }
    #[doc = "Checks if the value of the field is `DUTYCYCLE`"]
    #[inline(always)]
    pub fn is_dutycycle(&self) -> bool {
        *self == BIASMODE_A::DUTYCYCLE
    }
    #[doc = "Checks if the value of the field is `HIGHACC`"]
    #[inline(always)]
    pub fn is_highacc(&self) -> bool {
        *self == BIASMODE_A::HIGHACC
    }
}
#[doc = "Write proxy for field `BIASMODE`"]
pub struct BIASMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIASMODE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Bias module is controlled by the EMU and is not affected by LESENSE"]
    #[inline(always)]
    pub fn donttouch(self) -> &'a mut W {
        self.variant(BIASMODE_A::DONTTOUCH)
    }
    #[doc = "Bias module duty cycled between low power and high accuracy mode"]
    #[inline(always)]
    pub fn dutycycle(self) -> &'a mut W {
        self.variant(BIASMODE_A::DUTYCYCLE)
    }
    #[doc = "Bias module always in high accuracy mode"]
    #[inline(always)]
    pub fn highacc(self) -> &'a mut W {
        self.variant(BIASMODE_A::HIGHACC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select Bias Mode"]
    #[inline(always)]
    pub fn biasmode(&self) -> BIASMODE_R {
        BIASMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select Bias Mode"]
    #[inline(always)]
    pub fn biasmode(&mut self) -> BIASMODE_W {
        BIASMODE_W { w: self }
    }
}
