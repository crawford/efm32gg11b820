#[doc = "Reader of register PB_PINLOCKN"]
pub type R = crate::R<u32, super::PB_PINLOCKN>;
#[doc = "Writer for register PB_PINLOCKN"]
pub type W = crate::W<u32, super::PB_PINLOCKN>;
#[doc = "Register PB_PINLOCKN `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::PB_PINLOCKN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `PINLOCKN`"]
pub type PINLOCKN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PINLOCKN`"]
pub struct PINLOCKN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINLOCKN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PINLOCKN_R {
        PINLOCKN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&mut self) -> PINLOCKN_W {
        PINLOCKN_W { w: self }
    }
}
