#[doc = "Reader of register BIASPROG"]
pub type R = crate::R<u32, super::BIASPROG>;
#[doc = "Writer for register BIASPROG"]
pub type W = crate::W<u32, super::BIASPROG>;
#[doc = "Register BIASPROG `reset()`'s with value 0"]
impl crate::ResetValue for super::BIASPROG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bias Programming Value of Analog ADC Block\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCBIASPROG_A {
    #[doc = "0: Normal power (use for 1Msps operation)"]
    NORMAL = 0,
    #[doc = "4: Scaling bias to 1/2"]
    SCALE2 = 4,
    #[doc = "8: Scaling bias to 1/4"]
    SCALE4 = 8,
    #[doc = "12: Scaling bias to 1/8"]
    SCALE8 = 12,
    #[doc = "14: Scaling bias to 1/16"]
    SCALE16 = 14,
    #[doc = "15: Scaling bias to 1/32"]
    SCALE32 = 15,
}
impl From<ADCBIASPROG_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCBIASPROG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCBIASPROG`"]
pub type ADCBIASPROG_R = crate::R<u8, ADCBIASPROG_A>;
impl ADCBIASPROG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCBIASPROG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCBIASPROG_A::NORMAL),
            4 => Val(ADCBIASPROG_A::SCALE2),
            8 => Val(ADCBIASPROG_A::SCALE4),
            12 => Val(ADCBIASPROG_A::SCALE8),
            14 => Val(ADCBIASPROG_A::SCALE16),
            15 => Val(ADCBIASPROG_A::SCALE32),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ADCBIASPROG_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SCALE2`"]
    #[inline(always)]
    pub fn is_scale2(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE2
    }
    #[doc = "Checks if the value of the field is `SCALE4`"]
    #[inline(always)]
    pub fn is_scale4(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE4
    }
    #[doc = "Checks if the value of the field is `SCALE8`"]
    #[inline(always)]
    pub fn is_scale8(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE8
    }
    #[doc = "Checks if the value of the field is `SCALE16`"]
    #[inline(always)]
    pub fn is_scale16(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE16
    }
    #[doc = "Checks if the value of the field is `SCALE32`"]
    #[inline(always)]
    pub fn is_scale32(&self) -> bool {
        *self == ADCBIASPROG_A::SCALE32
    }
}
#[doc = "Write proxy for field `ADCBIASPROG`"]
pub struct ADCBIASPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCBIASPROG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCBIASPROG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal power (use for 1Msps operation)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::NORMAL)
    }
    #[doc = "Scaling bias to 1/2"]
    #[inline(always)]
    pub fn scale2(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE2)
    }
    #[doc = "Scaling bias to 1/4"]
    #[inline(always)]
    pub fn scale4(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE4)
    }
    #[doc = "Scaling bias to 1/8"]
    #[inline(always)]
    pub fn scale8(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE8)
    }
    #[doc = "Scaling bias to 1/16"]
    #[inline(always)]
    pub fn scale16(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE16)
    }
    #[doc = "Scaling bias to 1/32"]
    #[inline(always)]
    pub fn scale32(self) -> &'a mut W {
        self.variant(ADCBIASPROG_A::SCALE32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `VFAULTCLR`"]
pub type VFAULTCLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VFAULTCLR`"]
pub struct VFAULTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> VFAULTCLR_W<'a> {
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
#[doc = "Reader of field `GPBIASACC`"]
pub type GPBIASACC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPBIASACC`"]
pub struct GPBIASACC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPBIASACC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    pub fn adcbiasprog(&self) -> ADCBIASPROG_R {
        ADCBIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    pub fn vfaultclr(&self) -> VFAULTCLR_R {
        VFAULTCLR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    pub fn gpbiasacc(&self) -> GPBIASACC_R {
        GPBIASACC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value of Analog ADC Block"]
    #[inline(always)]
    pub fn adcbiasprog(&mut self) -> ADCBIASPROG_W {
        ADCBIASPROG_W { w: self }
    }
    #[doc = "Bit 12 - Clear VREFOF Flag"]
    #[inline(always)]
    pub fn vfaultclr(&mut self) -> VFAULTCLR_W {
        VFAULTCLR_W { w: self }
    }
    #[doc = "Bit 16 - Accuracy Setting for the System Bias During ADC Operation"]
    #[inline(always)]
    pub fn gpbiasacc(&mut self) -> GPBIASACC_W {
        GPBIASACC_W { w: self }
    }
}
