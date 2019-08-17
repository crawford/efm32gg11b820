#[doc = "Reader of register CH1_LOOP"]
pub type R = crate::R<u32, super::CH1_LOOP>;
#[doc = "Writer for register CH1_LOOP"]
pub type W = crate::W<u32, super::CH1_LOOP>;
#[doc = "Register CH1_LOOP `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_LOOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOOPCNT`"]
pub type LOOPCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LOOPCNT`"]
pub struct LOOPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LOOPCNT_R {
        LOOPCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&mut self) -> LOOPCNT_W {
        LOOPCNT_W { w: self }
    }
}
