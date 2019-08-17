#[doc = "Reader of register PBUFTXCUTTHRU"]
pub type R = crate::R<u32, super::PBUFTXCUTTHRU>;
#[doc = "Writer for register PBUFTXCUTTHRU"]
pub type W = crate::W<u32, super::PBUFTXCUTTHRU>;
#[doc = "Register PBUFTXCUTTHRU `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::PBUFTXCUTTHRU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `DMATXCUTTHRUTHR`"]
pub type DMATXCUTTHRUTHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMATXCUTTHRUTHR`"]
pub struct DMATXCUTTHRUTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXCUTTHRUTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `DMATXCUTTHRU`"]
pub type DMATXCUTTHRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMATXCUTTHRU`"]
pub struct DMATXCUTTHRU_W<'a> {
    w: &'a mut W,
}
impl<'a> DMATXCUTTHRU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&self) -> DMATXCUTTHRUTHR_R {
        DMATXCUTTHRUTHR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&self) -> DMATXCUTTHRU_R {
        DMATXCUTTHRU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmatxcutthruthr(&mut self) -> DMATXCUTTHRUTHR_W {
        DMATXCUTTHRUTHR_W { w: self }
    }
    #[doc = "Bit 31 - Enable TX partial store and forward operation"]
    #[inline(always)]
    pub fn dmatxcutthru(&mut self) -> DMATXCUTTHRU_W {
        DMATXCUTTHRU_W { w: self }
    }
}
