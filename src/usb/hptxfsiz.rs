#[doc = "Reader of register HPTXFSIZ"]
pub type R = crate::R<u32, super::HPTXFSIZ>;
#[doc = "Writer for register HPTXFSIZ"]
pub type W = crate::W<u32, super::HPTXFSIZ>;
#[doc = "Register HPTXFSIZ `reset()`'s with value 0x0200_0400"]
impl crate::ResetValue for super::HPTXFSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0200_0400
    }
}
#[doc = "Reader of field `PTXFSTADDR`"]
pub type PTXFSTADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTXFSTADDR`"]
pub struct PTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
        self.w
    }
}
#[doc = "Reader of field `PTXFSIZE`"]
pub type PTXFSIZE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PTXFSIZE`"]
pub struct PTXFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptxfstaddr(&self) -> PTXFSTADDR_R {
        PTXFSTADDR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:25 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptxfsize(&self) -> PTXFSIZE_R {
        PTXFSIZE_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Host Periodic TxFIFO Start Address"]
    #[inline(always)]
    pub fn ptxfstaddr(&mut self) -> PTXFSTADDR_W {
        PTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:25 - Host Periodic TxFIFO Depth"]
    #[inline(always)]
    pub fn ptxfsize(&mut self) -> PTXFSIZE_W {
        PTXFSIZE_W { w: self }
    }
}
