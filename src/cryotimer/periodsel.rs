#[doc = "Reader of register PERIODSEL"]
pub type R = crate::R<u32, super::PERIODSEL>;
#[doc = "Writer for register PERIODSEL"]
pub type W = crate::W<u32, super::PERIODSEL>;
#[doc = "Register PERIODSEL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::PERIODSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Reader of field `PERIODSEL`"]
pub type PERIODSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PERIODSEL`"]
pub struct PERIODSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIODSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    pub fn periodsel(&self) -> PERIODSEL_R {
        PERIODSEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    pub fn periodsel(&mut self) -> PERIODSEL_W {
        PERIODSEL_W { w: self }
    }
}
