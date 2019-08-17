#[doc = "Reader of register RXSTATUS"]
pub type R = crate::R<u32, super::RXSTATUS>;
#[doc = "Writer for register RXSTATUS"]
pub type W = crate::W<u32, super::RXSTATUS>;
#[doc = "Register RXSTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::RXSTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BUFFNOTAVAIL`"]
pub type BUFFNOTAVAIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUFFNOTAVAIL`"]
pub struct BUFFNOTAVAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFFNOTAVAIL_W<'a> {
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
#[doc = "Reader of field `FRMRX`"]
pub type FRMRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRMRX`"]
pub struct FRMRX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRMRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RXOVERRUN`"]
pub type RXOVERRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVERRUN`"]
pub struct RXOVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVERRUN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RESPNOTOK`"]
pub type RESPNOTOK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESPNOTOK`"]
pub struct RESPNOTOK_W<'a> {
    w: &'a mut W,
}
impl<'a> RESPNOTOK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&self) -> BUFFNOTAVAIL_R {
        BUFFNOTAVAIL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&self) -> FRMRX_R {
        FRMRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&self) -> RXOVERRUN_R {
        RXOVERRUN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&self) -> RESPNOTOK_R {
        RESPNOTOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer not available"]
    #[inline(always)]
    pub fn buffnotavail(&mut self) -> BUFFNOTAVAIL_W {
        BUFFNOTAVAIL_W { w: self }
    }
    #[doc = "Bit 1 - Frame received"]
    #[inline(always)]
    pub fn frmrx(&mut self) -> FRMRX_W {
        FRMRX_W { w: self }
    }
    #[doc = "Bit 2 - Receive overrun"]
    #[inline(always)]
    pub fn rxoverrun(&mut self) -> RXOVERRUN_W {
        RXOVERRUN_W { w: self }
    }
    #[doc = "Bit 3 - bresp/hresp not OK"]
    #[inline(always)]
    pub fn respnotok(&mut self) -> RESPNOTOK_W {
        RESPNOTOK_W { w: self }
    }
}
