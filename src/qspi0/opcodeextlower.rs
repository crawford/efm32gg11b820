#[doc = "Reader of register OPCODEEXTLOWER"]
pub type R = crate::R<u32, super::OPCODEEXTLOWER>;
#[doc = "Writer for register OPCODEEXTLOWER"]
pub type W = crate::W<u32, super::OPCODEEXTLOWER>;
#[doc = "Register OPCODEEXTLOWER `reset()`'s with value 0x13ed_fa00"]
impl crate::ResetValue for super::OPCODEEXTLOWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x13ed_fa00
    }
}
#[doc = "Reader of field `EXTSTIGOPCODE`"]
pub type EXTSTIGOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTSTIGOPCODE`"]
pub struct EXTSTIGOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSTIGOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `EXTPOLLOPCODE`"]
pub type EXTPOLLOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTPOLLOPCODE`"]
pub struct EXTPOLLOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTPOLLOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `EXTWRITEOPCODE`"]
pub type EXTWRITEOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTWRITEOPCODE`"]
pub struct EXTWRITEOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTWRITEOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `EXTREADOPCODE`"]
pub type EXTREADOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTREADOPCODE`"]
pub struct EXTREADOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTREADOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&self) -> EXTSTIGOPCODE_R {
        EXTSTIGOPCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&self) -> EXTPOLLOPCODE_R {
        EXTPOLLOPCODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&self) -> EXTWRITEOPCODE_R {
        EXTWRITEOPCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&self) -> EXTREADOPCODE_R {
        EXTREADOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STIG Opcode Extension"]
    #[inline(always)]
    pub fn extstigopcode(&mut self) -> EXTSTIGOPCODE_W {
        EXTSTIGOPCODE_W { w: self }
    }
    #[doc = "Bits 8:15 - Polling Opcode Extension"]
    #[inline(always)]
    pub fn extpollopcode(&mut self) -> EXTPOLLOPCODE_W {
        EXTPOLLOPCODE_W { w: self }
    }
    #[doc = "Bits 16:23 - Write Opcode Extension"]
    #[inline(always)]
    pub fn extwriteopcode(&mut self) -> EXTWRITEOPCODE_W {
        EXTWRITEOPCODE_W { w: self }
    }
    #[doc = "Bits 24:31 - Read Opcode Extension"]
    #[inline(always)]
    pub fn extreadopcode(&mut self) -> EXTREADOPCODE_W {
        EXTREADOPCODE_W { w: self }
    }
}
