#[doc = "Reader of register CFG1"]
pub type R = crate::R<u32, super::CFG1>;
#[doc = "Writer for register CFG1"]
pub type W = crate::W<u32, super::CFG1>;
#[doc = "Register CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ASYNCINTRSUP`"]
pub type ASYNCINTRSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCINTRSUP`"]
pub struct ASYNCINTRSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCINTRSUP_W<'a> {
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
#[doc = "Possible values of the field `SLOTTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOTTYPE_A {
    #[doc = "Removable SD Card Slot"]
    RMSDSLOT,
    #[doc = "Embedded SD Card Slot"]
    EMSDSLOT,
    #[doc = "Shared SD Card Slot"]
    SHBUSSLOT,
}
impl crate::ToBits<u8> for SLOTTYPE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SLOTTYPE_A::RMSDSLOT => 0,
            SLOTTYPE_A::EMSDSLOT => 1,
            SLOTTYPE_A::SHBUSSLOT => 2,
        }
    }
}
#[doc = "Reader of field `SLOTTYPE`"]
pub type SLOTTYPE_R = crate::R<u8, SLOTTYPE_A>;
impl SLOTTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SLOTTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SLOTTYPE_A::RMSDSLOT),
            1 => Val(SLOTTYPE_A::EMSDSLOT),
            2 => Val(SLOTTYPE_A::SHBUSSLOT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RMSDSLOT`"]
    #[inline(always)]
    pub fn is_rmsdslot(&self) -> bool {
        *self == SLOTTYPE_A::RMSDSLOT
    }
    #[doc = "Checks if the value of the field is `EMSDSLOT`"]
    #[inline(always)]
    pub fn is_emsdslot(&self) -> bool {
        *self == SLOTTYPE_A::EMSDSLOT
    }
    #[doc = "Checks if the value of the field is `SHBUSSLOT`"]
    #[inline(always)]
    pub fn is_shbusslot(&self) -> bool {
        *self == SLOTTYPE_A::SHBUSSLOT
    }
}
#[doc = "Write proxy for field `SLOTTYPE`"]
pub struct SLOTTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLOTTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLOTTYPE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Removable SD Card Slot"]
    #[inline(always)]
    pub fn rmsdslot(self) -> &'a mut W {
        self.variant(SLOTTYPE_A::RMSDSLOT)
    }
    #[doc = "Embedded SD Card Slot"]
    #[inline(always)]
    pub fn emsdslot(self) -> &'a mut W {
        self.variant(SLOTTYPE_A::EMSDSLOT)
    }
    #[doc = "Shared SD Card Slot"]
    #[inline(always)]
    pub fn shbusslot(self) -> &'a mut W {
        self.variant(SLOTTYPE_A::SHBUSSLOT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `CSDR50SUP`"]
pub type CSDR50SUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSDR50SUP`"]
pub struct CSDR50SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDR50SUP_W<'a> {
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
#[doc = "Reader of field `CSDR104SUP`"]
pub type CSDR104SUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSDR104SUP`"]
pub struct CSDR104SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSDR104SUP_W<'a> {
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
#[doc = "Reader of field `CDDR50SUP`"]
pub type CDDR50SUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDDR50SUP`"]
pub struct CDDR50SUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CDDR50SUP_W<'a> {
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
#[doc = "Reader of field `CDRVASUP`"]
pub type CDRVASUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDRVASUP`"]
pub struct CDRVASUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CDRVASUP_W<'a> {
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
#[doc = "Reader of field `CDRVCSUP`"]
pub type CDRVCSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDRVCSUP`"]
pub struct CDRVCSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CDRVCSUP_W<'a> {
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
#[doc = "Reader of field `CDRVDSUP`"]
pub type CDRVDSUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDRVDSUP`"]
pub struct CDRVDSUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CDRVDSUP_W<'a> {
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
#[doc = "Reader of field `RETUNTMRCTL`"]
pub type RETUNTMRCTL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETUNTMRCTL`"]
pub struct RETUNTMRCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RETUNTMRCTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Reader of field `TUNSDR50`"]
pub type TUNSDR50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TUNSDR50`"]
pub struct TUNSDR50_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNSDR50_W<'a> {
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
#[doc = "Reader of field `RETUNMODES`"]
pub type RETUNMODES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RETUNMODES`"]
pub struct RETUNMODES_W<'a> {
    w: &'a mut W,
}
impl<'a> RETUNMODES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPISUP`"]
pub type SPISUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPISUP`"]
pub struct SPISUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPISUP_W<'a> {
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
#[doc = "Reader of field `ASYNCWKUPEN`"]
pub type ASYNCWKUPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCWKUPEN`"]
pub struct ASYNCWKUPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCWKUPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintrsup(&self) -> ASYNCINTRSUP_R {
        ASYNCINTRSUP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline(always)]
    pub fn slottype(&self) -> SLOTTYPE_R {
        SLOTTYPE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline(always)]
    pub fn csdr50sup(&self) -> CSDR50SUP_R {
        CSDR50SUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline(always)]
    pub fn csdr104sup(&self) -> CSDR104SUP_R {
        CSDR104SUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline(always)]
    pub fn cddr50sup(&self) -> CDDR50SUP_R {
        CDDR50SUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline(always)]
    pub fn cdrvasup(&self) -> CDRVASUP_R {
        CDRVASUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline(always)]
    pub fn cdrvcsup(&self) -> CDRVCSUP_R {
        CDRVCSUP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline(always)]
    pub fn cdrvdsup(&self) -> CDRVDSUP_R {
        CDRVDSUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline(always)]
    pub fn retuntmrctl(&self) -> RETUNTMRCTL_R {
        RETUNTMRCTL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline(always)]
    pub fn tunsdr50(&self) -> TUNSDR50_R {
        TUNSDR50_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline(always)]
    pub fn retunmodes(&self) -> RETUNMODES_R {
        RETUNMODES_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline(always)]
    pub fn spisup(&self) -> SPISUP_R {
        SPISUP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline(always)]
    pub fn asyncwkupen(&self) -> ASYNCWKUPEN_R {
        ASYNCWKUPEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline(always)]
    pub fn asyncintrsup(&mut self) -> ASYNCINTRSUP_W {
        ASYNCINTRSUP_W { w: self }
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline(always)]
    pub fn slottype(&mut self) -> SLOTTYPE_W {
        SLOTTYPE_W { w: self }
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline(always)]
    pub fn csdr50sup(&mut self) -> CSDR50SUP_W {
        CSDR50SUP_W { w: self }
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline(always)]
    pub fn csdr104sup(&mut self) -> CSDR104SUP_W {
        CSDR104SUP_W { w: self }
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline(always)]
    pub fn cddr50sup(&mut self) -> CDDR50SUP_W {
        CDDR50SUP_W { w: self }
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline(always)]
    pub fn cdrvasup(&mut self) -> CDRVASUP_W {
        CDRVASUP_W { w: self }
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline(always)]
    pub fn cdrvcsup(&mut self) -> CDRVCSUP_W {
        CDRVCSUP_W { w: self }
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline(always)]
    pub fn cdrvdsup(&mut self) -> CDRVDSUP_W {
        CDRVDSUP_W { w: self }
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline(always)]
    pub fn retuntmrctl(&mut self) -> RETUNTMRCTL_W {
        RETUNTMRCTL_W { w: self }
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline(always)]
    pub fn tunsdr50(&mut self) -> TUNSDR50_W {
        TUNSDR50_W { w: self }
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline(always)]
    pub fn retunmodes(&mut self) -> RETUNMODES_W {
        RETUNMODES_W { w: self }
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline(always)]
    pub fn spisup(&mut self) -> SPISUP_W {
        SPISUP_W { w: self }
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline(always)]
    pub fn asyncwkupen(&mut self) -> ASYNCWKUPEN_W {
        ASYNCWKUPEN_W { w: self }
    }
}
