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
#[doc = "Write proxy for field `CH0CD`"]
pub struct CH0CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0CD_W<'a> {
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
#[doc = "Write proxy for field `CH1CD`"]
pub struct CH1CD_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1CD_W<'a> {
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
#[doc = "Write proxy for field `CH0OF`"]
pub struct CH0OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OF_W<'a> {
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
#[doc = "Write proxy for field `CH1OF`"]
pub struct CH1OF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1OF_W<'a> {
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
#[doc = "Write proxy for field `CH0UF`"]
pub struct CH0UF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0UF_W<'a> {
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
#[doc = "Write proxy for field `CH1UF`"]
pub struct CH1UF_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1UF_W<'a> {
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
#[doc = "Write proxy for field `EM23ERR`"]
pub struct EM23ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `OPA0APORTCONFLICT`"]
pub struct OPA0APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0APORTCONFLICT_W<'a> {
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
#[doc = "Write proxy for field `OPA1APORTCONFLICT`"]
pub struct OPA1APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1APORTCONFLICT_W<'a> {
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
#[doc = "Write proxy for field `OPA2APORTCONFLICT`"]
pub struct OPA2APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2APORTCONFLICT_W<'a> {
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
#[doc = "Write proxy for field `OPA3APORTCONFLICT`"]
pub struct OPA3APORTCONFLICT_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3APORTCONFLICT_W<'a> {
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
#[doc = "Write proxy for field `OPA0PRSTIMEDERR`"]
pub struct OPA0PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0PRSTIMEDERR_W<'a> {
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
#[doc = "Write proxy for field `OPA1PRSTIMEDERR`"]
pub struct OPA1PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1PRSTIMEDERR_W<'a> {
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
#[doc = "Write proxy for field `OPA2PRSTIMEDERR`"]
pub struct OPA2PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2PRSTIMEDERR_W<'a> {
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
#[doc = "Write proxy for field `OPA3PRSTIMEDERR`"]
pub struct OPA3PRSTIMEDERR_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3PRSTIMEDERR_W<'a> {
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
#[doc = "Write proxy for field `OPA0OUTVALID`"]
pub struct OPA0OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA0OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Write proxy for field `OPA1OUTVALID`"]
pub struct OPA1OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA1OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Write proxy for field `OPA2OUTVALID`"]
pub struct OPA2OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA2OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Write proxy for field `OPA3OUTVALID`"]
pub struct OPA3OUTVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> OPA3OUTVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Set CH0CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> CH0CD_W {
        CH0CD_W { w: self }
    }
    #[doc = "Bit 1 - Set CH1CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> CH1CD_W {
        CH1CD_W { w: self }
    }
    #[doc = "Bit 2 - Set CH0OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W { w: self }
    }
    #[doc = "Bit 3 - Set CH1OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W { w: self }
    }
    #[doc = "Bit 4 - Set CH0UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> CH0UF_W {
        CH0UF_W { w: self }
    }
    #[doc = "Bit 5 - Set CH1UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> CH1UF_W {
        CH1UF_W { w: self }
    }
    #[doc = "Bit 15 - Set EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W { w: self }
    }
    #[doc = "Bit 16 - Set OPA0APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W {
        OPA0APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 17 - Set OPA1APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W {
        OPA1APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 18 - Set OPA2APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W {
        OPA2APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 19 - Set OPA3APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&mut self) -> OPA3APORTCONFLICT_W {
        OPA3APORTCONFLICT_W { w: self }
    }
    #[doc = "Bit 20 - Set OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W {
        OPA0PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 21 - Set OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W {
        OPA1PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 22 - Set OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W {
        OPA2PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 23 - Set OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&mut self) -> OPA3PRSTIMEDERR_W {
        OPA3PRSTIMEDERR_W { w: self }
    }
    #[doc = "Bit 28 - Set OPA0OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W {
        OPA0OUTVALID_W { w: self }
    }
    #[doc = "Bit 29 - Set OPA1OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W {
        OPA1OUTVALID_W { w: self }
    }
    #[doc = "Bit 30 - Set OPA2OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W {
        OPA2OUTVALID_W { w: self }
    }
    #[doc = "Bit 31 - Set OPA3OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&mut self) -> OPA3OUTVALID_W {
        OPA3OUTVALID_W { w: self }
    }
}
