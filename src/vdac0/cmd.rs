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
#[doc = "Write proxy for field `CH0EN`"]
pub struct CH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0EN_W<'a> {
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
#[doc = "Write proxy for field `CH0DIS`"]
pub struct CH0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0DIS_W<'a> {
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
#[doc = "Write proxy for field `CH1EN`"]
pub struct CH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1EN_W<'a> {
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
#[doc = "Write proxy for field `CH1DIS`"]
pub struct CH1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1DIS_W<'a> {
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
#[doc = "Write proxy for field `OPA0EN`"]
pub struct OPA0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Write proxy for field `OPA0DIS`"]
pub struct OPA0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Write proxy for field `OPA1EN`"]
pub struct OPA1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Write proxy for field `OPA1DIS`"]
pub struct OPA1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Write proxy for field `OPA2EN`"]
pub struct OPA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Write proxy for field `OPA2DIS`"]
pub struct OPA2DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Write proxy for field `OPA3EN`"]
pub struct OPA3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Write proxy for field `OPA3DIS`"]
pub struct OPA3DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W { w: self }
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0dis(&mut self) -> CH0DIS_W {
        CH0DIS_W { w: self }
    }
    #[doc = "Bit 2 - DAC Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W { w: self }
    }
    #[doc = "Bit 3 - DAC Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1dis(&mut self) -> CH1DIS_W {
        CH1DIS_W { w: self }
    }
    #[doc = "Bit 16 - OPA0 Enable"]
    #[inline(always)]
    pub fn opa0en(&mut self) -> OPA0EN_W {
        OPA0EN_W { w: self }
    }
    #[doc = "Bit 17 - OPA0 Disable"]
    #[inline(always)]
    pub fn opa0dis(&mut self) -> OPA0DIS_W {
        OPA0DIS_W { w: self }
    }
    #[doc = "Bit 18 - OPA1 Enable"]
    #[inline(always)]
    pub fn opa1en(&mut self) -> OPA1EN_W {
        OPA1EN_W { w: self }
    }
    #[doc = "Bit 19 - OPA1 Disable"]
    #[inline(always)]
    pub fn opa1dis(&mut self) -> OPA1DIS_W {
        OPA1DIS_W { w: self }
    }
    #[doc = "Bit 20 - OPA2 Enable"]
    #[inline(always)]
    pub fn opa2en(&mut self) -> OPA2EN_W {
        OPA2EN_W { w: self }
    }
    #[doc = "Bit 21 - OPA2 Disable"]
    #[inline(always)]
    pub fn opa2dis(&mut self) -> OPA2DIS_W {
        OPA2DIS_W { w: self }
    }
    #[doc = "Bit 22 - OPA3 Enable"]
    #[inline(always)]
    pub fn opa3en(&mut self) -> OPA3EN_W {
        OPA3EN_W { w: self }
    }
    #[doc = "Bit 23 - OPA3 Disable"]
    #[inline(always)]
    pub fn opa3dis(&mut self) -> OPA3DIS_W {
        OPA3DIS_W { w: self }
    }
}
