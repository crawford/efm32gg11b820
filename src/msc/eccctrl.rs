#[doc = "Reader of register ECCCTRL"]
pub type R = crate::R<u32, super::ECCCTRL>;
#[doc = "Writer for register ECCCTRL"]
pub type W = crate::W<u32, super::ECCCTRL>;
#[doc = "Register ECCCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::ECCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RAMECCEWEN`"]
pub type RAMECCEWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMECCEWEN`"]
pub struct RAMECCEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMECCEWEN_W<'a> {
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
#[doc = "Reader of field `RAMECCCHKEN`"]
pub type RAMECCCHKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAMECCCHKEN`"]
pub struct RAMECCCHKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMECCCHKEN_W<'a> {
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
#[doc = "Reader of field `RAM1ECCEWEN`"]
pub type RAM1ECCEWEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1ECCEWEN`"]
pub struct RAM1ECCEWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1ECCEWEN_W<'a> {
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
#[doc = "Reader of field `RAM1ECCCHKEN`"]
pub type RAM1ECCCHKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RAM1ECCCHKEN`"]
pub struct RAM1ECCCHKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1ECCCHKEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&self) -> RAMECCEWEN_R {
        RAMECCEWEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&self) -> RAMECCCHKEN_R {
        RAMECCCHKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&self) -> RAM1ECCEWEN_R {
        RAM1ECCEWEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&self) -> RAM1ECCCHKEN_R {
        RAM1ECCCHKEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM ECC Write Enable"]
    #[inline(always)]
    pub fn rameccewen(&mut self) -> RAMECCEWEN_W {
        RAMECCEWEN_W { w: self }
    }
    #[doc = "Bit 1 - RAM ECC Check Enable"]
    #[inline(always)]
    pub fn rameccchken(&mut self) -> RAMECCCHKEN_W {
        RAMECCCHKEN_W { w: self }
    }
    #[doc = "Bit 2 - RAM1 ECC Write Enable"]
    #[inline(always)]
    pub fn ram1eccewen(&mut self) -> RAM1ECCEWEN_W {
        RAM1ECCEWEN_W { w: self }
    }
    #[doc = "Bit 3 - RAM1 ECC Check Enable"]
    #[inline(always)]
    pub fn ram1eccchken(&mut self) -> RAM1ECCCHKEN_W {
        RAM1ECCCHKEN_W { w: self }
    }
}
