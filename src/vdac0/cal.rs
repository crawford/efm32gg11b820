#[doc = "Reader of register CAL"]
pub type R = crate::R<u32, super::CAL>;
#[doc = "Writer for register CAL"]
pub type W = crate::W<u32, super::CAL>;
#[doc = "Register CAL `reset()`'s with value 0x0008_2004"]
impl crate::ResetValue for super::CAL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_2004
    }
}
#[doc = "Reader of field `OFFSETTRIM`"]
pub type OFFSETTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OFFSETTRIM`"]
pub struct OFFSETTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `GAINERRTRIM`"]
pub type GAINERRTRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAINERRTRIM`"]
pub struct GAINERRTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINERRTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `GAINERRTRIMCH1`"]
pub type GAINERRTRIMCH1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GAINERRTRIMCH1`"]
pub struct GAINERRTRIMCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAINERRTRIMCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&self) -> OFFSETTRIM_R {
        OFFSETTRIM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&self) -> GAINERRTRIM_R {
        GAINERRTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&self) -> GAINERRTRIMCH1_R {
        GAINERRTRIMCH1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&mut self) -> OFFSETTRIM_W {
        OFFSETTRIM_W { w: self }
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&mut self) -> GAINERRTRIM_W {
        GAINERRTRIM_W { w: self }
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&mut self) -> GAINERRTRIMCH1_W {
        GAINERRTRIMCH1_W { w: self }
    }
}
