#[doc = "Reader of register DATE"]
pub type R = crate::R<u32, super::DATE>;
#[doc = "Writer for register DATE"]
pub type W = crate::W<u32, super::DATE>;
#[doc = "Register DATE `reset()`'s with value 0"]
impl crate::ResetValue for super::DATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAYOMU`"]
pub type DAYOMU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYOMU`"]
pub struct DAYOMU_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOMU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DAYOMT`"]
pub type DAYOMT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYOMT`"]
pub struct DAYOMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOMT_W<'a> {
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
#[doc = "Reader of field `YEARU`"]
pub type YEARU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEARU`"]
pub struct YEARU_W<'a> {
    w: &'a mut W,
}
impl<'a> YEARU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `YEART`"]
pub type YEART_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEART`"]
pub struct YEART_W<'a> {
    w: &'a mut W,
}
impl<'a> YEART_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DAYOW`"]
pub type DAYOW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAYOW`"]
pub struct DAYOW_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&self) -> DAYOMU_R {
        DAYOMU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&self) -> DAYOMT_R {
        DAYOMT_R::new(((self.bits >> 4) & 0x03) as u8)
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
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&self) -> YEARU_R {
        YEARU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&self) -> YEART_R {
        YEART_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&self) -> DAYOW_R {
        DAYOW_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&mut self) -> DAYOMU_W {
        DAYOMU_W { w: self }
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&mut self) -> DAYOMT_W {
        DAYOMT_W { w: self }
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
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&mut self) -> YEARU_W {
        YEARU_W { w: self }
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&mut self) -> YEART_W {
        YEART_W { w: self }
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&mut self) -> DAYOW_W {
        DAYOW_W { w: self }
    }
}
