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
#[doc = "Write proxy for field `EM4UNLATCH`"]
pub struct EM4UNLATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4UNLATCH_W<'a> {
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
#[doc = "Write proxy for field `EM01VSCALE0`"]
pub struct EM01VSCALE0_W<'a> {
    w: &'a mut W,
}
impl<'a> EM01VSCALE0_W<'a> {
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
#[doc = "Write proxy for field `EM01VSCALE2`"]
pub struct EM01VSCALE2_W<'a> {
    w: &'a mut W,
}
impl<'a> EM01VSCALE2_W<'a> {
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
impl W {
    #[doc = "Bit 0 - EM4 Unlatch"]
    #[inline(always)]
    pub fn em4unlatch(&mut self) -> EM4UNLATCH_W {
        EM4UNLATCH_W { w: self }
    }
    #[doc = "Bit 4 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 0"]
    #[inline(always)]
    pub fn em01vscale0(&mut self) -> EM01VSCALE0_W {
        EM01VSCALE0_W { w: self }
    }
    #[doc = "Bit 6 - EM01 Voltage Scale Command to Scale to Voltage Scale Level 2"]
    #[inline(always)]
    pub fn em01vscale2(&mut self) -> EM01VSCALE2_W {
        EM01VSCALE2_W { w: self }
    }
}
