#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPA2_MUX {
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
pub struct POSSELR {
    bits: u8,
}
impl POSSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NEGSELR {
    bits: u8,
}
impl NEGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `RESINMUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESINMUXR {
    #[doc = "Set for Unity Gain"]
    DISABLE,
    #[doc = "Set for NEXTOUT(x-1) input"]
    OPANEXT,
    #[doc = "NEG pad connected"]
    NEGPAD,
    #[doc = "POS pad connected"]
    POSPAD,
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    COMPAD,
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    CENTER,
    #[doc = "VSS connected"]
    VSS,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RESINMUXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESINMUXR::DISABLE => 0,
            RESINMUXR::OPANEXT => 1,
            RESINMUXR::NEGPAD => 2,
            RESINMUXR::POSPAD => 3,
            RESINMUXR::COMPAD => 4,
            RESINMUXR::CENTER => 5,
            RESINMUXR::VSS => 6,
            RESINMUXR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESINMUXR {
        match value {
            0 => RESINMUXR::DISABLE,
            1 => RESINMUXR::OPANEXT,
            2 => RESINMUXR::NEGPAD,
            3 => RESINMUXR::POSPAD,
            4 => RESINMUXR::COMPAD,
            5 => RESINMUXR::CENTER,
            6 => RESINMUXR::VSS,
            i => RESINMUXR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline]
    pub fn is_disable(&self) -> bool {
        *self == RESINMUXR::DISABLE
    }
    #[doc = "Checks if the value of the field is `OPANEXT`"]
    #[inline]
    pub fn is_opanext(&self) -> bool {
        *self == RESINMUXR::OPANEXT
    }
    #[doc = "Checks if the value of the field is `NEGPAD`"]
    #[inline]
    pub fn is_negpad(&self) -> bool {
        *self == RESINMUXR::NEGPAD
    }
    #[doc = "Checks if the value of the field is `POSPAD`"]
    #[inline]
    pub fn is_pospad(&self) -> bool {
        *self == RESINMUXR::POSPAD
    }
    #[doc = "Checks if the value of the field is `COMPAD`"]
    #[inline]
    pub fn is_compad(&self) -> bool {
        *self == RESINMUXR::COMPAD
    }
    #[doc = "Checks if the value of the field is `CENTER`"]
    #[inline]
    pub fn is_center(&self) -> bool {
        *self == RESINMUXR::CENTER
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline]
    pub fn is_vss(&self) -> bool {
        *self == RESINMUXR::VSS
    }
}
#[doc = r" Value of the field"]
pub struct GAIN3XR {
    bits: bool,
}
impl GAIN3XR {
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
#[doc = "Possible values of the field `RESSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESSELR {
    #[doc = "Gain of 1/3"]
    RES0,
    #[doc = "Gain of 1"]
    RES1,
    #[doc = "Gain of 1 2/3"]
    RES2,
    #[doc = "Gain of 2 1/5"]
    RES3,
    #[doc = "Gain of 3"]
    RES4,
    #[doc = "Gain of 4 1/3"]
    RES5,
    #[doc = "Gain of 7"]
    RES6,
    #[doc = "Gain of 15"]
    RES7,
}
impl RESSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESSELR::RES0 => 0,
            RESSELR::RES1 => 1,
            RESSELR::RES2 => 2,
            RESSELR::RES3 => 3,
            RESSELR::RES4 => 4,
            RESSELR::RES5 => 5,
            RESSELR::RES6 => 6,
            RESSELR::RES7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESSELR {
        match value {
            0 => RESSELR::RES0,
            1 => RESSELR::RES1,
            2 => RESSELR::RES2,
            3 => RESSELR::RES3,
            4 => RESSELR::RES4,
            5 => RESSELR::RES5,
            6 => RESSELR::RES6,
            7 => RESSELR::RES7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline]
    pub fn is_res0(&self) -> bool {
        *self == RESSELR::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline]
    pub fn is_res1(&self) -> bool {
        *self == RESSELR::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline]
    pub fn is_res2(&self) -> bool {
        *self == RESSELR::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline]
    pub fn is_res3(&self) -> bool {
        *self == RESSELR::RES3
    }
    #[doc = "Checks if the value of the field is `RES4`"]
    #[inline]
    pub fn is_res4(&self) -> bool {
        *self == RESSELR::RES4
    }
    #[doc = "Checks if the value of the field is `RES5`"]
    #[inline]
    pub fn is_res5(&self) -> bool {
        *self == RESSELR::RES5
    }
    #[doc = "Checks if the value of the field is `RES6`"]
    #[inline]
    pub fn is_res6(&self) -> bool {
        *self == RESSELR::RES6
    }
    #[doc = "Checks if the value of the field is `RES7`"]
    #[inline]
    pub fn is_res7(&self) -> bool {
        *self == RESSELR::RES7
    }
}
#[doc = r" Proxy"]
pub struct _POSSELW<'a> {
    w: &'a mut W,
}
impl<'a> _POSSELW<'a> {
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
pub struct _NEGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _NEGSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESINMUX`"]
pub enum RESINMUXW {
    #[doc = "Set for Unity Gain"]
    DISABLE,
    #[doc = "Set for NEXTOUT(x-1) input"]
    OPANEXT,
    #[doc = "NEG pad connected"]
    NEGPAD,
    #[doc = "POS pad connected"]
    POSPAD,
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    COMPAD,
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    CENTER,
    #[doc = "VSS connected"]
    VSS,
}
impl RESINMUXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESINMUXW::DISABLE => 0,
            RESINMUXW::OPANEXT => 1,
            RESINMUXW::NEGPAD => 2,
            RESINMUXW::POSPAD => 3,
            RESINMUXW::COMPAD => 4,
            RESINMUXW::CENTER => 5,
            RESINMUXW::VSS => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESINMUXW<'a> {
    w: &'a mut W,
}
impl<'a> _RESINMUXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESINMUXW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Set for Unity Gain"]
    #[inline]
    pub fn disable(self) -> &'a mut W {
        self.variant(RESINMUXW::DISABLE)
    }
    #[doc = "Set for NEXTOUT(x-1) input"]
    #[inline]
    pub fn opanext(self) -> &'a mut W {
        self.variant(RESINMUXW::OPANEXT)
    }
    #[doc = "NEG pad connected"]
    #[inline]
    pub fn negpad(self) -> &'a mut W {
        self.variant(RESINMUXW::NEGPAD)
    }
    #[doc = "POS pad connected"]
    #[inline]
    pub fn pospad(self) -> &'a mut W {
        self.variant(RESINMUXW::POSPAD)
    }
    #[doc = "Neg pad of OPA0 connected. Direct input to support common reference."]
    #[inline]
    pub fn compad(self) -> &'a mut W {
        self.variant(RESINMUXW::COMPAD)
    }
    #[doc = "OPA0 and OPA1 Resmux connected to form fully differential instrumentation amplifier."]
    #[inline]
    pub fn center(self) -> &'a mut W {
        self.variant(RESINMUXW::CENTER)
    }
    #[doc = "VSS connected"]
    #[inline]
    pub fn vss(self) -> &'a mut W {
        self.variant(RESINMUXW::VSS)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GAIN3XW<'a> {
    w: &'a mut W,
}
impl<'a> _GAIN3XW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESSEL`"]
pub enum RESSELW {
    #[doc = "Gain of 1/3"]
    RES0,
    #[doc = "Gain of 1"]
    RES1,
    #[doc = "Gain of 1 2/3"]
    RES2,
    #[doc = "Gain of 2 1/5"]
    RES3,
    #[doc = "Gain of 3"]
    RES4,
    #[doc = "Gain of 4 1/3"]
    RES5,
    #[doc = "Gain of 7"]
    RES6,
    #[doc = "Gain of 15"]
    RES7,
}
impl RESSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESSELW::RES0 => 0,
            RESSELW::RES1 => 1,
            RESSELW::RES2 => 2,
            RESSELW::RES3 => 3,
            RESSELW::RES4 => 4,
            RESSELW::RES5 => 5,
            RESSELW::RES6 => 6,
            RESSELW::RES7 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RESSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Gain of 1/3"]
    #[inline]
    pub fn res0(self) -> &'a mut W {
        self.variant(RESSELW::RES0)
    }
    #[doc = "Gain of 1"]
    #[inline]
    pub fn res1(self) -> &'a mut W {
        self.variant(RESSELW::RES1)
    }
    #[doc = "Gain of 1 2/3"]
    #[inline]
    pub fn res2(self) -> &'a mut W {
        self.variant(RESSELW::RES2)
    }
    #[doc = "Gain of 2 1/5"]
    #[inline]
    pub fn res3(self) -> &'a mut W {
        self.variant(RESSELW::RES3)
    }
    #[doc = "Gain of 3"]
    #[inline]
    pub fn res4(self) -> &'a mut W {
        self.variant(RESSELW::RES4)
    }
    #[doc = "Gain of 4 1/3"]
    #[inline]
    pub fn res5(self) -> &'a mut W {
        self.variant(RESSELW::RES5)
    }
    #[doc = "Gain of 7"]
    #[inline]
    pub fn res6(self) -> &'a mut W {
        self.variant(RESSELW::RES6)
    }
    #[doc = "Gain of 15"]
    #[inline]
    pub fn res7(self) -> &'a mut W {
        self.variant(RESSELW::RES7)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline]
    pub fn possel(&self) -> POSSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        POSSELR { bits }
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline]
    pub fn negsel(&self) -> NEGSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NEGSELR { bits }
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline]
    pub fn resinmux(&self) -> RESINMUXR {
        RESINMUXR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline]
    pub fn gain3x(&self) -> GAIN3XR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GAIN3XR { bits }
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline]
    pub fn ressel(&self) -> RESSELR {
        RESSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1503985 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - OPAx Non-inverting Input Mux"]
    #[inline]
    pub fn possel(&mut self) -> _POSSELW {
        _POSSELW { w: self }
    }
    #[doc = "Bits 8:15 - OPAx Inverting Input Mux"]
    #[inline]
    pub fn negsel(&mut self) -> _NEGSELW {
        _NEGSELW { w: self }
    }
    #[doc = "Bits 16:18 - OPAx Resistor Ladder Input Mux"]
    #[inline]
    pub fn resinmux(&mut self) -> _RESINMUXW {
        _RESINMUXW { w: self }
    }
    #[doc = "Bit 20 - OPAx Dedicated 3x Gain Resistor Ladder"]
    #[inline]
    pub fn gain3x(&mut self) -> _GAIN3XW {
        _GAIN3XW { w: self }
    }
    #[doc = "Bits 24:26 - OPAx Resistor Ladder Select"]
    #[inline]
    pub fn ressel(&mut self) -> _RESSELW {
        _RESSELW { w: self }
    }
}
