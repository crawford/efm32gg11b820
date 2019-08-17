#[doc = "Reader of register GDFIFOCFG"]
pub type R = crate::R<u32, super::GDFIFOCFG>;
#[doc = "Writer for register GDFIFOCFG"]
pub type W = crate::W<u32, super::GDFIFOCFG>;
#[doc = "Register GDFIFOCFG `reset()`'s with value 0x01f2_0200"]
impl crate::ResetValue for super::GDFIFOCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01f2_0200
    }
}
#[doc = "Reader of field `GDFIFOCFG`"]
pub type GDFIFOCFG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GDFIFOCFG`"]
pub struct GDFIFOCFG_W<'a> {
    w: &'a mut W,
}
impl<'a> GDFIFOCFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `EPINFOBASEADDR`"]
pub type EPINFOBASEADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `EPINFOBASEADDR`"]
pub struct EPINFOBASEADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> EPINFOBASEADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gdfifocfg(&self) -> GDFIFOCFG_R {
        GDFIFOCFG_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn epinfobaseaddr(&self) -> EPINFOBASEADDR_R {
        EPINFOBASEADDR_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gdfifocfg(&mut self) -> GDFIFOCFG_W {
        GDFIFOCFG_W { w: self }
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn epinfobaseaddr(&mut self) -> EPINFOBASEADDR_W {
        EPINFOBASEADDR_W { w: self }
    }
}
