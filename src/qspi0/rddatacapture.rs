#[doc = "Reader of register RDDATACAPTURE"]
pub type R = crate::R<u32, super::RDDATACAPTURE>;
#[doc = "Writer for register RDDATACAPTURE"]
pub type W = crate::W<u32, super::RDDATACAPTURE>;
#[doc = "Register RDDATACAPTURE `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RDDATACAPTURE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
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
#[doc = "Reader of field `DELAY`"]
pub type DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DELAY`"]
pub struct DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `DQSENABLE`"]
pub type DQSENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DQSENABLE`"]
pub struct DQSENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSENABLE_W<'a> {
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
#[doc = "Reader of field `DDRREADDELAY`"]
pub type DDRREADDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DDRREADDELAY`"]
pub struct DDRREADDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRREADDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&self) -> DQSENABLE_R {
        DQSENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&self) -> DDRREADDELAY_R {
        DDRREADDELAY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass the Adapted Loopback Clock Circuit"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bits 1:4 - Read Delay"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W {
        DELAY_W { w: self }
    }
    #[doc = "Bit 8 - DQS Enable Bit"]
    #[inline(always)]
    pub fn dqsenable(&mut self) -> DQSENABLE_W {
        DQSENABLE_W { w: self }
    }
    #[doc = "Bits 16:19 - DDR Read Delay"]
    #[inline(always)]
    pub fn ddrreaddelay(&mut self) -> DDRREADDELAY_W {
        DDRREADDELAY_W { w: self }
    }
}
