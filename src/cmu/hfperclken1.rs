#[doc = "Reader of register HFPERCLKEN1"]
pub type R = crate::R<u32, super::HFPERCLKEN1>;
#[doc = "Writer for register HFPERCLKEN1"]
pub type W = crate::W<u32, super::HFPERCLKEN1>;
#[doc = "Register HFPERCLKEN1 `reset()`'s with value 0"]
impl crate::ResetValue for super::HFPERCLKEN1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WTIMER0`"]
pub type WTIMER0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER0`"]
pub struct WTIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER0_W<'a> {
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
#[doc = "Reader of field `WTIMER1`"]
pub type WTIMER1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER1`"]
pub struct WTIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER1_W<'a> {
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
#[doc = "Reader of field `WTIMER2`"]
pub type WTIMER2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER2`"]
pub struct WTIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER2_W<'a> {
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
#[doc = "Reader of field `WTIMER3`"]
pub type WTIMER3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTIMER3`"]
pub struct WTIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> WTIMER3_W<'a> {
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
#[doc = "Reader of field `UART0`"]
pub type UART0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0`"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
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
#[doc = "Reader of field `UART1`"]
pub type UART1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1`"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
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
#[doc = "Reader of field `CAN0`"]
pub type CAN0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN0`"]
pub struct CAN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN0_W<'a> {
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
#[doc = "Reader of field `CAN1`"]
pub type CAN1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAN1`"]
pub struct CAN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1_W<'a> {
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
#[doc = "Reader of field `VDAC0`"]
pub type VDAC0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDAC0`"]
pub struct VDAC0_W<'a> {
    w: &'a mut W,
}
impl<'a> VDAC0_W<'a> {
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
#[doc = "Reader of field `CSEN`"]
pub type CSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSEN`"]
pub struct CSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn wtimer2(&self) -> WTIMER2_R {
        WTIMER2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn wtimer3(&self) -> WTIMER3_R {
        WTIMER3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CAN 1 Clock Enable"]
    #[inline(always)]
    pub fn can1(&self) -> CAN1_R {
        CAN1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> WTIMER0_W {
        WTIMER0_W { w: self }
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> WTIMER1_W {
        WTIMER1_W { w: self }
    }
    #[doc = "Bit 2 - Wide Timer 2 Clock Enable"]
    #[inline(always)]
    pub fn wtimer2(&mut self) -> WTIMER2_W {
        WTIMER2_W { w: self }
    }
    #[doc = "Bit 3 - Wide Timer 3 Clock Enable"]
    #[inline(always)]
    pub fn wtimer3(&mut self) -> WTIMER3_W {
        WTIMER3_W { w: self }
    }
    #[doc = "Bit 4 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 5 - Universal Asynchronous Receiver/Transmitter 1 Clock Enable"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 6 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W {
        CAN0_W { w: self }
    }
    #[doc = "Bit 7 - CAN 1 Clock Enable"]
    #[inline(always)]
    pub fn can1(&mut self) -> CAN1_W {
        CAN1_W { w: self }
    }
    #[doc = "Bit 8 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> VDAC0_W {
        VDAC0_W { w: self }
    }
    #[doc = "Bit 9 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&mut self) -> CSEN_W {
        CSEN_W { w: self }
    }
}
