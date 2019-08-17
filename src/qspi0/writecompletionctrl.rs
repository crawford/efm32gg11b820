#[doc = "Reader of register WRITECOMPLETIONCTRL"]
pub type R = crate::R<u32, super::WRITECOMPLETIONCTRL>;
#[doc = "Writer for register WRITECOMPLETIONCTRL"]
pub type W = crate::W<u32, super::WRITECOMPLETIONCTRL>;
#[doc = "Register WRITECOMPLETIONCTRL `reset()`'s with value 0x0001_0005"]
impl crate::ResetValue for super::WRITECOMPLETIONCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0005
    }
}
#[doc = "Reader of field `OPCODE`"]
pub type OPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPCODE`"]
pub struct OPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `POLLINGBITINDEX`"]
pub type POLLINGBITINDEX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POLLINGBITINDEX`"]
pub struct POLLINGBITINDEX_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLINGBITINDEX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `POLLINGPOLARITY`"]
pub type POLLINGPOLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POLLINGPOLARITY`"]
pub struct POLLINGPOLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLINGPOLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DISABLEPOLLING`"]
pub type DISABLEPOLLING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISABLEPOLLING`"]
pub struct DISABLEPOLLING_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLEPOLLING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `ENABLEPOLLINGEXP`"]
pub type ENABLEPOLLINGEXP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEPOLLINGEXP`"]
pub struct ENABLEPOLLINGEXP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEPOLLINGEXP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `POLLCOUNT`"]
pub type POLLCOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POLLCOUNT`"]
pub struct POLLCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `POLLREPDELAY`"]
pub type POLLREPDELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `POLLREPDELAY`"]
pub struct POLLREPDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLLREPDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&self) -> POLLINGBITINDEX_R {
        POLLINGBITINDEX_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&self) -> POLLINGPOLARITY_R {
        POLLINGPOLARITY_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&self) -> DISABLEPOLLING_R {
        DISABLEPOLLING_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&self) -> ENABLEPOLLINGEXP_R {
        ENABLEPOLLINGEXP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&self) -> POLLCOUNT_R {
        POLLCOUNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&self) -> POLLREPDELAY_R {
        POLLREPDELAY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Opcode"]
    #[inline(always)]
    pub fn opcode(&mut self) -> OPCODE_W {
        OPCODE_W { w: self }
    }
    #[doc = "Bits 8:10 - Polling Bit Index"]
    #[inline(always)]
    pub fn pollingbitindex(&mut self) -> POLLINGBITINDEX_W {
        POLLINGBITINDEX_W { w: self }
    }
    #[doc = "Bit 13 - Polling Polarity"]
    #[inline(always)]
    pub fn pollingpolarity(&mut self) -> POLLINGPOLARITY_W {
        POLLINGPOLARITY_W { w: self }
    }
    #[doc = "Bit 14 - Disable Polling"]
    #[inline(always)]
    pub fn disablepolling(&mut self) -> DISABLEPOLLING_W {
        DISABLEPOLLING_W { w: self }
    }
    #[doc = "Bit 15 - Enable Polling Expiration"]
    #[inline(always)]
    pub fn enablepollingexp(&mut self) -> ENABLEPOLLINGEXP_W {
        ENABLEPOLLINGEXP_W { w: self }
    }
    #[doc = "Bits 16:23 - Poll Count"]
    #[inline(always)]
    pub fn pollcount(&mut self) -> POLLCOUNT_W {
        POLLCOUNT_W { w: self }
    }
    #[doc = "Bits 24:31 - Poll Repetition Delay"]
    #[inline(always)]
    pub fn pollrepdelay(&mut self) -> POLLREPDELAY_W {
        POLLREPDELAY_W { w: self }
    }
}
