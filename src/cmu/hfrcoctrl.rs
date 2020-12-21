#[doc = "Reader of register HFRCOCTRL"]
pub type R = crate::R<u32, super::HFRCOCTRL>;
#[doc = "Writer for register HFRCOCTRL"]
pub type W = crate::W<u32, super::HFRCOCTRL>;
#[doc = "Register HFRCOCTRL `reset()`'s with value 0xb148_1f7f"]
impl crate::ResetValue for super::HFRCOCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xb148_1f7f
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = "Reader of field `FINETUNING`"]
pub type FINETUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FINETUNING`"]
pub struct FINETUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FREQRANGE`"]
pub type FREQRANGE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQRANGE`"]
pub struct FREQRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQRANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMPBIAS`"]
pub type CMPBIAS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPBIAS`"]
pub struct CMPBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `LDOHP`"]
pub type LDOHP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LDOHP`"]
pub struct LDOHP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDOHP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Locally Divide HFRCO Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, CLKDIV_A>;
impl CLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKDIV_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKDIV_A::DIV1),
            1 => Val(CLKDIV_A::DIV2),
            2 => Val(CLKDIV_A::DIV4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIV_A::DIV4
    }
}
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `FINETUNINGEN`"]
pub type FINETUNINGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FINETUNINGEN`"]
pub struct FINETUNINGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETUNINGEN_W<'a> {
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
#[doc = "Reader of field `VREFTC`"]
pub type VREFTC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VREFTC`"]
pub struct VREFTC_W<'a> {
    w: &'a mut W,
}
impl<'a> VREFTC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - HFRCO Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - HFRCO Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - HFRCO Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CMPBIAS_R {
        CMPBIAS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - HFRCO LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&self) -> LDOHP_R {
        LDOHP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Locally Divide HFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline(always)]
    pub fn finetuningen(&self) -> FINETUNINGEN_R {
        FINETUNINGEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - HFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline(always)]
    pub fn vreftc(&self) -> VREFTC_R {
        VREFTC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bits 8:13 - HFRCO Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&mut self) -> FINETUNING_W {
        FINETUNING_W { w: self }
    }
    #[doc = "Bits 16:20 - HFRCO Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FREQRANGE_W {
        FREQRANGE_W { w: self }
    }
    #[doc = "Bits 21:23 - HFRCO Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&mut self) -> CMPBIAS_W {
        CMPBIAS_W { w: self }
    }
    #[doc = "Bit 24 - HFRCO LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&mut self) -> LDOHP_W {
        LDOHP_W { w: self }
    }
    #[doc = "Bits 25:26 - Locally Divide HFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline(always)]
    pub fn finetuningen(&mut self) -> FINETUNINGEN_W {
        FINETUNINGEN_W { w: self }
    }
    #[doc = "Bits 28:31 - HFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline(always)]
    pub fn vreftc(&mut self) -> VREFTC_W {
        VREFTC_W { w: self }
    }
}
