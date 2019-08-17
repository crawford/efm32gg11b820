#[doc = "Reader of register OPCODEEXTUPPER"]
pub type R = crate::R<u32, super::OPCODEEXTUPPER>;
#[doc = "Writer for register OPCODEEXTUPPER"]
pub type W = crate::W<u32, super::OPCODEEXTUPPER>;
#[doc = "Register OPCODEEXTUPPER `reset()`'s with value 0x06f9_0000"]
impl crate::ResetValue for super::OPCODEEXTUPPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x06f9_0000
    }
}
#[doc = "Reader of field `EXTWELOPCODE`"]
pub type EXTWELOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EXTWELOPCODE`"]
pub struct EXTWELOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTWELOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `WELOPCODE`"]
pub type WELOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WELOPCODE`"]
pub struct WELOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WELOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&self) -> EXTWELOPCODE_R {
        EXTWELOPCODE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&self) -> WELOPCODE_R {
        WELOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - WEL Opcode Extension"]
    #[inline(always)]
    pub fn extwelopcode(&mut self) -> EXTWELOPCODE_W {
        EXTWELOPCODE_W { w: self }
    }
    #[doc = "Bits 24:31 - WEL Opcode"]
    #[inline(always)]
    pub fn welopcode(&mut self) -> WELOPCODE_W {
        WELOPCODE_W { w: self }
    }
}
