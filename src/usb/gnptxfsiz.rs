#[doc = "Reader of register GNPTXFSIZ"]
pub type R = crate::R<u32, super::GNPTXFSIZ>;
#[doc = "Writer for register GNPTXFSIZ"]
pub type W = crate::W<u32, super::GNPTXFSIZ>;
#[doc = "Register GNPTXFSIZ `reset()`'s with value 0x0200_0200"]
impl crate::ResetValue for super::GNPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0200
    }
}
#[doc = "Reader of field `NPTXFSTADDR`"]
pub type NPTXFSTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPTXFSTADDR`"]
pub struct NPTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `NPTXFDEP`"]
pub type NPTXFDEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `NPTXFDEP`"]
pub struct NPTXFDEP_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFDEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptxfdep(&self) -> NPTXFDEP_R {
        NPTXFDEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Non-periodic Transmit RAM Start Address"]
    #[inline(always)]
    pub fn nptxfstaddr(&mut self) -> NPTXFSTADDR_W {
        NPTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn nptxfdep(&mut self) -> NPTXFDEP_W {
        NPTXFDEP_W { w: self }
    }
}
