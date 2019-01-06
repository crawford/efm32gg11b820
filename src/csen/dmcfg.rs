#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMCFG {
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
#[doc = r" Value of the field"]
pub struct DMGR {
    bits: u8,
}
impl DMGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMRR {
    bits: u8,
}
impl DMRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DMCRR {
    bits: u8,
}
impl DMCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `CRMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRMODER {
    #[doc = "10-bit delta modulator"]
    DM10,
    #[doc = "12-bit delta modulator"]
    DM12,
    #[doc = "14-bit delta modulator"]
    DM14,
    #[doc = "16-bit delta modulator"]
    DM16,
}
impl CRMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CRMODER::DM10 => 0,
            CRMODER::DM12 => 1,
            CRMODER::DM14 => 2,
            CRMODER::DM16 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CRMODER {
        match value {
            0 => CRMODER::DM10,
            1 => CRMODER::DM12,
            2 => CRMODER::DM14,
            3 => CRMODER::DM16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DM10`"]
    #[inline]
    pub fn is_dm10(&self) -> bool {
        *self == CRMODER::DM10
    }
    #[doc = "Checks if the value of the field is `DM12`"]
    #[inline]
    pub fn is_dm12(&self) -> bool {
        *self == CRMODER::DM12
    }
    #[doc = "Checks if the value of the field is `DM14`"]
    #[inline]
    pub fn is_dm14(&self) -> bool {
        *self == CRMODER::DM14
    }
    #[doc = "Checks if the value of the field is `DM16`"]
    #[inline]
    pub fn is_dm16(&self) -> bool {
        *self == CRMODER::DM16
    }
}
#[doc = r" Value of the field"]
pub struct DMGRDISR {
    bits: bool,
}
impl DMGRDISR {
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
#[doc = r" Proxy"]
pub struct _DMGW<'a> {
    w: &'a mut W,
}
impl<'a> _DMGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMRW<'a> {
    w: &'a mut W,
}
impl<'a> _DMRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMCRW<'a> {
    w: &'a mut W,
}
impl<'a> _DMCRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CRMODE`"]
pub enum CRMODEW {
    #[doc = "10-bit delta modulator"]
    DM10,
    #[doc = "12-bit delta modulator"]
    DM12,
    #[doc = "14-bit delta modulator"]
    DM14,
    #[doc = "16-bit delta modulator"]
    DM16,
}
impl CRMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CRMODEW::DM10 => 0,
            CRMODEW::DM12 => 1,
            CRMODEW::DM14 => 2,
            CRMODEW::DM16 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CRMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CRMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CRMODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "10-bit delta modulator"]
    #[inline]
    pub fn dm10(self) -> &'a mut W {
        self.variant(CRMODEW::DM10)
    }
    #[doc = "12-bit delta modulator"]
    #[inline]
    pub fn dm12(self) -> &'a mut W {
        self.variant(CRMODEW::DM12)
    }
    #[doc = "14-bit delta modulator"]
    #[inline]
    pub fn dm14(self) -> &'a mut W {
        self.variant(CRMODEW::DM14)
    }
    #[doc = "16-bit delta modulator"]
    #[inline]
    pub fn dm16(self) -> &'a mut W {
        self.variant(CRMODEW::DM16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMGRDISW<'a> {
    w: &'a mut W,
}
impl<'a> _DMGRDISW<'a> {
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
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline]
    pub fn dmg(&self) -> DMGR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMGR { bits }
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline]
    pub fn dmr(&self) -> DMRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMRR { bits }
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline]
    pub fn dmcr(&self) -> DMCRR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DMCRR { bits }
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline]
    pub fn crmode(&self) -> CRMODER {
        CRMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline]
    pub fn dmgrdis(&self) -> DMGRDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMGRDISR { bits }
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
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline]
    pub fn dmg(&mut self) -> _DMGW {
        _DMGW { w: self }
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline]
    pub fn dmr(&mut self) -> _DMRW {
        _DMRW { w: self }
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline]
    pub fn dmcr(&mut self) -> _DMCRW {
        _DMCRW { w: self }
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline]
    pub fn crmode(&mut self) -> _CRMODEW {
        _CRMODEW { w: self }
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline]
    pub fn dmgrdis(&mut self) -> _DMGRDISW {
        _DMGRDISW { w: self }
    }
}
