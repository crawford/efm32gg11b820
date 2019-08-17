#[doc = "Reader of register POLLINGFLASHSTATUS"]
pub type R = crate::R<u32, super::POLLINGFLASHSTATUS>;
#[doc = "Writer for register POLLINGFLASHSTATUS"]
pub type W = crate::W<u32, super::POLLINGFLASHSTATUS>;
#[doc = "Register POLLINGFLASHSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::POLLINGFLASHSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DEVICESTATUS`"]
pub type DEVICESTATUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DEVICESTATUSVALID`"]
pub type DEVICESTATUSVALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `DEVICESTATUSNBDUMMY`"]
pub type DEVICESTATUSNBDUMMY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DEVICESTATUSNBDUMMY`"]
pub struct DEVICESTATUSNBDUMMY_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICESTATUSNBDUMMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Device Status"]
    #[inline(always)]
    pub fn devicestatus(&self) -> DEVICESTATUS_R {
        DEVICESTATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Device Status Valid"]
    #[inline(always)]
    pub fn devicestatusvalid(&self) -> DEVICESTATUSVALID_R {
        DEVICESTATUSVALID_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&self) -> DEVICESTATUSNBDUMMY_R {
        DEVICESTATUSNBDUMMY_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - Auto-polling Dummy Cycles"]
    #[inline(always)]
    pub fn devicestatusnbdummy(&mut self) -> DEVICESTATUSNBDUMMY_W {
        DEVICESTATUSNBDUMMY_W { w: self }
    }
}
