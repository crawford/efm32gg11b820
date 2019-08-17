#[doc = "Writer for register CACHECMD"]
pub type W = crate::W<u32, super::CACHECMD>;
#[doc = "Register CACHECMD `reset()`'s with value 0"]
impl crate::ResetValue for super::CACHECMD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `INVCACHE`"]
pub struct INVCACHE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVCACHE_W<'a> {
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
#[doc = "Write proxy for field `STARTPC`"]
pub struct STARTPC_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTPC_W<'a> {
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
#[doc = "Write proxy for field `STOPPC`"]
pub struct STOPPC_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Invalidate Instruction Cache"]
    #[inline(always)]
    pub fn invcache(&mut self) -> INVCACHE_W {
        INVCACHE_W { w: self }
    }
    #[doc = "Bit 1 - Start Performance Counters"]
    #[inline(always)]
    pub fn startpc(&mut self) -> STARTPC_W {
        STARTPC_W { w: self }
    }
    #[doc = "Bit 2 - Stop Performance Counters"]
    #[inline(always)]
    pub fn stoppc(&mut self) -> STOPPC_W {
        STOPPC_W { w: self }
    }
}
