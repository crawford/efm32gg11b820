#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMACFG {
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
#[doc = r" Value of the field"]
pub struct AMBABRSTLENR {
    bits: u8,
}
impl AMBABRSTLENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct HDRDATASPLITENR {
    bits: bool,
}
impl HDRDATASPLITENR {
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
#[doc = "Possible values of the field `RXPBUFSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXPBUFSIZER {
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    SIZE0,
    #[doc = "Do not use top two address bits (1 Kb)"]
    SIZE1,
    #[doc = "Do not use top address bit (2 Kb)"]
    SIZE2,
    #[doc = "Use full configured addressable space (4 Kb)"]
    SIZE3,
}
impl RXPBUFSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXPBUFSIZER::SIZE0 => 0,
            RXPBUFSIZER::SIZE1 => 1,
            RXPBUFSIZER::SIZE2 => 2,
            RXPBUFSIZER::SIZE3 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXPBUFSIZER {
        match value {
            0 => RXPBUFSIZER::SIZE0,
            1 => RXPBUFSIZER::SIZE1,
            2 => RXPBUFSIZER::SIZE2,
            3 => RXPBUFSIZER::SIZE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE0`"]
    #[inline]
    pub fn is_size0(&self) -> bool {
        *self == RXPBUFSIZER::SIZE0
    }
    #[doc = "Checks if the value of the field is `SIZE1`"]
    #[inline]
    pub fn is_size1(&self) -> bool {
        *self == RXPBUFSIZER::SIZE1
    }
    #[doc = "Checks if the value of the field is `SIZE2`"]
    #[inline]
    pub fn is_size2(&self) -> bool {
        *self == RXPBUFSIZER::SIZE2
    }
    #[doc = "Checks if the value of the field is `SIZE3`"]
    #[inline]
    pub fn is_size3(&self) -> bool {
        *self == RXPBUFSIZER::SIZE3
    }
}
#[doc = r" Value of the field"]
pub struct TXPBUFSIZER {
    bits: bool,
}
impl TXPBUFSIZER {
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
pub struct TXPBUFTCPENR {
    bits: bool,
}
impl TXPBUFTCPENR {
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
pub struct INFLASTDBUFSIZEENR {
    bits: bool,
}
impl INFLASTDBUFSIZEENR {
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
pub struct RXBUFSIZER {
    bits: u8,
}
impl RXBUFSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FRCDISCARDONERRR {
    bits: bool,
}
impl FRCDISCARDONERRR {
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
pub struct FRCMAXAMBABRSTRXR {
    bits: bool,
}
impl FRCMAXAMBABRSTRXR {
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
pub struct FRCMAXAMBABRSTTXR {
    bits: bool,
}
impl FRCMAXAMBABRSTTXR {
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
pub struct RXBDEXTNDMODEENR {
    bits: bool,
}
impl RXBDEXTNDMODEENR {
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
pub struct TXBDEXTENDMODEENR {
    bits: bool,
}
impl TXBDEXTENDMODEENR {
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
pub struct _AMBABRSTLENW<'a> {
    w: &'a mut W,
}
impl<'a> _AMBABRSTLENW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _HDRDATASPLITENW<'a> {
    w: &'a mut W,
}
impl<'a> _HDRDATASPLITENW<'a> {
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXPBUFSIZE`"]
pub enum RXPBUFSIZEW {
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    SIZE0,
    #[doc = "Do not use top two address bits (1 Kb)"]
    SIZE1,
    #[doc = "Do not use top address bit (2 Kb)"]
    SIZE2,
    #[doc = "Use full configured addressable space (4 Kb)"]
    SIZE3,
}
impl RXPBUFSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXPBUFSIZEW::SIZE0 => 0,
            RXPBUFSIZEW::SIZE1 => 1,
            RXPBUFSIZEW::SIZE2 => 2,
            RXPBUFSIZEW::SIZE3 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXPBUFSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXPBUFSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXPBUFSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline]
    pub fn size0(self) -> &'a mut W {
        self.variant(RXPBUFSIZEW::SIZE0)
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline]
    pub fn size1(self) -> &'a mut W {
        self.variant(RXPBUFSIZEW::SIZE1)
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline]
    pub fn size2(self) -> &'a mut W {
        self.variant(RXPBUFSIZEW::SIZE2)
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline]
    pub fn size3(self) -> &'a mut W {
        self.variant(RXPBUFSIZEW::SIZE3)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXPBUFSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPBUFSIZEW<'a> {
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TXPBUFTCPENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXPBUFTCPENW<'a> {
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _INFLASTDBUFSIZEENW<'a> {
    w: &'a mut W,
}
impl<'a> _INFLASTDBUFSIZEENW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXBUFSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBUFSIZEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRCDISCARDONERRW<'a> {
    w: &'a mut W,
}
impl<'a> _FRCDISCARDONERRW<'a> {
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
#[doc = r" Proxy"]
pub struct _FRCMAXAMBABRSTRXW<'a> {
    w: &'a mut W,
}
impl<'a> _FRCMAXAMBABRSTRXW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FRCMAXAMBABRSTTXW<'a> {
    w: &'a mut W,
}
impl<'a> _FRCMAXAMBABRSTTXW<'a> {
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXBDEXTNDMODEENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBDEXTNDMODEENW<'a> {
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
#[doc = r" Proxy"]
pub struct _TXBDEXTENDMODEENW<'a> {
    w: &'a mut W,
}
impl<'a> _TXBDEXTENDMODEENW<'a> {
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
        const OFFSET: u8 = 29;
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
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline]
    pub fn ambabrstlen(&self) -> AMBABRSTLENR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        AMBABRSTLENR { bits }
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline]
    pub fn hdrdataspliten(&self) -> HDRDATASPLITENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        HDRDATASPLITENR { bits }
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline]
    pub fn rxpbufsize(&self) -> RXPBUFSIZER {
        RXPBUFSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline]
    pub fn txpbufsize(&self) -> TXPBUFSIZER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXPBUFSIZER { bits }
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline]
    pub fn txpbuftcpen(&self) -> TXPBUFTCPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXPBUFTCPENR { bits }
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline]
    pub fn inflastdbufsizeen(&self) -> INFLASTDBUFSIZEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        INFLASTDBUFSIZEENR { bits }
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline]
    pub fn rxbufsize(&self) -> RXBUFSIZER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXBUFSIZER { bits }
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline]
    pub fn frcdiscardonerr(&self) -> FRCDISCARDONERRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRCDISCARDONERRR { bits }
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline]
    pub fn frcmaxambabrstrx(&self) -> FRCMAXAMBABRSTRXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRCMAXAMBABRSTRXR { bits }
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline]
    pub fn frcmaxambabrsttx(&self) -> FRCMAXAMBABRSTTXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FRCMAXAMBABRSTTXR { bits }
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline]
    pub fn rxbdextndmodeen(&self) -> RXBDEXTNDMODEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXBDEXTNDMODEENR { bits }
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline]
    pub fn txbdextendmodeen(&self) -> TXBDEXTENDMODEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TXBDEXTENDMODEENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 132868 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline]
    pub fn ambabrstlen(&mut self) -> _AMBABRSTLENW {
        _AMBABRSTLENW { w: self }
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline]
    pub fn hdrdataspliten(&mut self) -> _HDRDATASPLITENW {
        _HDRDATASPLITENW { w: self }
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline]
    pub fn rxpbufsize(&mut self) -> _RXPBUFSIZEW {
        _RXPBUFSIZEW { w: self }
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline]
    pub fn txpbufsize(&mut self) -> _TXPBUFSIZEW {
        _TXPBUFSIZEW { w: self }
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline]
    pub fn txpbuftcpen(&mut self) -> _TXPBUFTCPENW {
        _TXPBUFTCPENW { w: self }
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline]
    pub fn inflastdbufsizeen(&mut self) -> _INFLASTDBUFSIZEENW {
        _INFLASTDBUFSIZEENW { w: self }
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline]
    pub fn rxbufsize(&mut self) -> _RXBUFSIZEW {
        _RXBUFSIZEW { w: self }
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline]
    pub fn frcdiscardonerr(&mut self) -> _FRCDISCARDONERRW {
        _FRCDISCARDONERRW { w: self }
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline]
    pub fn frcmaxambabrstrx(&mut self) -> _FRCMAXAMBABRSTRXW {
        _FRCMAXAMBABRSTRXW { w: self }
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline]
    pub fn frcmaxambabrsttx(&mut self) -> _FRCMAXAMBABRSTTXW {
        _FRCMAXAMBABRSTTXW { w: self }
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline]
    pub fn rxbdextndmodeen(&mut self) -> _RXBDEXTNDMODEENW {
        _RXBDEXTNDMODEENW { w: self }
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline]
    pub fn txbdextendmodeen(&mut self) -> _TXBDEXTENDMODEENW {
        _TXBDEXTENDMODEENW { w: self }
    }
}
