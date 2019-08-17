#[doc = "Reader of register DIEPTXF2"]
pub type R = crate::R<u32, super::DIEPTXF2>;
#[doc = "Writer for register DIEPTXF2"]
pub type W = crate::W<u32, super::DIEPTXF2>;
#[doc = "Register DIEPTXF2 `reset()`'s with value 0x0200_0600"]
impl crate::ResetValue for super::DIEPTXF2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0600
    }
}
#[doc = "Reader of field `INEPNTXFSTADDR`"]
pub type INEPNTXFSTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPNTXFSTADDR`"]
pub struct INEPNTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `INEPNTXFDEP`"]
pub type INEPNTXFDEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `INEPNTXFDEP`"]
pub struct INEPNTXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INEPNTXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&self) -> INEPNTXFSTADDR_R {
        INEPNTXFSTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&self) -> INEPNTXFDEP_R {
        INEPNTXFDEP_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - IN Endpoint FIFOn Transmit RAM Start Address"]
    #[inline(always)]
    pub fn inepntxfstaddr(&mut self) -> INEPNTXFSTADDR_W {
        INEPNTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:25 - IN Endpoint TxFIFO Depth"]
    #[inline(always)]
    pub fn inepntxfdep(&mut self) -> INEPNTXFDEP_W {
        INEPNTXFDEP_W { w: self }
    }
}
