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
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
#[doc = "Write proxy for field `CTO0`"]
pub struct CTO0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTO0_W<'a> {
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
#[doc = "Write proxy for field `CTO1`"]
pub struct CTO1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTO1_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Start LETIMER"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 1 - Stop LETIMER"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 2 - Clear LETIMER"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Clear Toggle Output 0"]
    #[inline(always)]
    pub fn cto0(&mut self) -> CTO0_W {
        CTO0_W { w: self }
    }
    #[doc = "Bit 4 - Clear Toggle Output 1"]
    #[inline(always)]
    pub fn cto1(&mut self) -> CTO1_W {
        CTO1_W { w: self }
    }
}
