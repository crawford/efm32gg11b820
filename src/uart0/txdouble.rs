#[doc = "Reader of register TXDOUBLE"]
pub type R = crate::R<u32, super::TXDOUBLE>;
#[doc = "Writer for register TXDOUBLE"]
pub type W = crate::W<u32, super::TXDOUBLE>;
#[doc = "Register TXDOUBLE `reset()`'s with value 0"]
impl crate::ResetValue for super::TXDOUBLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXDATA0`"]
pub type TXDATA0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDATA0`"]
pub struct TXDATA0_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TXDATA1`"]
pub type TXDATA1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXDATA1`"]
pub struct TXDATA1_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDATA1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&self) -> TXDATA0_R {
        TXDATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&self) -> TXDATA1_R {
        TXDATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> TXDATA0_W {
        TXDATA0_W { w: self }
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> TXDATA1_W {
        TXDATA1_W { w: self }
    }
}
