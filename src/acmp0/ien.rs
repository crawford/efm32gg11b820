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
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EDGE`"]
pub struct EDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> EDGE_W<'a> {
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
#[doc = "Reader of field `WARMUP`"]
pub type WARMUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WARMUP`"]
pub struct WARMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EDGE Interrupt Enable"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WARMUP Interrupt Enable"]
    #[inline(always)]
    pub fn warmup(&self) -> WARMUP_R {
        WARMUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EDGE Interrupt Enable"]
    #[inline(always)]
    pub fn edge(&mut self) -> EDGE_W {
        EDGE_W { w: self }
    }
    #[doc = "Bit 1 - WARMUP Interrupt Enable"]
    #[inline(always)]
    pub fn warmup(&mut self) -> WARMUP_W {
        WARMUP_W { w: self }
    }
    #[doc = "Bit 2 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W {
        APORTCONFLICT_W { w: self }
    }
}
