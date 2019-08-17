#[doc = "Reader of register SRAMPARTITIONCFG"]
pub type R = crate::R<u32, super::SRAMPARTITIONCFG>;
#[doc = "Writer for register SRAMPARTITIONCFG"]
pub type W = crate::W<u32, super::SRAMPARTITIONCFG>;
#[doc = "Register SRAMPARTITIONCFG `reset()`'s with value 0x80"]
impl crate::ResetValue for super::SRAMPARTITIONCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x80
    }
}
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
}
