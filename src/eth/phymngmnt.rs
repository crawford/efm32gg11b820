#[doc = "Reader of register PHYMNGMNT"]
pub type R = crate::R<u32, super::PHYMNGMNT>;
#[doc = "Writer for register PHYMNGMNT"]
pub type W = crate::W<u32, super::PHYMNGMNT>;
#[doc = "Register PHYMNGMNT `reset()`'s with value 0"]
impl crate::ResetValue for super::PHYMNGMNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHYRWDATA`"]
pub type PHYRWDATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PHYRWDATA`"]
pub struct PHYRWDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYRWDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `WRITE10`"]
pub type WRITE10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRITE10`"]
pub struct WRITE10_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `REGADDR`"]
pub type REGADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGADDR`"]
pub struct REGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | (((value as u32) & 0x1f) << 18);
        self.w
    }
}
#[doc = "Reader of field `PHYADDR`"]
pub type PHYADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PHYADDR`"]
pub struct PHYADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 23)) | (((value as u32) & 0x1f) << 23);
        self.w
    }
}
#[doc = "Reader of field `OPERATION`"]
pub type OPERATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPERATION`"]
pub struct OPERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> OPERATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `WRITE1`"]
pub type WRITE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE1`"]
pub struct WRITE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `WRITE0`"]
pub type WRITE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRITE0`"]
pub struct WRITE0_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&self) -> PHYRWDATA_R {
        PHYRWDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&self) -> WRITE10_R {
        WRITE10_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&self) -> OPERATION_R {
        OPERATION_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&self) -> WRITE1_R {
        WRITE1_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&self) -> WRITE0_R {
        WRITE0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - PHY read write data"]
    #[inline(always)]
    pub fn phyrwdata(&mut self) -> PHYRWDATA_W {
        PHYRWDATA_W { w: self }
    }
    #[doc = "Bits 16:17 - Must be written with 10."]
    #[inline(always)]
    pub fn write10(&mut self) -> WRITE10_W {
        WRITE10_W { w: self }
    }
    #[doc = "Bits 18:22 - Register address - specifies the register in the PHY to access."]
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W {
        REGADDR_W { w: self }
    }
    #[doc = "Bits 23:27 - PHY address."]
    #[inline(always)]
    pub fn phyaddr(&mut self) -> PHYADDR_W {
        PHYADDR_W { w: self }
    }
    #[doc = "Bits 28:29 - Operation. For a Clause 45 frame: 00 is an addr, 01 is a write, 10 is a post read increment, 11 is a read frame. For a Clause 22 frame: 10 is a read, 01 is a write."]
    #[inline(always)]
    pub fn operation(&mut self) -> OPERATION_W {
        OPERATION_W { w: self }
    }
    #[doc = "Bit 30 - Must be written to 1 for a valid Clause 22 frame and to 0 for a valid Clause 45 frame."]
    #[inline(always)]
    pub fn write1(&mut self) -> WRITE1_W {
        WRITE1_W { w: self }
    }
    #[doc = "Bit 31 - Must be written with 0."]
    #[inline(always)]
    pub fn write0(&mut self) -> WRITE0_W {
        WRITE0_W { w: self }
    }
}
