#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCCTRL {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = "Possible values of the field `ADC0CLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0CLKDIVR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC0CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0CLKDIVR::NODIVISION => 0,
            ADC0CLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0CLKDIVR {
        match value {
            0 => ADC0CLKDIVR::NODIVISION,
            i => ADC0CLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC0CLKDIVR::NODIVISION
    }
}
#[doc = "Possible values of the field `ADC0CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC0CLKSELR {
    #[doc = "ADC0 is not clocked"]
    DISABLED,
    #[doc = "AUXHFRCO is clocking ADC0"]
    AUXHFRCO,
    #[doc = "HFXO is clocking ADC0"]
    HFXO,
    #[doc = "HFSRCCLK is clocking ADC0"]
    HFSRCCLK,
}
impl ADC0CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC0CLKSELR::DISABLED => 0,
            ADC0CLKSELR::AUXHFRCO => 1,
            ADC0CLKSELR::HFXO => 2,
            ADC0CLKSELR::HFSRCCLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC0CLKSELR {
        match value {
            0 => ADC0CLKSELR::DISABLED,
            1 => ADC0CLKSELR::AUXHFRCO,
            2 => ADC0CLKSELR::HFXO,
            3 => ADC0CLKSELR::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSELR::DISABLED
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSELR::HFSRCCLK
    }
}
#[doc = r" Value of the field"]
pub struct ADC0CLKINVR {
    bits: bool,
}
impl ADC0CLKINVR {
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
#[doc = "Possible values of the field `ADC1CLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1CLKDIVR {
    #[doc = "\"\""]
    NODIVISION,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ADC1CLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC1CLKDIVR::NODIVISION => 0,
            ADC1CLKDIVR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC1CLKDIVR {
        match value {
            0 => ADC1CLKDIVR::NODIVISION,
            i => ADC1CLKDIVR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC1CLKDIVR::NODIVISION
    }
}
#[doc = "Possible values of the field `ADC1CLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC1CLKSELR {
    #[doc = "ADC1 is not clocked"]
    DISABLED,
    #[doc = "AUXHFRCO is clocking ADC1"]
    AUXHFRCO,
    #[doc = "HFXO is clocking ADC1"]
    HFXO,
    #[doc = "HFSRCCLK is clocking ADC1"]
    HFSRCCLK,
}
impl ADC1CLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC1CLKSELR::DISABLED => 0,
            ADC1CLKSELR::AUXHFRCO => 1,
            ADC1CLKSELR::HFXO => 2,
            ADC1CLKSELR::HFSRCCLK => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADC1CLKSELR {
        match value {
            0 => ADC1CLKSELR::DISABLED,
            1 => ADC1CLKSELR::AUXHFRCO,
            2 => ADC1CLKSELR::HFXO,
            3 => ADC1CLKSELR::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == ADC1CLKSELR::DISABLED
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC1CLKSELR::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC1CLKSELR::HFXO
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC1CLKSELR::HFSRCCLK
    }
}
#[doc = r" Value of the field"]
pub struct ADC1CLKINVR {
    bits: bool,
}
impl ADC1CLKINVR {
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
#[doc = "Values that can be written to the field `ADC0CLKDIV`"]
pub enum ADC0CLKDIVW {
    #[doc = "\"\""]
    NODIVISION,
}
impl ADC0CLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC0CLKDIVW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0CLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0CLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(ADC0CLKDIVW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC0CLKSEL`"]
pub enum ADC0CLKSELW {
    #[doc = "ADC0 is not clocked"]
    DISABLED,
    #[doc = "AUXHFRCO is clocking ADC0"]
    AUXHFRCO,
    #[doc = "HFXO is clocking ADC0"]
    HFXO,
    #[doc = "HFSRCCLK is clocking ADC0"]
    HFSRCCLK,
}
impl ADC0CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC0CLKSELW::DISABLED => 0,
            ADC0CLKSELW::AUXHFRCO => 1,
            ADC0CLKSELW::HFXO => 2,
            ADC0CLKSELW::HFSRCCLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC0CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC0CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC0 is not clocked"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(ADC0CLKSELW::HFSRCCLK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ADC0CLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC0CLKINVW<'a> {
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1CLKDIV`"]
pub enum ADC1CLKDIVW {
    #[doc = "\"\""]
    NODIVISION,
}
impl ADC1CLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC1CLKDIVW::NODIVISION => 0,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1CLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1CLKDIVW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "\"\""]
    #[inline]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(ADC1CLKDIVW::NODIVISION)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADC1CLKSEL`"]
pub enum ADC1CLKSELW {
    #[doc = "ADC1 is not clocked"]
    DISABLED,
    #[doc = "AUXHFRCO is clocking ADC1"]
    AUXHFRCO,
    #[doc = "HFXO is clocking ADC1"]
    HFXO,
    #[doc = "HFSRCCLK is clocking ADC1"]
    HFSRCCLK,
}
impl ADC1CLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC1CLKSELW::DISABLED => 0,
            ADC1CLKSELW::AUXHFRCO => 1,
            ADC1CLKSELW::HFXO => 2,
            ADC1CLKSELW::HFSRCCLK => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADC1CLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1CLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADC1CLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC1 is not clocked"]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC1CLKSELW::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC1"]
    #[inline]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(ADC1CLKSELW::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC1"]
    #[inline]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(ADC1CLKSELW::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC1"]
    #[inline]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(ADC1CLKSELW::HFSRCCLK)
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
pub struct _ADC1CLKINVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC1CLKINVW<'a> {
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
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline]
    pub fn adc0clkdiv(&self) -> ADC0CLKDIVR {
        ADC0CLKDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline]
    pub fn adc0clksel(&self) -> ADC0CLKSELR {
        ADC0CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline]
    pub fn adc0clkinv(&self) -> ADC0CLKINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC0CLKINVR { bits }
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline]
    pub fn adc1clkdiv(&self) -> ADC1CLKDIVR {
        ADC1CLKDIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline]
    pub fn adc1clksel(&self) -> ADC1CLKSELR {
        ADC1CLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline]
    pub fn adc1clkinv(&self) -> ADC1CLKINVR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ADC1CLKINVR { bits }
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
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline]
    pub fn adc0clkdiv(&mut self) -> _ADC0CLKDIVW {
        _ADC0CLKDIVW { w: self }
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline]
    pub fn adc0clksel(&mut self) -> _ADC0CLKSELW {
        _ADC0CLKSELW { w: self }
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline]
    pub fn adc0clkinv(&mut self) -> _ADC0CLKINVW {
        _ADC0CLKINVW { w: self }
    }
    #[doc = "Bits 16:17 - ADC1 Clock Prescaler"]
    #[inline]
    pub fn adc1clkdiv(&mut self) -> _ADC1CLKDIVW {
        _ADC1CLKDIVW { w: self }
    }
    #[doc = "Bits 20:21 - ADC1 Clock Select"]
    #[inline]
    pub fn adc1clksel(&mut self) -> _ADC1CLKSELW {
        _ADC1CLKSELW { w: self }
    }
    #[doc = "Bit 24 - Invert Clock Selected By ADC1CLKSEL"]
    #[inline]
    pub fn adc1clkinv(&mut self) -> _ADC1CLKINVW {
        _ADC1CLKINVW { w: self }
    }
}
