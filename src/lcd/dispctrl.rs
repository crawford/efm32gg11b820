#[doc = "Reader of register DISPCTRL"]
pub type R = crate::R<u32, super::DISPCTRL>;
#[doc = "Writer for register DISPCTRL"]
pub type W = crate::W<u32, super::DISPCTRL>;
#[doc = "Register DISPCTRL `reset()`'s with value 0x0010_3f00"]
impl crate::ResetValue for super::DISPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_3f00
    }
}
#[doc = "Possible values of the field `MUX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUX_A {
    #[doc = "Static"]
    STATIC,
    #[doc = "Duplex"]
    DUPLEX,
    #[doc = "Triplex"]
    TRIPLEX,
    #[doc = "Quadruplex"]
    QUADRUPLEX,
    #[doc = "Sextaplex"]
    SEXTAPLEX,
    #[doc = "Octaplex"]
    OCTAPLEX,
}
impl crate::ToBits<u8> for MUX_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MUX_A::STATIC => 0,
            MUX_A::DUPLEX => 1,
            MUX_A::TRIPLEX => 2,
            MUX_A::QUADRUPLEX => 3,
            MUX_A::SEXTAPLEX => 5,
            MUX_A::OCTAPLEX => 7,
        }
    }
}
#[doc = "Reader of field `MUX`"]
pub type MUX_R = crate::R<u8, MUX_A>;
impl MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MUX_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MUX_A::STATIC),
            1 => Val(MUX_A::DUPLEX),
            2 => Val(MUX_A::TRIPLEX),
            3 => Val(MUX_A::QUADRUPLEX),
            5 => Val(MUX_A::SEXTAPLEX),
            7 => Val(MUX_A::OCTAPLEX),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static_(&self) -> bool {
        *self == MUX_A::STATIC
    }
    #[doc = "Checks if the value of the field is `DUPLEX`"]
    #[inline(always)]
    pub fn is_duplex(&self) -> bool {
        *self == MUX_A::DUPLEX
    }
    #[doc = "Checks if the value of the field is `TRIPLEX`"]
    #[inline(always)]
    pub fn is_triplex(&self) -> bool {
        *self == MUX_A::TRIPLEX
    }
    #[doc = "Checks if the value of the field is `QUADRUPLEX`"]
    #[inline(always)]
    pub fn is_quadruplex(&self) -> bool {
        *self == MUX_A::QUADRUPLEX
    }
    #[doc = "Checks if the value of the field is `SEXTAPLEX`"]
    #[inline(always)]
    pub fn is_sextaplex(&self) -> bool {
        *self == MUX_A::SEXTAPLEX
    }
    #[doc = "Checks if the value of the field is `OCTAPLEX`"]
    #[inline(always)]
    pub fn is_octaplex(&self) -> bool {
        *self == MUX_A::OCTAPLEX
    }
}
#[doc = "Write proxy for field `MUX`"]
pub struct MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(MUX_A::STATIC)
    }
    #[doc = "Duplex"]
    #[inline(always)]
    pub fn duplex(self) -> &'a mut W {
        self.variant(MUX_A::DUPLEX)
    }
    #[doc = "Triplex"]
    #[inline(always)]
    pub fn triplex(self) -> &'a mut W {
        self.variant(MUX_A::TRIPLEX)
    }
    #[doc = "Quadruplex"]
    #[inline(always)]
    pub fn quadruplex(self) -> &'a mut W {
        self.variant(MUX_A::QUADRUPLEX)
    }
    #[doc = "Sextaplex"]
    #[inline(always)]
    pub fn sextaplex(self) -> &'a mut W {
        self.variant(MUX_A::SEXTAPLEX)
    }
    #[doc = "Octaplex"]
    #[inline(always)]
    pub fn octaplex(self) -> &'a mut W {
        self.variant(MUX_A::OCTAPLEX)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
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
#[doc = "Reader of field `CONTRAST`"]
pub type CONTRAST_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CONTRAST`"]
pub struct CONTRAST_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTRAST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CHGRDST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHGRDST_A {
    #[doc = "Disable charge redistribution."]
    DISABLE,
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    ONE,
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    TWO,
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    THREE,
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    FOUR,
}
impl crate::ToBits<u8> for CHGRDST_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CHGRDST_A::DISABLE => 0,
            CHGRDST_A::ONE => 1,
            CHGRDST_A::TWO => 2,
            CHGRDST_A::THREE => 3,
            CHGRDST_A::FOUR => 4,
        }
    }
}
#[doc = "Reader of field `CHGRDST`"]
pub type CHGRDST_R = crate::R<u8, CHGRDST_A>;
impl CHGRDST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHGRDST_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHGRDST_A::DISABLE),
            1 => Val(CHGRDST_A::ONE),
            2 => Val(CHGRDST_A::TWO),
            3 => Val(CHGRDST_A::THREE),
            4 => Val(CHGRDST_A::FOUR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHGRDST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CHGRDST_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CHGRDST_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CHGRDST_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == CHGRDST_A::FOUR
    }
}
#[doc = "Write proxy for field `CHGRDST`"]
pub struct CHGRDST_W<'a> {
    w: &'a mut W,
}
impl<'a> CHGRDST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHGRDST_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Disable charge redistribution."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHGRDST_A::DISABLE)
    }
    #[doc = "Use 1 prescaled low frequency clock cycle for charge redistribution."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CHGRDST_A::ONE)
    }
    #[doc = "Use 2 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(CHGRDST_A::TWO)
    }
    #[doc = "Use 3 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(CHGRDST_A::THREE)
    }
    #[doc = "Use 4 prescaled low frequency clock cycles for charge redistribution."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(CHGRDST_A::FOUR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Possible values of the field `BIAS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIAS_A {
    #[doc = "Static"]
    STATIC,
    #[doc = "1/2 Bias"]
    ONEHALF,
    #[doc = "1/3 Bias"]
    ONETHIRD,
    #[doc = "1/4 Bias"]
    ONEFOURTH,
}
impl crate::ToBits<u8> for BIAS_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BIAS_A::STATIC => 0,
            BIAS_A::ONEHALF => 1,
            BIAS_A::ONETHIRD => 2,
            BIAS_A::ONEFOURTH => 3,
        }
    }
}
#[doc = "Reader of field `BIAS`"]
pub type BIAS_R = crate::R<u8, BIAS_A>;
impl BIAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIAS_A {
        match self.bits {
            0 => BIAS_A::STATIC,
            1 => BIAS_A::ONEHALF,
            2 => BIAS_A::ONETHIRD,
            3 => BIAS_A::ONEFOURTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `STATIC`"]
    #[inline(always)]
    pub fn is_static_(&self) -> bool {
        *self == BIAS_A::STATIC
    }
    #[doc = "Checks if the value of the field is `ONEHALF`"]
    #[inline(always)]
    pub fn is_onehalf(&self) -> bool {
        *self == BIAS_A::ONEHALF
    }
    #[doc = "Checks if the value of the field is `ONETHIRD`"]
    #[inline(always)]
    pub fn is_onethird(&self) -> bool {
        *self == BIAS_A::ONETHIRD
    }
    #[doc = "Checks if the value of the field is `ONEFOURTH`"]
    #[inline(always)]
    pub fn is_onefourth(&self) -> bool {
        *self == BIAS_A::ONEFOURTH
    }
}
#[doc = "Write proxy for field `BIAS`"]
pub struct BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> BIAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIAS_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Static"]
    #[inline(always)]
    pub fn static_(self) -> &'a mut W {
        self.variant(BIAS_A::STATIC)
    }
    #[doc = "1/2 Bias"]
    #[inline(always)]
    pub fn onehalf(self) -> &'a mut W {
        self.variant(BIAS_A::ONEHALF)
    }
    #[doc = "1/3 Bias"]
    #[inline(always)]
    pub fn onethird(self) -> &'a mut W {
        self.variant(BIAS_A::ONETHIRD)
    }
    #[doc = "1/4 Bias"]
    #[inline(always)]
    pub fn onefourth(self) -> &'a mut W {
        self.variant(BIAS_A::ONEFOURTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    NOEXTCAP,
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    STEPDOWN,
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    CPINTOSC,
}
impl crate::ToBits<u8> for MODE_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MODE_A::NOEXTCAP => 0,
            MODE_A::STEPDOWN => 1,
            MODE_A::CPINTOSC => 2,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::NOEXTCAP),
            1 => Val(MODE_A::STEPDOWN),
            2 => Val(MODE_A::CPINTOSC),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOEXTCAP`"]
    #[inline(always)]
    pub fn is_noextcap(&self) -> bool {
        *self == MODE_A::NOEXTCAP
    }
    #[doc = "Checks if the value of the field is `STEPDOWN`"]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == MODE_A::STEPDOWN
    }
    #[doc = "Checks if the value of the field is `CPINTOSC`"]
    #[inline(always)]
    pub fn is_cpintosc(&self) -> bool {
        *self == MODE_A::CPINTOSC
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No External Cap. Uses an internal current source to generate VLCD. Use CONTRAST\\[4:0\\] to control VLCD."]
    #[inline(always)]
    pub fn noextcap(self) -> &'a mut W {
        self.variant(MODE_A::NOEXTCAP)
    }
    #[doc = "Use step down control with VLCD less than VDD. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut W {
        self.variant(MODE_A::STEPDOWN)
    }
    #[doc = "Charge pump used with internal oscillator. Use CONTRAST\\[5:0\\] to control VLCD level, and use SPEED to adjust oscillator frequency."]
    #[inline(always)]
    pub fn cpintosc(self) -> &'a mut W {
        self.variant(MODE_A::CPINTOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&self) -> CONTRAST_R {
        CONTRAST_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&self) -> CHGRDST_R {
        CHGRDST_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mux Configuration"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W {
        MUX_W { w: self }
    }
    #[doc = "Bit 4 - Waveform Selection"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bits 8:13 - Contrast Control"]
    #[inline(always)]
    pub fn contrast(&mut self) -> CONTRAST_W {
        CONTRAST_W { w: self }
    }
    #[doc = "Bits 20:22 - Charge Redistribution Cycles"]
    #[inline(always)]
    pub fn chgrdst(&mut self) -> CHGRDST_W {
        CHGRDST_W { w: self }
    }
    #[doc = "Bits 24:25 - Bias Configuration"]
    #[inline(always)]
    pub fn bias(&mut self) -> BIAS_W {
        BIAS_W { w: self }
    }
    #[doc = "Bits 28:29 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
}
