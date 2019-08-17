#[doc = "Reader of register PBUFRXCUTTHRU"]
pub type R = crate::R<u32, super::PBUFRXCUTTHRU>;
#[doc = "Writer for register PBUFRXCUTTHRU"]
pub type W = crate::W<u32, super::PBUFRXCUTTHRU>;
#[doc = "Register PBUFRXCUTTHRU `reset()`'s with value 0x03ff"]
impl crate::ResetValue for super::PBUFRXCUTTHRU {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03ff
    }
}
#[doc = "Reader of field `DMARXCUTTHRUTHR`"]
pub type DMARXCUTTHRUTHR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DMARXCUTTHRUTHR`"]
pub struct DMARXCUTTHRUTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXCUTTHRUTHR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `DMARXCUTTHRU`"]
pub type DMARXCUTTHRU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMARXCUTTHRU`"]
pub struct DMARXCUTTHRU_W<'a> {
    w: &'a mut W,
}
impl<'a> DMARXCUTTHRU_W<'a> {
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
    pub fn dmarxcutthruthr(&self) -> DMARXCUTTHRUTHR_R {
        DMARXCUTTHRUTHR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&self) -> DMARXCUTTHRU_R {
        DMARXCUTTHRU_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Watermark value"]
    #[inline(always)]
    pub fn dmarxcutthruthr(&mut self) -> DMARXCUTTHRUTHR_W {
        DMARXCUTTHRUTHR_W { w: self }
    }
    #[doc = "Bit 31 - Enable RX partial store and forward operation"]
    #[inline(always)]
    pub fn dmarxcutthru(&mut self) -> DMARXCUTTHRU_W {
        DMARXCUTTHRU_W { w: self }
    }
}
