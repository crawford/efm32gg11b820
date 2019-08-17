#[doc = "Reader of register DBGHALT"]
pub type R = crate::R<u32, super::DBGHALT>;
#[doc = "Writer for register DBGHALT"]
pub type W = crate::W<u32, super::DBGHALT>;
#[doc = "Register DBGHALT `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGHALT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBGHALT`"]
pub type DBGHALT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DBGHALT`"]
pub struct DBGHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGHALT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W { w: self }
    }
}
