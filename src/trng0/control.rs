#[doc = "Reader of register CONTROL"]
pub type R = crate::R<u32, super::CONTROL>;
#[doc = "Writer for register CONTROL"]
pub type W = crate::W<u32, super::CONTROL>;
#[doc = "Register CONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `TESTEN`"]
pub type TESTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TESTEN`"]
pub struct TESTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TESTEN_W<'a> {
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
#[doc = "Reader of field `CONDBYPASS`"]
pub type CONDBYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONDBYPASS`"]
pub struct CONDBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONDBYPASS_W<'a> {
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
#[doc = "Reader of field `REPCOUNTIEN`"]
pub type REPCOUNTIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REPCOUNTIEN`"]
pub struct REPCOUNTIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REPCOUNTIEN_W<'a> {
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
#[doc = "Reader of field `APT64IEN`"]
pub type APT64IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APT64IEN`"]
pub struct APT64IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APT64IEN_W<'a> {
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
#[doc = "Reader of field `APT4096IEN`"]
pub type APT4096IEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APT4096IEN`"]
pub struct APT4096IEN_W<'a> {
    w: &'a mut W,
}
impl<'a> APT4096IEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FULLIEN`"]
pub type FULLIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULLIEN`"]
pub struct FULLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLIEN_W<'a> {
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
#[doc = "Reader of field `SOFTRESET`"]
pub type SOFTRESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOFTRESET`"]
pub struct SOFTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PREIEN`"]
pub type PREIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREIEN`"]
pub struct PREIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PREIEN_W<'a> {
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
#[doc = "Reader of field `ALMIEN`"]
pub type ALMIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALMIEN`"]
pub struct ALMIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALMIEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `FORCERUN`"]
pub type FORCERUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCERUN`"]
pub struct FORCERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCERUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `BYPNIST`"]
pub type BYPNIST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPNIST`"]
pub struct BYPNIST_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPNIST_W<'a> {
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
#[doc = "Reader of field `BYPAIS31`"]
pub type BYPAIS31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPAIS31`"]
pub struct BYPAIS31_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPAIS31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&self) -> CONDBYPASS_R {
        CONDBYPASS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    pub fn repcountien(&self) -> REPCOUNTIEN_R {
        REPCOUNTIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    pub fn apt64ien(&self) -> APT64IEN_R {
        APT64IEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    pub fn apt4096ien(&self) -> APT4096IEN_R {
        APT4096IEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    pub fn fullien(&self) -> FULLIEN_R {
        FULLIEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    pub fn preien(&self) -> PREIEN_R {
        PREIEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&self) -> ALMIEN_R {
        ALMIEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&self) -> FORCERUN_R {
        FORCERUN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&self) -> BYPNIST_R {
        BYPNIST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&self) -> BYPAIS31_R {
        BYPAIS31_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TRNG Module Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Test Enable"]
    #[inline(always)]
    pub fn testen(&mut self) -> TESTEN_W {
        TESTEN_W { w: self }
    }
    #[doc = "Bit 3 - Conditioning Bypass"]
    #[inline(always)]
    pub fn condbypass(&mut self) -> CONDBYPASS_W {
        CONDBYPASS_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt Enable for Repetition Count Test Failure"]
    #[inline(always)]
    pub fn repcountien(&mut self) -> REPCOUNTIEN_W {
        REPCOUNTIEN_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Enable for Adaptive Proportion Test Failure (64-sample Window)"]
    #[inline(always)]
    pub fn apt64ien(&mut self) -> APT64IEN_W {
        APT64IEN_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt Enable for Adaptive Proportion Test Failure (4096-sample Window)"]
    #[inline(always)]
    pub fn apt4096ien(&mut self) -> APT4096IEN_W {
        APT4096IEN_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt Enable for FIFO Full"]
    #[inline(always)]
    pub fn fullien(&mut self) -> FULLIEN_W {
        FULLIEN_W { w: self }
    }
    #[doc = "Bit 8 - Software Reset"]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt enable for AIS31 preliminary noise alarm"]
    #[inline(always)]
    pub fn preien(&mut self) -> PREIEN_W {
        PREIEN_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt enable for AIS31 noise alarm"]
    #[inline(always)]
    pub fn almien(&mut self) -> ALMIEN_W {
        ALMIEN_W { w: self }
    }
    #[doc = "Bit 11 - Oscillator Force Run"]
    #[inline(always)]
    pub fn forcerun(&mut self) -> FORCERUN_W {
        FORCERUN_W { w: self }
    }
    #[doc = "Bit 12 - NIST Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypnist(&mut self) -> BYPNIST_W {
        BYPNIST_W { w: self }
    }
    #[doc = "Bit 13 - AIS31 Start-up Test Bypass."]
    #[inline(always)]
    pub fn bypais31(&mut self) -> BYPAIS31_W {
        BYPAIS31_W { w: self }
    }
}
