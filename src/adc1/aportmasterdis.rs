#[doc = "Reader of register APORTMASTERDIS"]
pub type R = crate::R<u32, super::APORTMASTERDIS>;
#[doc = "Writer for register APORTMASTERDIS"]
pub type W = crate::W<u32, super::APORTMASTERDIS>;
#[doc = "Register APORTMASTERDIS `reset()`'s with value 0"]
impl crate::ResetValue for super::APORTMASTERDIS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APORT1XMASTERDIS`"]
pub type APORT1XMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT1XMASTERDIS`"]
pub struct APORT1XMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT1XMASTERDIS_W<'a> {
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
#[doc = "Reader of field `APORT1YMASTERDIS`"]
pub type APORT1YMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT1YMASTERDIS`"]
pub struct APORT1YMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT1YMASTERDIS_W<'a> {
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
#[doc = "Reader of field `APORT2XMASTERDIS`"]
pub type APORT2XMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT2XMASTERDIS`"]
pub struct APORT2XMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT2XMASTERDIS_W<'a> {
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
#[doc = "Reader of field `APORT2YMASTERDIS`"]
pub type APORT2YMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT2YMASTERDIS`"]
pub struct APORT2YMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT2YMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `APORT3XMASTERDIS`"]
pub type APORT3XMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT3XMASTERDIS`"]
pub struct APORT3XMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT3XMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `APORT3YMASTERDIS`"]
pub type APORT3YMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT3YMASTERDIS`"]
pub struct APORT3YMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT3YMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `APORT4XMASTERDIS`"]
pub type APORT4XMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT4XMASTERDIS`"]
pub struct APORT4XMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT4XMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `APORT4YMASTERDIS`"]
pub type APORT4YMASTERDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APORT4YMASTERDIS`"]
pub struct APORT4YMASTERDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> APORT4YMASTERDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&self) -> APORT1XMASTERDIS_R {
        APORT1XMASTERDIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&self) -> APORT1YMASTERDIS_R {
        APORT1YMASTERDIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&self) -> APORT2XMASTERDIS_R {
        APORT2XMASTERDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&self) -> APORT2YMASTERDIS_R {
        APORT2YMASTERDIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&self) -> APORT3XMASTERDIS_R {
        APORT3XMASTERDIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&self) -> APORT3YMASTERDIS_R {
        APORT3YMASTERDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&self) -> APORT4XMASTERDIS_R {
        APORT4XMASTERDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&self) -> APORT4YMASTERDIS_R {
        APORT4YMASTERDIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&mut self) -> APORT1XMASTERDIS_W {
        APORT1XMASTERDIS_W { w: self }
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&mut self) -> APORT1YMASTERDIS_W {
        APORT1YMASTERDIS_W { w: self }
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&mut self) -> APORT2XMASTERDIS_W {
        APORT2XMASTERDIS_W { w: self }
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&mut self) -> APORT2YMASTERDIS_W {
        APORT2YMASTERDIS_W { w: self }
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&mut self) -> APORT3XMASTERDIS_W {
        APORT3XMASTERDIS_W { w: self }
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&mut self) -> APORT3YMASTERDIS_W {
        APORT3YMASTERDIS_W { w: self }
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&mut self) -> APORT4XMASTERDIS_W {
        APORT4XMASTERDIS_W { w: self }
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&mut self) -> APORT4YMASTERDIS_W {
        APORT4YMASTERDIS_W { w: self }
    }
}
