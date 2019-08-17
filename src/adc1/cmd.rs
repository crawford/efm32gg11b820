#[doc = "Writer for register CMD"]
pub type W = crate::W<u32, super::CMD>;
#[doc = "Register CMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SINGLESTART`"]
pub struct SINGLESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLESTART_W<'a> {
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
#[doc = "Write proxy for field `SINGLESTOP`"]
pub struct SINGLESTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLESTOP_W<'a> {
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
#[doc = "Write proxy for field `SCANSTART`"]
pub struct SCANSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANSTART_W<'a> {
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
#[doc = "Write proxy for field `SCANSTOP`"]
pub struct SCANSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANSTOP_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Single Channel Conversion Start"]
    #[inline(always)]
    pub fn singlestart(&mut self) -> SINGLESTART_W {
        SINGLESTART_W { w: self }
    }
    #[doc = "Bit 1 - Single Channel Conversion Stop"]
    #[inline(always)]
    pub fn singlestop(&mut self) -> SINGLESTOP_W {
        SINGLESTOP_W { w: self }
    }
    #[doc = "Bit 2 - Scan Sequence Start"]
    #[inline(always)]
    pub fn scanstart(&mut self) -> SCANSTART_W {
        SCANSTART_W { w: self }
    }
    #[doc = "Bit 3 - Scan Sequence Stop"]
    #[inline(always)]
    pub fn scanstop(&mut self) -> SCANSTOP_W {
        SCANSTOP_W { w: self }
    }
}
