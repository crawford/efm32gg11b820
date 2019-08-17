#[doc = "Reader of register SDMASYSADDR"]
pub type R = crate::R<u32, super::SDMASYSADDR>;
#[doc = "Writer for register SDMASYSADDR"]
pub type W = crate::W<u32, super::SDMASYSADDR>;
#[doc = "Register SDMASYSADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::SDMASYSADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SDMASYSADDRARG`"]
pub type SDMASYSADDRARG_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SDMASYSADDRARG`"]
pub struct SDMASYSADDRARG_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMASYSADDRARG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&self) -> SDMASYSADDRARG_R {
        SDMASYSADDRARG_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&mut self) -> SDMASYSADDRARG_W {
        SDMASYSADDRARG_W { w: self }
    }
}
