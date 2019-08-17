#[doc = "Reader of register STACKEDVLAN"]
pub type R = crate::R<u32, super::STACKEDVLAN>;
#[doc = "Writer for register STACKEDVLAN"]
pub type W = crate::W<u32, super::STACKEDVLAN>;
#[doc = "Register STACKEDVLAN `reset()`'s with value 0"]
impl crate::ResetValue for super::STACKEDVLAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MATCH`"]
pub type MATCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `MATCH`"]
pub struct MATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> MATCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `ENBPROCESSING`"]
pub type ENBPROCESSING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENBPROCESSING`"]
pub struct ENBPROCESSING_W<'a> {
    w: &'a mut W,
}
impl<'a> ENBPROCESSING_W<'a> {
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
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&self) -> MATCH_R {
        MATCH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&self) -> ENBPROCESSING_R {
        ENBPROCESSING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - User defined VLAN_TYPE field"]
    #[inline(always)]
    pub fn match_(&mut self) -> MATCH_W {
        MATCH_W { w: self }
    }
    #[doc = "Bit 31 - Enable stacked VLAN processing mode"]
    #[inline(always)]
    pub fn enbprocessing(&mut self) -> ENBPROCESSING_W {
        ENBPROCESSING_W { w: self }
    }
}
