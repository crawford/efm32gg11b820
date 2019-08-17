#[doc = "Reader of register BUFDATPORT"]
pub type R = crate::R<u32, super::BUFDATPORT>;
#[doc = "Writer for register BUFDATPORT"]
pub type W = crate::W<u32, super::BUFDATPORT>;
#[doc = "Register BUFDATPORT `reset()`'s with value 0"]
impl crate::ResetValue for super::BUFDATPORT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFDAT`"]
pub type BUFDAT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `BUFDAT`"]
pub struct BUFDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&self) -> BUFDAT_R {
        BUFDAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Data"]
    #[inline(always)]
    pub fn bufdat(&mut self) -> BUFDAT_W {
        BUFDAT_W { w: self }
    }
}
