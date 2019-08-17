#[doc = "Writer for register SWPULSE"]
pub type W = crate::W<u32, super::SWPULSE>;
#[doc = "Register SWPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::SWPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `CH0PULSE`"]
pub struct CH0PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH1PULSE`"]
pub struct CH1PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH2PULSE`"]
pub struct CH2PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH3PULSE`"]
pub struct CH3PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH4PULSE`"]
pub struct CH4PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH5PULSE`"]
pub struct CH5PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH6PULSE`"]
pub struct CH6PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH7PULSE`"]
pub struct CH7PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH8PULSE`"]
pub struct CH8PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH9PULSE`"]
pub struct CH9PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH10PULSE`"]
pub struct CH10PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH11PULSE`"]
pub struct CH11PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH12PULSE`"]
pub struct CH12PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH13PULSE`"]
pub struct CH13PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH14PULSE`"]
pub struct CH14PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH15PULSE`"]
pub struct CH15PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH16PULSE`"]
pub struct CH16PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH16PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH17PULSE`"]
pub struct CH17PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH18PULSE`"]
pub struct CH18PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH19PULSE`"]
pub struct CH19PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH20PULSE`"]
pub struct CH20PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH21PULSE`"]
pub struct CH21PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH22PULSE`"]
pub struct CH22PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22PULSE_W<'a> {
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
#[doc = "Write proxy for field `CH23PULSE`"]
pub struct CH23PULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23PULSE_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Pulse Generation"]
    #[inline(always)]
    pub fn ch0pulse(&mut self) -> CH0PULSE_W {
        CH0PULSE_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Pulse Generation"]
    #[inline(always)]
    pub fn ch1pulse(&mut self) -> CH1PULSE_W {
        CH1PULSE_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Pulse Generation"]
    #[inline(always)]
    pub fn ch2pulse(&mut self) -> CH2PULSE_W {
        CH2PULSE_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Pulse Generation"]
    #[inline(always)]
    pub fn ch3pulse(&mut self) -> CH3PULSE_W {
        CH3PULSE_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Pulse Generation"]
    #[inline(always)]
    pub fn ch4pulse(&mut self) -> CH4PULSE_W {
        CH4PULSE_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Pulse Generation"]
    #[inline(always)]
    pub fn ch5pulse(&mut self) -> CH5PULSE_W {
        CH5PULSE_W { w: self }
    }
    #[doc = "Bit 6 - Channel 6 Pulse Generation"]
    #[inline(always)]
    pub fn ch6pulse(&mut self) -> CH6PULSE_W {
        CH6PULSE_W { w: self }
    }
    #[doc = "Bit 7 - Channel 7 Pulse Generation"]
    #[inline(always)]
    pub fn ch7pulse(&mut self) -> CH7PULSE_W {
        CH7PULSE_W { w: self }
    }
    #[doc = "Bit 8 - Channel 8 Pulse Generation"]
    #[inline(always)]
    pub fn ch8pulse(&mut self) -> CH8PULSE_W {
        CH8PULSE_W { w: self }
    }
    #[doc = "Bit 9 - Channel 9 Pulse Generation"]
    #[inline(always)]
    pub fn ch9pulse(&mut self) -> CH9PULSE_W {
        CH9PULSE_W { w: self }
    }
    #[doc = "Bit 10 - Channel 10 Pulse Generation"]
    #[inline(always)]
    pub fn ch10pulse(&mut self) -> CH10PULSE_W {
        CH10PULSE_W { w: self }
    }
    #[doc = "Bit 11 - Channel 11 Pulse Generation"]
    #[inline(always)]
    pub fn ch11pulse(&mut self) -> CH11PULSE_W {
        CH11PULSE_W { w: self }
    }
    #[doc = "Bit 12 - Channel 12 Pulse Generation"]
    #[inline(always)]
    pub fn ch12pulse(&mut self) -> CH12PULSE_W {
        CH12PULSE_W { w: self }
    }
    #[doc = "Bit 13 - Channel 13 Pulse Generation"]
    #[inline(always)]
    pub fn ch13pulse(&mut self) -> CH13PULSE_W {
        CH13PULSE_W { w: self }
    }
    #[doc = "Bit 14 - Channel 14 Pulse Generation"]
    #[inline(always)]
    pub fn ch14pulse(&mut self) -> CH14PULSE_W {
        CH14PULSE_W { w: self }
    }
    #[doc = "Bit 15 - Channel 15 Pulse Generation"]
    #[inline(always)]
    pub fn ch15pulse(&mut self) -> CH15PULSE_W {
        CH15PULSE_W { w: self }
    }
    #[doc = "Bit 16 - Channel 16 Pulse Generation"]
    #[inline(always)]
    pub fn ch16pulse(&mut self) -> CH16PULSE_W {
        CH16PULSE_W { w: self }
    }
    #[doc = "Bit 17 - Channel 17 Pulse Generation"]
    #[inline(always)]
    pub fn ch17pulse(&mut self) -> CH17PULSE_W {
        CH17PULSE_W { w: self }
    }
    #[doc = "Bit 18 - Channel 18 Pulse Generation"]
    #[inline(always)]
    pub fn ch18pulse(&mut self) -> CH18PULSE_W {
        CH18PULSE_W { w: self }
    }
    #[doc = "Bit 19 - Channel 19 Pulse Generation"]
    #[inline(always)]
    pub fn ch19pulse(&mut self) -> CH19PULSE_W {
        CH19PULSE_W { w: self }
    }
    #[doc = "Bit 20 - Channel 20 Pulse Generation"]
    #[inline(always)]
    pub fn ch20pulse(&mut self) -> CH20PULSE_W {
        CH20PULSE_W { w: self }
    }
    #[doc = "Bit 21 - Channel 21 Pulse Generation"]
    #[inline(always)]
    pub fn ch21pulse(&mut self) -> CH21PULSE_W {
        CH21PULSE_W { w: self }
    }
    #[doc = "Bit 22 - Channel 22 Pulse Generation"]
    #[inline(always)]
    pub fn ch22pulse(&mut self) -> CH22PULSE_W {
        CH22PULSE_W { w: self }
    }
    #[doc = "Bit 23 - Channel 23 Pulse Generation"]
    #[inline(always)]
    pub fn ch23pulse(&mut self) -> CH23PULSE_W {
        CH23PULSE_W { w: self }
    }
}
