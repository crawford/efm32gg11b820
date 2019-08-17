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
#[doc = "Reader of field `TOUT`"]
pub type TOUT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUT`"]
pub struct TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUT_W<'a> {
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
#[doc = "Reader of field `WARN`"]
pub type WARN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WARN`"]
pub struct WARN_W<'a> {
    w: &'a mut W,
}
impl<'a> WARN_W<'a> {
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
#[doc = "Reader of field `WIN`"]
pub type WIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WIN`"]
pub struct WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIN_W<'a> {
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
#[doc = "Reader of field `PEM0`"]
pub type PEM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEM0`"]
pub struct PEM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEM0_W<'a> {
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
#[doc = "Reader of field `PEM1`"]
pub type PEM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PEM1`"]
pub struct PEM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEM1_W<'a> {
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
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W { w: self }
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&mut self) -> WARN_W {
        WARN_W { w: self }
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W { w: self }
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&mut self) -> PEM0_W {
        PEM0_W { w: self }
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&mut self) -> PEM1_W {
        PEM1_W { w: self }
    }
}
