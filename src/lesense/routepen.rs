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
#[doc = "Reader of field `CH0PEN`"]
pub type CH0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0PEN`"]
pub struct CH0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PEN_W<'a> {
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
#[doc = "Reader of field `CH1PEN`"]
pub type CH1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1PEN`"]
pub struct CH1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PEN_W<'a> {
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
#[doc = "Reader of field `CH2PEN`"]
pub type CH2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2PEN`"]
pub struct CH2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PEN_W<'a> {
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
#[doc = "Reader of field `CH3PEN`"]
pub type CH3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3PEN`"]
pub struct CH3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PEN_W<'a> {
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
#[doc = "Reader of field `CH4PEN`"]
pub type CH4PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4PEN`"]
pub struct CH4PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4PEN_W<'a> {
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
#[doc = "Reader of field `CH5PEN`"]
pub type CH5PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5PEN`"]
pub struct CH5PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5PEN_W<'a> {
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
#[doc = "Reader of field `CH6PEN`"]
pub type CH6PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6PEN`"]
pub struct CH6PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6PEN_W<'a> {
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
#[doc = "Reader of field `CH7PEN`"]
pub type CH7PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7PEN`"]
pub struct CH7PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7PEN_W<'a> {
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
#[doc = "Reader of field `CH8PEN`"]
pub type CH8PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH8PEN`"]
pub struct CH8PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8PEN_W<'a> {
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
#[doc = "Reader of field `CH9PEN`"]
pub type CH9PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH9PEN`"]
pub struct CH9PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9PEN_W<'a> {
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
#[doc = "Reader of field `CH10PEN`"]
pub type CH10PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH10PEN`"]
pub struct CH10PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10PEN_W<'a> {
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
#[doc = "Reader of field `CH11PEN`"]
pub type CH11PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH11PEN`"]
pub struct CH11PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CH12PEN`"]
pub type CH12PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH12PEN`"]
pub struct CH12PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12PEN_W<'a> {
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
#[doc = "Reader of field `CH13PEN`"]
pub type CH13PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH13PEN`"]
pub struct CH13PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13PEN_W<'a> {
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
#[doc = "Reader of field `CH14PEN`"]
pub type CH14PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH14PEN`"]
pub struct CH14PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14PEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `CH15PEN`"]
pub type CH15PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH15PEN`"]
pub struct CH15PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX0PEN`"]
pub type ALTEX0PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX0PEN`"]
pub struct ALTEX0PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX0PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX1PEN`"]
pub type ALTEX1PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX1PEN`"]
pub struct ALTEX1PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX1PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX2PEN`"]
pub type ALTEX2PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX2PEN`"]
pub struct ALTEX2PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX2PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX3PEN`"]
pub type ALTEX3PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX3PEN`"]
pub struct ALTEX3PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX3PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX4PEN`"]
pub type ALTEX4PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX4PEN`"]
pub struct ALTEX4PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX4PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX5PEN`"]
pub type ALTEX5PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX5PEN`"]
pub struct ALTEX5PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX5PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX6PEN`"]
pub type ALTEX6PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX6PEN`"]
pub struct ALTEX6PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX6PEN_W<'a> {
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
#[doc = "Reader of field `ALTEX7PEN`"]
pub type ALTEX7PEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX7PEN`"]
pub struct ALTEX7PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX7PEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> CH0PEN_R {
        CH0PEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> CH1PEN_R {
        CH1PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> CH2PEN_R {
        CH2PEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> CH3PEN_R {
        CH3PEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&self) -> CH4PEN_R {
        CH4PEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&self) -> CH5PEN_R {
        CH5PEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&self) -> CH6PEN_R {
        CH6PEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&self) -> CH7PEN_R {
        CH7PEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&self) -> CH8PEN_R {
        CH8PEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&self) -> CH9PEN_R {
        CH9PEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&self) -> CH10PEN_R {
        CH10PEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&self) -> CH11PEN_R {
        CH11PEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - CH12 Pin Enable"]
    #[inline(always)]
    pub fn ch12pen(&self) -> CH12PEN_R {
        CH12PEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CH13 Pin Enable"]
    #[inline(always)]
    pub fn ch13pen(&self) -> CH13PEN_R {
        CH13PEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CH14 Pin Enable"]
    #[inline(always)]
    pub fn ch14pen(&self) -> CH14PEN_R {
        CH14PEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CH15 Pin Enable"]
    #[inline(always)]
    pub fn ch15pen(&self) -> CH15PEN_R {
        CH15PEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ALTEX0 Pin Enable"]
    #[inline(always)]
    pub fn altex0pen(&self) -> ALTEX0PEN_R {
        ALTEX0PEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ALTEX1 Pin Enable"]
    #[inline(always)]
    pub fn altex1pen(&self) -> ALTEX1PEN_R {
        ALTEX1PEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ALTEX2 Pin Enable"]
    #[inline(always)]
    pub fn altex2pen(&self) -> ALTEX2PEN_R {
        ALTEX2PEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ALTEX3 Pin Enable"]
    #[inline(always)]
    pub fn altex3pen(&self) -> ALTEX3PEN_R {
        ALTEX3PEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ALTEX4 Pin Enable"]
    #[inline(always)]
    pub fn altex4pen(&self) -> ALTEX4PEN_R {
        ALTEX4PEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ALTEX5 Pin Enable"]
    #[inline(always)]
    pub fn altex5pen(&self) -> ALTEX5PEN_R {
        ALTEX5PEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ALTEX6 Pin Enable"]
    #[inline(always)]
    pub fn altex6pen(&self) -> ALTEX6PEN_R {
        ALTEX6PEN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ALTEX7 Pin Enable"]
    #[inline(always)]
    pub fn altex7pen(&self) -> ALTEX7PEN_R {
        ALTEX7PEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&mut self) -> CH0PEN_W {
        CH0PEN_W { w: self }
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&mut self) -> CH1PEN_W {
        CH1PEN_W { w: self }
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&mut self) -> CH2PEN_W {
        CH2PEN_W { w: self }
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&mut self) -> CH3PEN_W {
        CH3PEN_W { w: self }
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&mut self) -> CH4PEN_W {
        CH4PEN_W { w: self }
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&mut self) -> CH5PEN_W {
        CH5PEN_W { w: self }
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&mut self) -> CH6PEN_W {
        CH6PEN_W { w: self }
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&mut self) -> CH7PEN_W {
        CH7PEN_W { w: self }
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&mut self) -> CH8PEN_W {
        CH8PEN_W { w: self }
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&mut self) -> CH9PEN_W {
        CH9PEN_W { w: self }
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&mut self) -> CH10PEN_W {
        CH10PEN_W { w: self }
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&mut self) -> CH11PEN_W {
        CH11PEN_W { w: self }
    }
    #[doc = "Bit 12 - CH12 Pin Enable"]
    #[inline(always)]
    pub fn ch12pen(&mut self) -> CH12PEN_W {
        CH12PEN_W { w: self }
    }
    #[doc = "Bit 13 - CH13 Pin Enable"]
    #[inline(always)]
    pub fn ch13pen(&mut self) -> CH13PEN_W {
        CH13PEN_W { w: self }
    }
    #[doc = "Bit 14 - CH14 Pin Enable"]
    #[inline(always)]
    pub fn ch14pen(&mut self) -> CH14PEN_W {
        CH14PEN_W { w: self }
    }
    #[doc = "Bit 15 - CH15 Pin Enable"]
    #[inline(always)]
    pub fn ch15pen(&mut self) -> CH15PEN_W {
        CH15PEN_W { w: self }
    }
    #[doc = "Bit 16 - ALTEX0 Pin Enable"]
    #[inline(always)]
    pub fn altex0pen(&mut self) -> ALTEX0PEN_W {
        ALTEX0PEN_W { w: self }
    }
    #[doc = "Bit 17 - ALTEX1 Pin Enable"]
    #[inline(always)]
    pub fn altex1pen(&mut self) -> ALTEX1PEN_W {
        ALTEX1PEN_W { w: self }
    }
    #[doc = "Bit 18 - ALTEX2 Pin Enable"]
    #[inline(always)]
    pub fn altex2pen(&mut self) -> ALTEX2PEN_W {
        ALTEX2PEN_W { w: self }
    }
    #[doc = "Bit 19 - ALTEX3 Pin Enable"]
    #[inline(always)]
    pub fn altex3pen(&mut self) -> ALTEX3PEN_W {
        ALTEX3PEN_W { w: self }
    }
    #[doc = "Bit 20 - ALTEX4 Pin Enable"]
    #[inline(always)]
    pub fn altex4pen(&mut self) -> ALTEX4PEN_W {
        ALTEX4PEN_W { w: self }
    }
    #[doc = "Bit 21 - ALTEX5 Pin Enable"]
    #[inline(always)]
    pub fn altex5pen(&mut self) -> ALTEX5PEN_W {
        ALTEX5PEN_W { w: self }
    }
    #[doc = "Bit 22 - ALTEX6 Pin Enable"]
    #[inline(always)]
    pub fn altex6pen(&mut self) -> ALTEX6PEN_W {
        ALTEX6PEN_W { w: self }
    }
    #[doc = "Bit 23 - ALTEX7 Pin Enable"]
    #[inline(always)]
    pub fn altex7pen(&mut self) -> ALTEX7PEN_W {
        ALTEX7PEN_W { w: self }
    }
}
