#[doc = "Reader of register HC10_TSIZ"]
pub type R = crate::R<u32, super::HC10_TSIZ>;
#[doc = "Writer for register HC10_TSIZ"]
pub type W = crate::W<u32, super::HC10_TSIZ>;
#[doc = "Register HC10_TSIZ `reset()`'s with value 0"]
impl crate::ResetValue for super::HC10_TSIZ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `XFERSIZE`"]
pub type XFERSIZE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `XFERSIZE`"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
#[doc = "Reader of field `PKTCNT`"]
pub type PKTCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PKTCNT`"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 19)) | (((value as u32) & 0x03ff) << 19);
        self.w
    }
}
#[doc = "Reader of field `PID`"]
pub type PID_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PID`"]
pub struct PID_W<'a> {
    w: &'a mut W,
}
impl<'a> PID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | (((value as u32) & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x0007_ffff) as u32)
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:28 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - The Application Programs This Field With the Type of"]
    #[inline(always)]
    pub fn pid(&mut self) -> PID_W {
        PID_W { w: self }
    }
}
