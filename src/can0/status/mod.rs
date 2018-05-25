#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LECR {
    #[doc = " No error occurred during last CAN bus event."]
    NONE,
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF,
    #[doc = "A fixed format part of a received frame has the wrong format."]
    FORM,
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    ACK,
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    BIT1,
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    BIT0,
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    CRC,
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    UNUSED,
}
impl LECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LECR::NONE => 0,
            LECR::STUFF => 1,
            LECR::FORM => 2,
            LECR::ACK => 3,
            LECR::BIT1 => 4,
            LECR::BIT0 => 5,
            LECR::CRC => 6,
            LECR::UNUSED => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LECR {
        match value {
            0 => LECR::NONE,
            1 => LECR::STUFF,
            2 => LECR::FORM,
            3 => LECR::ACK,
            4 => LECR::BIT1,
            5 => LECR::BIT0,
            6 => LECR::CRC,
            7 => LECR::UNUSED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == LECR::NONE
    }
    #[doc = "Checks if the value of the field is `STUFF`"]
    #[inline]
    pub fn is_stuff(&self) -> bool {
        *self == LECR::STUFF
    }
    #[doc = "Checks if the value of the field is `FORM`"]
    #[inline]
    pub fn is_form(&self) -> bool {
        *self == LECR::FORM
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline]
    pub fn is_ack(&self) -> bool {
        *self == LECR::ACK
    }
    #[doc = "Checks if the value of the field is `BIT1`"]
    #[inline]
    pub fn is_bit1(&self) -> bool {
        *self == LECR::BIT1
    }
    #[doc = "Checks if the value of the field is `BIT0`"]
    #[inline]
    pub fn is_bit0(&self) -> bool {
        *self == LECR::BIT0
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline]
    pub fn is_crc(&self) -> bool {
        *self == LECR::CRC
    }
    #[doc = "Checks if the value of the field is `UNUSED`"]
    #[inline]
    pub fn is_unused(&self) -> bool {
        *self == LECR::UNUSED
    }
}
#[doc = r" Value of the field"]
pub struct TXOKR {
    bits: bool,
}
impl TXOKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RXOKR {
    bits: bool,
}
impl RXOKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct EPASSR {
    bits: bool,
}
impl EPASSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct EWARNR {
    bits: bool,
}
impl EWARNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct BOFFR {
    bits: bool,
}
impl BOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Values that can be written to the field `LEC`"]
pub enum LECW {
    #[doc = " No error occurred during last CAN bus event."]
    NONE,
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    STUFF,
    #[doc = "A fixed format part of a received frame has the wrong format."]
    FORM,
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    ACK,
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    BIT1,
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    BIT0,
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    CRC,
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    UNUSED,
}
impl LECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LECW::NONE => 0,
            LECW::STUFF => 1,
            LECW::FORM => 2,
            LECW::ACK => 3,
            LECW::BIT1 => 4,
            LECW::BIT0 => 5,
            LECW::CRC => 6,
            LECW::UNUSED => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LECW<'a> {
    w: &'a mut W,
}
impl<'a> _LECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No error occurred during last CAN bus event."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(LECW::NONE)
    }
    #[doc = "More than 5 equal bits in a sequence have occurred in a part of a received message where this is not allowed."]
    #[inline]
    pub fn stuff(self) -> &'a mut W {
        self.variant(LECW::STUFF)
    }
    #[doc = "A fixed format part of a received frame has the wrong format."]
    #[inline]
    pub fn form(self) -> &'a mut W {
        self.variant(LECW::FORM)
    }
    #[doc = "The message this CAN Core transmitted was not acknowledged by another node."]
    #[inline]
    pub fn ack(self) -> &'a mut W {
        self.variant(LECW::ACK)
    }
    #[doc = "During the transmission of a message (with the exception of the arbitration field), the device wanted to send a recessive level (bit of logical value 1), but the monitored bus value was dominant."]
    #[inline]
    pub fn bit1(self) -> &'a mut W {
        self.variant(LECW::BIT1)
    }
    #[doc = "During the transmission of a message (or acknowledge bit, or active error flag, or overload flag), the device wanted to send a dominant level (data or identifier bit logical value 0), but the monitored Bus value was recessive. During Bus Off recovery this status is set each time a sequence of 11 recessive bits has been monitored. This enables the CPU to monitor the proceeding of the Bus Off recovery sequence (indicating the bus is not stuck at dominant or continuously disturbed)."]
    #[inline]
    pub fn bit0(self) -> &'a mut W {
        self.variant(LECW::BIT0)
    }
    #[doc = "The CRC check sum was incorrect in the message received; the CRC received for an incoming message does not match with the calculated CRC for the received data."]
    #[inline]
    pub fn crc(self) -> &'a mut W {
        self.variant(LECW::CRC)
    }
    #[doc = "When the LEC shows the value '7', no CAN bus event was detected since the CPU wrote this value to the LEC."]
    #[inline]
    pub fn unused(self) -> &'a mut W {
        self.variant(LECW::UNUSED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXOKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline]
    pub fn lec(&self) -> LECR {
        LECR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline]
    pub fn txok(&self) -> TXOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXOKR { bits }
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline]
    pub fn rxok(&self) -> RXOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXOKR { bits }
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline]
    pub fn epass(&self) -> EPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EPASSR { bits }
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline]
    pub fn ewarn(&self) -> EWARNR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EWARNR { bits }
    }
    #[doc = "Bit 7 - Bus Off Status"]
    #[inline]
    pub fn boff(&self) -> BOFFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BOFFR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline]
    pub fn lec(&mut self) -> _LECW {
        _LECW { w: self }
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline]
    pub fn txok(&mut self) -> _TXOKW {
        _TXOKW { w: self }
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline]
    pub fn rxok(&mut self) -> _RXOKW {
        _RXOKW { w: self }
    }
}
