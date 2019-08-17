#[doc = "Reader of register SYNC"]
pub type R = crate::R<u32, super::SYNC>;
#[doc = "Writer for register SYNC"]
pub type W = crate::W<u32, super::SYNC>;
#[doc = "Register SYNC `reset()`'s with value 0"]
impl crate::ResetValue for super::SYNC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYNCTRIG`"]
pub type SYNCTRIG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYNCTRIG`"]
pub struct SYNCTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNCTRIG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Synchronization Trigger"]
    #[inline(always)]
    pub fn synctrig(&self) -> SYNCTRIG_R {
        SYNCTRIG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization Trigger"]
    #[inline(always)]
    pub fn synctrig(&mut self) -> SYNCTRIG_W {
        SYNCTRIG_W { w: self }
    }
}
