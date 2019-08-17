#[doc = "Reader of register FLASHCOMMANDCTRLMEM"]
pub type R = crate::R<u32, super::FLASHCOMMANDCTRLMEM>;
#[doc = "Writer for register FLASHCOMMANDCTRLMEM"]
pub type W = crate::W<u32, super::FLASHCOMMANDCTRLMEM>;
#[doc = "Register FLASHCOMMANDCTRLMEM `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHCOMMANDCTRLMEM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `TRIGGERMEMBANKREQ`"]
pub struct TRIGGERMEMBANKREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGGERMEMBANKREQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MEMBANKREQINPROGRESS`"]
pub type MEMBANKREQINPROGRESS_R = crate::R<bool, bool>;
#[doc = "Reader of field `MEMBANKREADDATA`"]
pub type MEMBANKREADDATA_R = crate::R<u8, u8>;
#[doc = "Reader of field `NBOFSTIGREADBYTES`"]
pub type NBOFSTIGREADBYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBOFSTIGREADBYTES`"]
pub struct NBOFSTIGREADBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBOFSTIGREADBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `MEMBANKADDR`"]
pub type MEMBANKADDR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MEMBANKADDR`"]
pub struct MEMBANKADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> MEMBANKADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 20)) | (((value as u32) & 0x01ff) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Memory Bank Data Request in Progress"]
    #[inline(always)]
    pub fn membankreqinprogress(&self) -> MEMBANKREQINPROGRESS_R {
        MEMBANKREQINPROGRESS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Last Requested Data From the STIG Memory Bank"]
    #[inline(always)]
    pub fn membankreaddata(&self) -> MEMBANKREADDATA_R {
        MEMBANKREADDATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline(always)]
    pub fn nbofstigreadbytes(&self) -> NBOFSTIGREADBYTES_R {
        NBOFSTIGREADBYTES_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline(always)]
    pub fn membankaddr(&self) -> MEMBANKADDR_R {
        MEMBANKADDR_R::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger the Memory Bank Data Request"]
    #[inline(always)]
    pub fn triggermembankreq(&mut self) -> TRIGGERMEMBANKREQ_W {
        TRIGGERMEMBANKREQ_W { w: self }
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline(always)]
    pub fn nbofstigreadbytes(&mut self) -> NBOFSTIGREADBYTES_W {
        NBOFSTIGREADBYTES_W { w: self }
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline(always)]
    pub fn membankaddr(&mut self) -> MEMBANKADDR_W {
        MEMBANKADDR_W { w: self }
    }
}
