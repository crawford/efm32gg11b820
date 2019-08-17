#[doc = "Reader of register PCGCCTL"]
pub type R = crate::R<u32, super::PCGCCTL>;
#[doc = "Writer for register PCGCCTL"]
pub type W = crate::W<u32, super::PCGCCTL>;
#[doc = "Register PCGCCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::PCGCCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STOPPCLK`"]
pub type STOPPCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOPPCLK`"]
pub struct STOPPCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPCLK_W<'a> {
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
#[doc = "Reader of field `GATEHCLK`"]
pub type GATEHCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GATEHCLK`"]
pub struct GATEHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> GATEHCLK_W<'a> {
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
#[doc = "Reader of field `PWRCLMP`"]
pub type PWRCLMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRCLMP`"]
pub struct PWRCLMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCLMP_W<'a> {
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
#[doc = "Reader of field `RSTPDWNMODULE`"]
pub type RSTPDWNMODULE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RSTPDWNMODULE`"]
pub struct RSTPDWNMODULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTPDWNMODULE_W<'a> {
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
#[doc = "Reader of field `PHYSLEEP`"]
pub type PHYSLEEP_R = crate::R<bool, bool>;
#[doc = "Reader of field `L1SUSPENDED`"]
pub type L1SUSPENDED_R = crate::R<bool, bool>;
#[doc = "Reader of field `RESETAFTERSUSP`"]
pub type RESETAFTERSUSP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stoppclk(&self) -> STOPPCLK_R {
        STOPPCLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Power Clamp"]
    #[inline(always)]
    pub fn pwrclmp(&self) -> PWRCLMP_R {
        PWRCLMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Power-Down Modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&self) -> RSTPDWNMODULE_R {
        RSTPDWNMODULE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn physleep(&self) -> PHYSLEEP_R {
        PHYSLEEP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn l1suspended(&self) -> L1SUSPENDED_R {
        L1SUSPENDED_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn resetaftersusp(&self) -> RESETAFTERSUSP_R {
        RESETAFTERSUSP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Pclk"]
    #[inline(always)]
    pub fn stoppclk(&mut self) -> STOPPCLK_W {
        STOPPCLK_W { w: self }
    }
    #[doc = "Bit 1 - Gate Hclk"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W {
        GATEHCLK_W { w: self }
    }
    #[doc = "Bit 2 - Power Clamp"]
    #[inline(always)]
    pub fn pwrclmp(&mut self) -> PWRCLMP_W {
        PWRCLMP_W { w: self }
    }
    #[doc = "Bit 3 - Reset Power-Down Modules"]
    #[inline(always)]
    pub fn rstpdwnmodule(&mut self) -> RSTPDWNMODULE_W {
        RSTPDWNMODULE_W { w: self }
    }
}
