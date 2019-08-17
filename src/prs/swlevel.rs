#[doc = "Reader of register SWLEVEL"]
pub type R = crate::R<u32, super::SWLEVEL>;
#[doc = "Writer for register SWLEVEL"]
pub type W = crate::W<u32, super::SWLEVEL>;
#[doc = "Register SWLEVEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SWLEVEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0LEVEL`"]
pub type CH0LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0LEVEL`"]
pub struct CH0LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0LEVEL_W<'a> {
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
#[doc = "Reader of field `CH1LEVEL`"]
pub type CH1LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH1LEVEL`"]
pub struct CH1LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1LEVEL_W<'a> {
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
#[doc = "Reader of field `CH2LEVEL`"]
pub type CH2LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH2LEVEL`"]
pub struct CH2LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2LEVEL_W<'a> {
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
#[doc = "Reader of field `CH3LEVEL`"]
pub type CH3LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH3LEVEL`"]
pub struct CH3LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3LEVEL_W<'a> {
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
#[doc = "Reader of field `CH4LEVEL`"]
pub type CH4LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH4LEVEL`"]
pub struct CH4LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LEVEL_W<'a> {
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
#[doc = "Reader of field `CH5LEVEL`"]
pub type CH5LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH5LEVEL`"]
pub struct CH5LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LEVEL_W<'a> {
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
#[doc = "Reader of field `CH6LEVEL`"]
pub type CH6LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH6LEVEL`"]
pub struct CH6LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LEVEL_W<'a> {
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
#[doc = "Reader of field `CH7LEVEL`"]
pub type CH7LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH7LEVEL`"]
pub struct CH7LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LEVEL_W<'a> {
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
#[doc = "Reader of field `CH8LEVEL`"]
pub type CH8LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH8LEVEL`"]
pub struct CH8LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8LEVEL_W<'a> {
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
#[doc = "Reader of field `CH9LEVEL`"]
pub type CH9LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH9LEVEL`"]
pub struct CH9LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9LEVEL_W<'a> {
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
#[doc = "Reader of field `CH10LEVEL`"]
pub type CH10LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH10LEVEL`"]
pub struct CH10LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10LEVEL_W<'a> {
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
#[doc = "Reader of field `CH11LEVEL`"]
pub type CH11LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH11LEVEL`"]
pub struct CH11LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11LEVEL_W<'a> {
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
#[doc = "Reader of field `CH12LEVEL`"]
pub type CH12LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH12LEVEL`"]
pub struct CH12LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12LEVEL_W<'a> {
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
#[doc = "Reader of field `CH13LEVEL`"]
pub type CH13LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH13LEVEL`"]
pub struct CH13LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13LEVEL_W<'a> {
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
#[doc = "Reader of field `CH14LEVEL`"]
pub type CH14LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH14LEVEL`"]
pub struct CH14LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14LEVEL_W<'a> {
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
#[doc = "Reader of field `CH15LEVEL`"]
pub type CH15LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH15LEVEL`"]
pub struct CH15LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15LEVEL_W<'a> {
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
#[doc = "Reader of field `CH16LEVEL`"]
pub type CH16LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH16LEVEL`"]
pub struct CH16LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH16LEVEL_W<'a> {
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
#[doc = "Reader of field `CH17LEVEL`"]
pub type CH17LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH17LEVEL`"]
pub struct CH17LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17LEVEL_W<'a> {
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
#[doc = "Reader of field `CH18LEVEL`"]
pub type CH18LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH18LEVEL`"]
pub struct CH18LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18LEVEL_W<'a> {
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
#[doc = "Reader of field `CH19LEVEL`"]
pub type CH19LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH19LEVEL`"]
pub struct CH19LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19LEVEL_W<'a> {
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
#[doc = "Reader of field `CH20LEVEL`"]
pub type CH20LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH20LEVEL`"]
pub struct CH20LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20LEVEL_W<'a> {
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
#[doc = "Reader of field `CH21LEVEL`"]
pub type CH21LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH21LEVEL`"]
pub struct CH21LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21LEVEL_W<'a> {
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
#[doc = "Reader of field `CH22LEVEL`"]
pub type CH22LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH22LEVEL`"]
pub struct CH22LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22LEVEL_W<'a> {
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
#[doc = "Reader of field `CH23LEVEL`"]
pub type CH23LEVEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH23LEVEL`"]
pub struct CH23LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23LEVEL_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&self) -> CH6LEVEL_R {
        CH6LEVEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&self) -> CH7LEVEL_R {
        CH7LEVEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&self) -> CH8LEVEL_R {
        CH8LEVEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&self) -> CH9LEVEL_R {
        CH9LEVEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&self) -> CH10LEVEL_R {
        CH10LEVEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&self) -> CH11LEVEL_R {
        CH11LEVEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 Software Level"]
    #[inline(always)]
    pub fn ch12level(&self) -> CH12LEVEL_R {
        CH12LEVEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 Software Level"]
    #[inline(always)]
    pub fn ch13level(&self) -> CH13LEVEL_R {
        CH13LEVEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 Software Level"]
    #[inline(always)]
    pub fn ch14level(&self) -> CH14LEVEL_R {
        CH14LEVEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 Software Level"]
    #[inline(always)]
    pub fn ch15level(&self) -> CH15LEVEL_R {
        CH15LEVEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 Software Level"]
    #[inline(always)]
    pub fn ch16level(&self) -> CH16LEVEL_R {
        CH16LEVEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 Software Level"]
    #[inline(always)]
    pub fn ch17level(&self) -> CH17LEVEL_R {
        CH17LEVEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 Software Level"]
    #[inline(always)]
    pub fn ch18level(&self) -> CH18LEVEL_R {
        CH18LEVEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 Software Level"]
    #[inline(always)]
    pub fn ch19level(&self) -> CH19LEVEL_R {
        CH19LEVEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 Software Level"]
    #[inline(always)]
    pub fn ch20level(&self) -> CH20LEVEL_R {
        CH20LEVEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 Software Level"]
    #[inline(always)]
    pub fn ch21level(&self) -> CH21LEVEL_R {
        CH21LEVEL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 Software Level"]
    #[inline(always)]
    pub fn ch22level(&self) -> CH22LEVEL_R {
        CH22LEVEL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 Software Level"]
    #[inline(always)]
    pub fn ch23level(&self) -> CH23LEVEL_R {
        CH23LEVEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&mut self) -> CH0LEVEL_W {
        CH0LEVEL_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&mut self) -> CH1LEVEL_W {
        CH1LEVEL_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&mut self) -> CH2LEVEL_W {
        CH2LEVEL_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&mut self) -> CH3LEVEL_W {
        CH3LEVEL_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&mut self) -> CH4LEVEL_W {
        CH4LEVEL_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&mut self) -> CH5LEVEL_W {
        CH5LEVEL_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Software Level"]
    #[inline(always)]
    pub fn ch6level(&mut self) -> CH6LEVEL_W {
        CH6LEVEL_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Software Level"]
    #[inline(always)]
    pub fn ch7level(&mut self) -> CH7LEVEL_W {
        CH7LEVEL_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Software Level"]
    #[inline(always)]
    pub fn ch8level(&mut self) -> CH8LEVEL_W {
        CH8LEVEL_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Software Level"]
    #[inline(always)]
    pub fn ch9level(&mut self) -> CH9LEVEL_W {
        CH9LEVEL_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Software Level"]
    #[inline(always)]
    pub fn ch10level(&mut self) -> CH10LEVEL_W {
        CH10LEVEL_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Software Level"]
    #[inline(always)]
    pub fn ch11level(&mut self) -> CH11LEVEL_W {
        CH11LEVEL_W { w: self }
    }
    #[doc = "Bit 12 - Channel 12 Software Level"]
    #[inline(always)]
    pub fn ch12level(&mut self) -> CH12LEVEL_W {
        CH12LEVEL_W { w: self }
    }
    #[doc = "Bit 13 - Channel 13 Software Level"]
    #[inline(always)]
    pub fn ch13level(&mut self) -> CH13LEVEL_W {
        CH13LEVEL_W { w: self }
    }
    #[doc = "Bit 14 - Channel 14 Software Level"]
    #[inline(always)]
    pub fn ch14level(&mut self) -> CH14LEVEL_W {
        CH14LEVEL_W { w: self }
    }
    #[doc = "Bit 15 - Channel 15 Software Level"]
    #[inline(always)]
    pub fn ch15level(&mut self) -> CH15LEVEL_W {
        CH15LEVEL_W { w: self }
    }
    #[doc = "Bit 16 - Channel 16 Software Level"]
    #[inline(always)]
    pub fn ch16level(&mut self) -> CH16LEVEL_W {
        CH16LEVEL_W { w: self }
    }
    #[doc = "Bit 17 - Channel 17 Software Level"]
    #[inline(always)]
    pub fn ch17level(&mut self) -> CH17LEVEL_W {
        CH17LEVEL_W { w: self }
    }
    #[doc = "Bit 18 - Channel 18 Software Level"]
    #[inline(always)]
    pub fn ch18level(&mut self) -> CH18LEVEL_W {
        CH18LEVEL_W { w: self }
    }
    #[doc = "Bit 19 - Channel 19 Software Level"]
    #[inline(always)]
    pub fn ch19level(&mut self) -> CH19LEVEL_W {
        CH19LEVEL_W { w: self }
    }
    #[doc = "Bit 20 - Channel 20 Software Level"]
    #[inline(always)]
    pub fn ch20level(&mut self) -> CH20LEVEL_W {
        CH20LEVEL_W { w: self }
    }
    #[doc = "Bit 21 - Channel 21 Software Level"]
    #[inline(always)]
    pub fn ch21level(&mut self) -> CH21LEVEL_W {
        CH21LEVEL_W { w: self }
    }
    #[doc = "Bit 22 - Channel 22 Software Level"]
    #[inline(always)]
    pub fn ch22level(&mut self) -> CH22LEVEL_W {
        CH22LEVEL_W { w: self }
    }
    #[doc = "Bit 23 - Channel 23 Software Level"]
    #[inline(always)]
    pub fn ch23level(&mut self) -> CH23LEVEL_W {
        CH23LEVEL_W { w: self }
    }
}
