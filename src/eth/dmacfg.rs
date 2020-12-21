#[doc = "Reader of register DMACFG"]
pub type R = crate::R<u32, super::DMACFG>;
#[doc = "Writer for register DMACFG"]
pub type W = crate::W<u32, super::DMACFG>;
#[doc = "Register DMACFG `reset()`'s with value 0x0002_0704"]
impl crate::ResetValue for super::DMACFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0002_0704
    }
}
#[doc = "Reader of field `AMBABRSTLEN`"]
pub type AMBABRSTLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMBABRSTLEN`"]
pub struct AMBABRSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AMBABRSTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `HDRDATASPLITEN`"]
pub type HDRDATASPLITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDRDATASPLITEN`"]
pub struct HDRDATASPLITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HDRDATASPLITEN_W<'a> {
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
#[doc = "Receiver packet buffer memory size select.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPBUFSIZE_A {
    #[doc = "0: Do not use top three address bits (0.5 Kb)"]
    SIZE0 = 0,
    #[doc = "1: Do not use top two address bits (1 Kb)"]
    SIZE1 = 1,
    #[doc = "2: Do not use top address bit (2 Kb)"]
    SIZE2 = 2,
    #[doc = "3: Use full configured addressable space (4 Kb)"]
    SIZE3 = 3,
}
impl From<RXPBUFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPBUFSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RXPBUFSIZE`"]
pub type RXPBUFSIZE_R = crate::R<u8, RXPBUFSIZE_A>;
impl RXPBUFSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPBUFSIZE_A {
        match self.bits {
            0 => RXPBUFSIZE_A::SIZE0,
            1 => RXPBUFSIZE_A::SIZE1,
            2 => RXPBUFSIZE_A::SIZE2,
            3 => RXPBUFSIZE_A::SIZE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE0`"]
    #[inline(always)]
    pub fn is_size0(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE0
    }
    #[doc = "Checks if the value of the field is `SIZE1`"]
    #[inline(always)]
    pub fn is_size1(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE1
    }
    #[doc = "Checks if the value of the field is `SIZE2`"]
    #[inline(always)]
    pub fn is_size2(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE2
    }
    #[doc = "Checks if the value of the field is `SIZE3`"]
    #[inline(always)]
    pub fn is_size3(&self) -> bool {
        *self == RXPBUFSIZE_A::SIZE3
    }
}
#[doc = "Write proxy for field `RXPBUFSIZE`"]
pub struct RXPBUFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPBUFSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXPBUFSIZE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Do not use top three address bits (0.5 Kb)"]
    #[inline(always)]
    pub fn size0(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE0)
    }
    #[doc = "Do not use top two address bits (1 Kb)"]
    #[inline(always)]
    pub fn size1(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE1)
    }
    #[doc = "Do not use top address bit (2 Kb)"]
    #[inline(always)]
    pub fn size2(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE2)
    }
    #[doc = "Use full configured addressable space (4 Kb)"]
    #[inline(always)]
    pub fn size3(self) -> &'a mut W {
        self.variant(RXPBUFSIZE_A::SIZE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXPBUFSIZE`"]
pub type TXPBUFSIZE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPBUFSIZE`"]
pub struct TXPBUFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBUFSIZE_W<'a> {
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
#[doc = "Reader of field `TXPBUFTCPEN`"]
pub type TXPBUFTCPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPBUFTCPEN`"]
pub struct TXPBUFTCPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPBUFTCPEN_W<'a> {
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
#[doc = "Reader of field `INFLASTDBUFSIZEEN`"]
pub type INFLASTDBUFSIZEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INFLASTDBUFSIZEEN`"]
pub struct INFLASTDBUFSIZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> INFLASTDBUFSIZEEN_W<'a> {
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
#[doc = "Reader of field `RXBUFSIZE`"]
pub type RXBUFSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXBUFSIZE`"]
pub struct RXBUFSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `FRCDISCARDONERR`"]
pub type FRCDISCARDONERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRCDISCARDONERR`"]
pub struct FRCDISCARDONERR_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCDISCARDONERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `FRCMAXAMBABRSTRX`"]
pub type FRCMAXAMBABRSTRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRCMAXAMBABRSTRX`"]
pub struct FRCMAXAMBABRSTRX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCMAXAMBABRSTRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `FRCMAXAMBABRSTTX`"]
pub type FRCMAXAMBABRSTTX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FRCMAXAMBABRSTTX`"]
pub struct FRCMAXAMBABRSTTX_W<'a> {
    w: &'a mut W,
}
impl<'a> FRCMAXAMBABRSTTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `RXBDEXTNDMODEEN`"]
pub type RXBDEXTNDMODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXBDEXTNDMODEEN`"]
pub struct RXBDEXTNDMODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBDEXTNDMODEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `TXBDEXTENDMODEEN`"]
pub type TXBDEXTENDMODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXBDEXTENDMODEEN`"]
pub struct TXBDEXTENDMODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXBDEXTENDMODEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&self) -> AMBABRSTLEN_R {
        AMBABRSTLEN_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&self) -> HDRDATASPLITEN_R {
        HDRDATASPLITEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&self) -> RXPBUFSIZE_R {
        RXPBUFSIZE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&self) -> TXPBUFSIZE_R {
        TXPBUFSIZE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&self) -> TXPBUFTCPEN_R {
        TXPBUFTCPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&self) -> INFLASTDBUFSIZEEN_R {
        INFLASTDBUFSIZEEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&self) -> RXBUFSIZE_R {
        RXBUFSIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&self) -> FRCDISCARDONERR_R {
        FRCDISCARDONERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&self) -> FRCMAXAMBABRSTRX_R {
        FRCMAXAMBABRSTRX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&self) -> FRCMAXAMBABRSTTX_R {
        FRCMAXAMBABRSTTX_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&self) -> RXBDEXTNDMODEEN_R {
        RXBDEXTNDMODEEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&self) -> TXBDEXTENDMODEEN_R {
        TXBDEXTENDMODEEN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the burst length to use on the AMBA (AHB) when transferring frame data."]
    #[inline(always)]
    pub fn ambabrstlen(&mut self) -> AMBABRSTLEN_W {
        AMBABRSTLEN_W { w: self }
    }
    #[doc = "Bit 5 - Enable header data Splitting."]
    #[inline(always)]
    pub fn hdrdataspliten(&mut self) -> HDRDATASPLITEN_W {
        HDRDATASPLITEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Receiver packet buffer memory size select."]
    #[inline(always)]
    pub fn rxpbufsize(&mut self) -> RXPBUFSIZE_W {
        RXPBUFSIZE_W { w: self }
    }
    #[doc = "Bit 10 - Transmitter packet buffer memory size select."]
    #[inline(always)]
    pub fn txpbufsize(&mut self) -> TXPBUFSIZE_W {
        TXPBUFSIZE_W { w: self }
    }
    #[doc = "Bit 11 - Transmitter IP, TCP and UDP checksum generation offload enable"]
    #[inline(always)]
    pub fn txpbuftcpen(&mut self) -> TXPBUFTCPEN_W {
        TXPBUFTCPEN_W { w: self }
    }
    #[doc = "Bit 12 - Forces the DMA"]
    #[inline(always)]
    pub fn inflastdbufsizeen(&mut self) -> INFLASTDBUFSIZEEN_W {
        INFLASTDBUFSIZEEN_W { w: self }
    }
    #[doc = "Bits 16:23 - DMA receive buffer size in external AMBA (AHB) system memory."]
    #[inline(always)]
    pub fn rxbufsize(&mut self) -> RXBUFSIZE_W {
        RXBUFSIZE_W { w: self }
    }
    #[doc = "Bit 24 - Auto Discard RX pkts during lack of resource."]
    #[inline(always)]
    pub fn frcdiscardonerr(&mut self) -> FRCDISCARDONERR_W {
        FRCDISCARDONERR_W { w: self }
    }
    #[doc = "Bit 25 - Force max length bursts on RX."]
    #[inline(always)]
    pub fn frcmaxambabrstrx(&mut self) -> FRCMAXAMBABRSTRX_W {
        FRCMAXAMBABRSTRX_W { w: self }
    }
    #[doc = "Bit 26 - Force max length bursts on TX."]
    #[inline(always)]
    pub fn frcmaxambabrsttx(&mut self) -> FRCMAXAMBABRSTTX_W {
        FRCMAXAMBABRSTTX_W { w: self }
    }
    #[doc = "Bit 28 - Enable RX extended BD mode."]
    #[inline(always)]
    pub fn rxbdextndmodeen(&mut self) -> RXBDEXTNDMODEEN_W {
        RXBDEXTNDMODEEN_W { w: self }
    }
    #[doc = "Bit 29 - Enable TX extended BD mode."]
    #[inline(always)]
    pub fn txbdextendmodeen(&mut self) -> TXBDEXTENDMODEEN_W {
        TXBDEXTENDMODEEN_W { w: self }
    }
}
