#[doc = "Reader of register CC1_DATE"]
pub type R = crate::R<u32, super::CC1_DATE>;
#[doc = "Writer for register CC1_DATE"]
pub type W = crate::W<u32, super::CC1_DATE>;
#[doc = "Register CC1_DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::CC1_DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAYU`"]
pub type DAYU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYU`"]
pub struct DAYU_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DAYT`"]
pub type DAYT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYT`"]
pub struct DAYT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `MONTHU`"]
pub type MONTHU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTHU`"]
pub struct MONTHU_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTHU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MONTHT`"]
pub type MONTHT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MONTHT`"]
pub struct MONTHT_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTHT_W<'a> {
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
impl R {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&self) -> MONTHU_R {
        MONTHU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&self) -> MONTHT_R {
        MONTHT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DAYU_W {
        DAYU_W { w: self }
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&mut self) -> DAYT_W {
        DAYT_W { w: self }
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&mut self) -> MONTHU_W {
        MONTHU_W { w: self }
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&mut self) -> MONTHT_W {
        MONTHT_W { w: self }
    }
}
