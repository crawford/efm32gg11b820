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
#[doc = "Write proxy for field `VMONAVDDFALL`"]
pub struct VMONAVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONAVDDFALL_W<'a> {
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
#[doc = "Write proxy for field `VMONAVDDRISE`"]
pub struct VMONAVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONAVDDRISE_W<'a> {
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
#[doc = "Write proxy for field `VMONALTAVDDFALL`"]
pub struct VMONALTAVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONALTAVDDFALL_W<'a> {
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
#[doc = "Write proxy for field `VMONALTAVDDRISE`"]
pub struct VMONALTAVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONALTAVDDRISE_W<'a> {
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
#[doc = "Write proxy for field `VMONDVDDFALL`"]
pub struct VMONDVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONDVDDFALL_W<'a> {
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
#[doc = "Write proxy for field `VMONDVDDRISE`"]
pub struct VMONDVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONDVDDRISE_W<'a> {
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
#[doc = "Write proxy for field `VMONIO0FALL`"]
pub struct VMONIO0FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO0FALL_W<'a> {
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
#[doc = "Write proxy for field `VMONIO0RISE`"]
pub struct VMONIO0RISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO0RISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write proxy for field `VMONIO1FALL`"]
pub struct VMONIO1FALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO1FALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `VMONIO1RISE`"]
pub struct VMONIO1RISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONIO1RISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Write proxy for field `R5VREADY`"]
pub struct R5VREADY_W<'a> {
    w: &'a mut W,
}
impl<'a> R5VREADY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `VMONBUVDDFALL`"]
pub struct VMONBUVDDFALL_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONBUVDDFALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `VMONBUVDDRISE`"]
pub struct VMONBUVDDRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> VMONBUVDDRISE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `PFETOVERCURRENTLIMIT`"]
pub struct PFETOVERCURRENTLIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> PFETOVERCURRENTLIMIT_W<'a> {
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
#[doc = "Write proxy for field `NFETOVERCURRENTLIMIT`"]
pub struct NFETOVERCURRENTLIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NFETOVERCURRENTLIMIT_W<'a> {
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
#[doc = "Write proxy for field `DCDCLPRUNNING`"]
pub struct DCDCLPRUNNING_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCLPRUNNING_W<'a> {
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
#[doc = "Write proxy for field `DCDCLNRUNNING`"]
pub struct DCDCLNRUNNING_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCLNRUNNING_W<'a> {
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
#[doc = "Write proxy for field `DCDCINBYPASS`"]
pub struct DCDCINBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDCINBYPASS_W<'a> {
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
#[doc = "Write proxy for field `BURDY`"]
pub struct BURDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BURDY_W<'a> {
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
#[doc = "Write proxy for field `R5VVSINT`"]
pub struct R5VVSINT_W<'a> {
    w: &'a mut W,
}
impl<'a> R5VVSINT_W<'a> {
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
#[doc = "Write proxy for field `EM23WAKEUP`"]
pub struct EM23WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> EM23WAKEUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `VSCALEDONE`"]
pub struct VSCALEDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> VSCALEDONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Write proxy for field `TEMP`"]
pub struct TEMP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMP_W<'a> {
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
#[doc = "Write proxy for field `TEMPLOW`"]
pub struct TEMPLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPLOW_W<'a> {
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
#[doc = "Write proxy for field `TEMPHIGH`"]
pub struct TEMPHIGH_W<'a> {
    w: &'a mut W,
}
impl<'a> TEMPHIGH_W<'a> {
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
    #[doc = "Bit 0 - Set VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W {
        VMONAVDDFALL_W { w: self }
    }
    #[doc = "Bit 1 - Set VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W {
        VMONAVDDRISE_W { w: self }
    }
    #[doc = "Bit 2 - Set VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W {
        VMONALTAVDDFALL_W { w: self }
    }
    #[doc = "Bit 3 - Set VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W {
        VMONALTAVDDRISE_W { w: self }
    }
    #[doc = "Bit 4 - Set VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W {
        VMONDVDDFALL_W { w: self }
    }
    #[doc = "Bit 5 - Set VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W {
        VMONDVDDRISE_W { w: self }
    }
    #[doc = "Bit 6 - Set VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W {
        VMONIO0FALL_W { w: self }
    }
    #[doc = "Bit 7 - Set VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W {
        VMONIO0RISE_W { w: self }
    }
    #[doc = "Bit 8 - Set VMONIO1FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio1fall(&mut self) -> VMONIO1FALL_W {
        VMONIO1FALL_W { w: self }
    }
    #[doc = "Bit 9 - Set VMONIO1RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio1rise(&mut self) -> VMONIO1RISE_W {
        VMONIO1RISE_W { w: self }
    }
    #[doc = "Bit 10 - Set R5VREADY Interrupt Flag"]
    #[inline(always)]
    pub fn r5vready(&mut self) -> R5VREADY_W {
        R5VREADY_W { w: self }
    }
    #[doc = "Bit 12 - Set VMONBUVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W {
        VMONBUVDDFALL_W { w: self }
    }
    #[doc = "Bit 13 - Set VMONBUVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W {
        VMONBUVDDRISE_W { w: self }
    }
    #[doc = "Bit 16 - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W {
        PFETOVERCURRENTLIMIT_W { w: self }
    }
    #[doc = "Bit 17 - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W {
        NFETOVERCURRENTLIMIT_W { w: self }
    }
    #[doc = "Bit 18 - Set DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W {
        DCDCLPRUNNING_W { w: self }
    }
    #[doc = "Bit 19 - Set DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W {
        DCDCLNRUNNING_W { w: self }
    }
    #[doc = "Bit 20 - Set DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W {
        DCDCINBYPASS_W { w: self }
    }
    #[doc = "Bit 22 - Set BURDY Interrupt Flag"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BURDY_W {
        BURDY_W { w: self }
    }
    #[doc = "Bit 23 - Set R5VVSINT Interrupt Flag"]
    #[inline(always)]
    pub fn r5vvsint(&mut self) -> R5VVSINT_W {
        R5VVSINT_W { w: self }
    }
    #[doc = "Bit 24 - Set EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W {
        EM23WAKEUP_W { w: self }
    }
    #[doc = "Bit 25 - Set VSCALEDONE Interrupt Flag"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W {
        VSCALEDONE_W { w: self }
    }
    #[doc = "Bit 29 - Set TEMP Interrupt Flag"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W {
        TEMP_W { w: self }
    }
    #[doc = "Bit 30 - Set TEMPLOW Interrupt Flag"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W { w: self }
    }
    #[doc = "Bit 31 - Set TEMPHIGH Interrupt Flag"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TEMPHIGH_W {
        TEMPHIGH_W { w: self }
    }
}
