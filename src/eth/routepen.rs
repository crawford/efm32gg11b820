#[doc = "Reader of register ROUTEPEN"]
pub type R = crate::R<u32, super::ROUTEPEN>;
#[doc = "Writer for register ROUTEPEN"]
pub type W = crate::W<u32, super::ROUTEPEN>;
#[doc = "Register ROUTEPEN `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTEPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MDIOPEN`"]
pub type MDIOPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MDIOPEN`"]
pub struct MDIOPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MDIOPEN_W<'a> {
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
#[doc = "Reader of field `MIITXERPEN`"]
pub type MIITXERPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIITXERPEN`"]
pub struct MIITXERPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIITXERPEN_W<'a> {
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
#[doc = "Reader of field `MIIRXERPEN`"]
pub type MIIRXERPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIIRXERPEN`"]
pub struct MIIRXERPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIRXERPEN_W<'a> {
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
#[doc = "Reader of field `MIIPEN`"]
pub type MIIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MIIPEN`"]
pub struct MIIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIPEN_W<'a> {
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
#[doc = "Reader of field `RMIIPEN`"]
pub type RMIIPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMIIPEN`"]
pub struct RMIIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIIPEN_W<'a> {
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
#[doc = "Reader of field `TSUTMRTOGPEN`"]
pub type TSUTMRTOGPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSUTMRTOGPEN`"]
pub struct TSUTMRTOGPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSUTMRTOGPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&self) -> MDIOPEN_R {
        MDIOPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&self) -> MIITXERPEN_R {
        MIITXERPEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&self) -> MIIRXERPEN_R {
        MIIRXERPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&self) -> MIIPEN_R {
        MIIPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&self) -> RMIIPEN_R {
        RMIIPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&self) -> TSUTMRTOGPEN_R {
        TSUTMRTOGPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDIO I/O Enable"]
    #[inline(always)]
    pub fn mdiopen(&mut self) -> MDIOPEN_W {
        MDIOPEN_W { w: self }
    }
    #[doc = "Bit 1 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miitxerpen(&mut self) -> MIITXERPEN_W {
        MIITXERPEN_W { w: self }
    }
    #[doc = "Bit 2 - MII TX ER I/O Enable"]
    #[inline(always)]
    pub fn miirxerpen(&mut self) -> MIIRXERPEN_W {
        MIIRXERPEN_W { w: self }
    }
    #[doc = "Bit 3 - MII I/O Enable"]
    #[inline(always)]
    pub fn miipen(&mut self) -> MIIPEN_W {
        MIIPEN_W { w: self }
    }
    #[doc = "Bit 4 - RMII I/O Enable"]
    #[inline(always)]
    pub fn rmiipen(&mut self) -> RMIIPEN_W {
        RMIIPEN_W { w: self }
    }
    #[doc = "Bit 5 - TSU_TMR_CNT_SEC Output Enable"]
    #[inline(always)]
    pub fn tsutmrtogpen(&mut self) -> TSUTMRTOGPEN_W {
        TSUTMRTOGPEN_W { w: self }
    }
}
