#[doc = "Reader of register TSUTIMERINCRSUBNSEC"]
pub type R = crate::R<u32, super::TSUTIMERINCRSUBNSEC>;
#[doc = "Writer for register TSUTIMERINCRSUBNSEC"]
pub type W = crate::W<u32, super::TSUTIMERINCRSUBNSEC>;
#[doc = "Register TSUTIMERINCRSUBNSEC `reset()`'s with value 0"]
impl crate::ResetValue for super::TSUTIMERINCRSUBNSEC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBNSINCR`"]
pub type SUBNSINCR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SUBNSINCR`"]
pub struct SUBNSINCR_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBNSINCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `SUBNSINCRLSB`"]
pub type SUBNSINCRLSB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SUBNSINCRLSB`"]
pub struct SUBNSINCRLSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBNSINCRLSB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - MSB \\[23:8\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&self) -> SUBNSINCR_R {
        SUBNSINCR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&self) -> SUBNSINCRLSB_R {
        SUBNSINCRLSB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - MSB \\[23:8\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincr(&mut self) -> SUBNSINCR_W {
        SUBNSINCR_W { w: self }
    }
    #[doc = "Bits 24:31 - LSB \\[7:0\\]
of the subscript-ns value"]
    #[inline(always)]
    pub fn subnsincrlsb(&mut self) -> SUBNSINCRLSB_W {
        SUBNSINCRLSB_W { w: self }
    }
}
