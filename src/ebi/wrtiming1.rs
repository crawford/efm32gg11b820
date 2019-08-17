#[doc = "Reader of register WRTIMING1"]
pub type R = crate::R<u32, super::WRTIMING1>;
#[doc = "Writer for register WRTIMING1"]
pub type W = crate::W<u32, super::WRTIMING1>;
#[doc = "Register WRTIMING1 `reset()`'s with value 0x0007_7f07"]
impl crate::ResetValue for super::WRTIMING1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0007_7f07
    }
}
#[doc = "Reader of field `WRSETUP`"]
pub type WRSETUP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRSETUP`"]
pub struct WRSETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSETUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WRSTRB`"]
pub type WRSTRB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRSTRB`"]
pub struct WRSTRB_W<'a> {
    w: &'a mut W,
}
impl<'a> WRSTRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `WRHOLD`"]
pub type WRHOLD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRHOLD`"]
pub struct WRHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WRHOLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `HALFWE`"]
pub type HALFWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HALFWE`"]
pub struct HALFWE_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFWE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `WBUFDIS`"]
pub type WBUFDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WBUFDIS`"]
pub struct WBUFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WBUFDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&self) -> WRSETUP_R {
        WRSETUP_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&self) -> WRSTRB_R {
        WRSTRB_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&self) -> WRHOLD_R {
        WRHOLD_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfwe(&self) -> HALFWE_R {
        HALFWE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    pub fn wbufdis(&self) -> WBUFDIS_R {
        WBUFDIS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Write Setup Time"]
    #[inline(always)]
    pub fn wrsetup(&mut self) -> WRSETUP_W {
        WRSETUP_W { w: self }
    }
    #[doc = "Bits 8:14 - Write Strobe Time"]
    #[inline(always)]
    pub fn wrstrb(&mut self) -> WRSTRB_W {
        WRSTRB_W { w: self }
    }
    #[doc = "Bits 16:18 - Write Hold Time"]
    #[inline(always)]
    pub fn wrhold(&mut self) -> WRHOLD_W {
        WRHOLD_W { w: self }
    }
    #[doc = "Bit 28 - Half Cycle WEn Strobe Duration Enable"]
    #[inline(always)]
    pub fn halfwe(&mut self) -> HALFWE_W {
        HALFWE_W { w: self }
    }
    #[doc = "Bit 29 - Write Buffer Disable"]
    #[inline(always)]
    pub fn wbufdis(&mut self) -> WBUFDIS_W {
        WBUFDIS_W { w: self }
    }
}
