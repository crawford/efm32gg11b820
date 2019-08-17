#[doc = "Reader of register CC0_TIME"]
pub type R = crate::R<u32, super::CC0_TIME>;
#[doc = "Writer for register CC0_TIME"]
pub type W = crate::W<u32, super::CC0_TIME>;
#[doc = "Register CC0_TIME `reset()`'s with value 0"]
impl crate::ResetValue for super::CC0_TIME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SECU`"]
pub type SECU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECU`"]
pub struct SECU_W<'a> {
    w: &'a mut W,
}
impl<'a> SECU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `SECT`"]
pub type SECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SECT`"]
pub struct SECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `MINU`"]
pub type MINU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINU`"]
pub struct MINU_W<'a> {
    w: &'a mut W,
}
impl<'a> MINU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `MINT`"]
pub type MINT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MINT`"]
pub struct MINT_W<'a> {
    w: &'a mut W,
}
impl<'a> MINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `HOURU`"]
pub type HOURU_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURU`"]
pub struct HOURU_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `HOURT`"]
pub type HOURT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOURT`"]
pub struct HOURT_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SECU_R {
        SECU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SECT_R {
        SECT_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MINU_R {
        MINU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MINT_R {
        MINT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HOURU_R {
        HOURU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HOURT_R {
        HOURT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&mut self) -> SECU_W {
        SECU_W { w: self }
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&mut self) -> SECT_W {
        SECT_W { w: self }
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&mut self) -> MINU_W {
        MINU_W { w: self }
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&mut self) -> MINT_W {
        MINT_W { w: self }
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&mut self) -> HOURU_W {
        HOURU_W { w: self }
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&mut self) -> HOURT_W {
        HOURT_W { w: self }
    }
}
