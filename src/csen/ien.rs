#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMP`"]
pub type CMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMP`"]
pub struct CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_W<'a> {
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
#[doc = "Reader of field `CONV`"]
pub type CONV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONV`"]
pub struct CONV_W<'a> {
    w: &'a mut W,
}
impl<'a> CONV_W<'a> {
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
#[doc = "Reader of field `EOS`"]
pub type EOS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOS`"]
pub struct EOS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOS_W<'a> {
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
#[doc = "Reader of field `DMAOF`"]
pub type DMAOF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAOF`"]
pub struct DMAOF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAOF_W<'a> {
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
#[doc = "Reader of field `APORTCONFLICT`"]
pub type APORTCONFLICT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORTCONFLICT`"]
pub struct APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> APORTCONFLICT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&self) -> DMAOF_R {
        DMAOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W { w: self }
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&mut self) -> CONV_W {
        CONV_W { w: self }
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W { w: self }
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DMAOF_W {
        DMAOF_W { w: self }
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W {
        APORTCONFLICT_W { w: self }
    }
}
