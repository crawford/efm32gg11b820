#[doc = "Reader of register CH6_LINK"]
pub type R = crate::R<u32, super::CH6_LINK>;
#[doc = "Writer for register CH6_LINK"]
pub type W = crate::W<u32, super::CH6_LINK>;
#[doc = "Register CH6_LINK `reset()`'s with value 0"]
impl crate::ResetValue for super::CH6_LINK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LINKMODE`"]
pub type LINKMODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `LINK`"]
pub type LINK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LINK`"]
pub struct LINK_W<'a> {
    w: &'a mut W,
}
impl<'a> LINK_W<'a> {
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
#[doc = "Reader of field `LINKADDR`"]
pub type LINKADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LINKADDR`"]
pub struct LINKADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LINKADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Link Structure Addressing Mode"]
    #[inline(always)]
    pub fn linkmode(&self) -> LINKMODE_R {
        LINKMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&self) -> LINK_R {
        LINK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&self) -> LINKADDR_R {
        LINKADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 1 - Link Next Structure"]
    #[inline(always)]
    pub fn link(&mut self) -> LINK_W {
        LINK_W { w: self }
    }
    #[doc = "Bits 2:31 - Link Structure Address"]
    #[inline(always)]
    pub fn linkaddr(&mut self) -> LINKADDR_W {
        LINKADDR_W { w: self }
    }
}
