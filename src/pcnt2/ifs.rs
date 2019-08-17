#[doc = "Writer for register IFS"]
pub type W = crate::W<u32, super::IFS>;
#[doc = "Register IFS `reset()`'s with value 0"]
impl crate::ResetValue for super::IFS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `UF`"]
pub struct UF_W<'a> {
    w: &'a mut W,
}
impl<'a> UF_W<'a> {
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
#[doc = "Write proxy for field `OF`"]
pub struct OF_W<'a> {
    w: &'a mut W,
}
impl<'a> OF_W<'a> {
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
#[doc = "Write proxy for field `DIRCNG`"]
pub struct DIRCNG_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRCNG_W<'a> {
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
#[doc = "Write proxy for field `AUXOF`"]
pub struct AUXOF_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXOF_W<'a> {
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
#[doc = "Write proxy for field `TCC`"]
pub struct TCC_W<'a> {
    w: &'a mut W,
}
impl<'a> TCC_W<'a> {
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
#[doc = "Write proxy for field `OQSTERR`"]
pub struct OQSTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OQSTERR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Set UF Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W { w: self }
    }
    #[doc = "Bit 1 - Set OF Interrupt Flag"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W { w: self }
    }
    #[doc = "Bit 2 - Set DIRCNG Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&mut self) -> DIRCNG_W {
        DIRCNG_W { w: self }
    }
    #[doc = "Bit 3 - Set AUXOF Interrupt Flag"]
    #[inline(always)]
    pub fn auxof(&mut self) -> AUXOF_W {
        AUXOF_W { w: self }
    }
    #[doc = "Bit 4 - Set TCC Interrupt Flag"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TCC_W {
        TCC_W { w: self }
    }
    #[doc = "Bit 5 - Set OQSTERR Interrupt Flag"]
    #[inline(always)]
    pub fn oqsterr(&mut self) -> OQSTERR_W {
        OQSTERR_W { w: self }
    }
}
