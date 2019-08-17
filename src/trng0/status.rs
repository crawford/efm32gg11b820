#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Writer for register STATUS"]
pub type W = crate::W<u32, super::STATUS>;
#[doc = "Register STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TESTDATABUSY`"]
pub type TESTDATABUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `REPCOUNTIF`"]
pub type REPCOUNTIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `APT64IF`"]
pub type APT64IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `APT4096IF`"]
pub type APT4096IF_R = crate::R<bool, bool>;
#[doc = "Reader of field `FULLIF`"]
pub type FULLIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PREIF`"]
pub type PREIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREIF`"]
pub struct PREIF_W<'a> {
    w: &'a mut W,
}
impl<'a> PREIF_W<'a> {
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
#[doc = "Reader of field `ALMIF`"]
pub type ALMIF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Test Data Busy"]
    #[inline(always)]
    pub fn testdatabusy(&self) -> TESTDATABUSY_R {
        TESTDATABUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - Repetition Count Test Interrupt Status"]
    #[inline(always)]
    pub fn repcountif(&self) -> REPCOUNTIF_R {
        REPCOUNTIF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Adaptive Proportion test failure (64-sample window) interrupt status"]
    #[inline(always)]
    pub fn apt64if(&self) -> APT64IF_R {
        APT64IF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Adaptive Proportion test failure (4096-sample window) interrupt status"]
    #[inline(always)]
    pub fn apt4096if(&self) -> APT4096IF_R {
        APT4096IF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FIFO Full Interrupt Status"]
    #[inline(always)]
    pub fn fullif(&self) -> FULLIF_R {
        FULLIF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn preif(&self) -> PREIF_R {
        PREIF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AIS31 Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn almif(&self) -> ALMIF_R {
        ALMIF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - AIS31 Preliminary Noise Alarm interrupt status"]
    #[inline(always)]
    pub fn preif(&mut self) -> PREIF_W {
        PREIF_W { w: self }
    }
}
