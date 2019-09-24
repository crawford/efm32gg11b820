#[doc = "Reader of register PERCTRL"]
pub type R = crate::R<u32, super::PERCTRL>;
#[doc = "Writer for register PERCTRL"]
pub type W = crate::W<u32, super::PERCTRL>;
#[doc = "Register PERCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PERCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DACCH0EN`"]
pub type DACCH0EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCH0EN`"]
pub struct DACCH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCH0EN_W<'a> {
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
#[doc = "Reader of field `DACCH1EN`"]
pub type DACCH1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCH1EN`"]
pub struct DACCH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCH1EN_W<'a> {
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
#[doc = "Reader of field `DACCH0DATA`"]
pub type DACCH0DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCH0DATA`"]
pub struct DACCH0DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCH0DATA_W<'a> {
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
#[doc = "Reader of field `DACCH1DATA`"]
pub type DACCH1DATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCH1DATA`"]
pub struct DACCH1DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCH1DATA_W<'a> {
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
#[doc = "Reader of field `DACSTARTUP`"]
pub type DACSTARTUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACSTARTUP`"]
pub struct DACSTARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> DACSTARTUP_W<'a> {
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
#[doc = "Reader of field `DACCONVTRIG`"]
pub type DACCONVTRIG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DACCONVTRIG`"]
pub struct DACCONVTRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> DACCONVTRIG_W<'a> {
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
#[doc = "ACMP0 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP0MODE_A {
    #[doc = "0: LESENSE does not control ACMP0"]
    DISABLE,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP0"]
    MUX,
    #[doc = "2: LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    MUXTHRES,
}
impl From<ACMP0MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP0MODE_A) -> Self {
        match variant {
            ACMP0MODE_A::DISABLE => 0,
            ACMP0MODE_A::MUX => 1,
            ACMP0MODE_A::MUXTHRES => 2,
        }
    }
}
#[doc = "Reader of field `ACMP0MODE`"]
pub type ACMP0MODE_R = crate::R<u8, ACMP0MODE_A>;
impl ACMP0MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACMP0MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACMP0MODE_A::DISABLE),
            1 => Val(ACMP0MODE_A::MUX),
            2 => Val(ACMP0MODE_A::MUXTHRES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP0MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP0MODE_A::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP0MODE_A::MUXTHRES
    }
}
#[doc = "Write proxy for field `ACMP0MODE`"]
pub struct ACMP0MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP0MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LESENSE does not control ACMP0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP0"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::MUX)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) and the threshold value (VDDLEVEL) of ACMP0"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP0MODE_A::MUXTHRES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "ACMP1 Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACMP1MODE_A {
    #[doc = "0: LESENSE does not control ACMP1"]
    DISABLE,
    #[doc = "1: LESENSE controls the input mux (POSSEL) of ACMP1"]
    MUX,
    #[doc = "2: LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    MUXTHRES,
}
impl From<ACMP1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: ACMP1MODE_A) -> Self {
        match variant {
            ACMP1MODE_A::DISABLE => 0,
            ACMP1MODE_A::MUX => 1,
            ACMP1MODE_A::MUXTHRES => 2,
        }
    }
}
#[doc = "Reader of field `ACMP1MODE`"]
pub type ACMP1MODE_R = crate::R<u8, ACMP1MODE_A>;
impl ACMP1MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ACMP1MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ACMP1MODE_A::DISABLE),
            1 => Val(ACMP1MODE_A::MUX),
            2 => Val(ACMP1MODE_A::MUXTHRES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACMP1MODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `MUX`"]
    #[inline(always)]
    pub fn is_mux(&self) -> bool {
        *self == ACMP1MODE_A::MUX
    }
    #[doc = "Checks if the value of the field is `MUXTHRES`"]
    #[inline(always)]
    pub fn is_muxthres(&self) -> bool {
        *self == ACMP1MODE_A::MUXTHRES
    }
}
#[doc = "Write proxy for field `ACMP1MODE`"]
pub struct ACMP1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACMP1MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LESENSE does not control ACMP1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::DISABLE)
    }
    #[doc = "LESENSE controls the input mux (POSSEL) of ACMP1"]
    #[inline(always)]
    pub fn mux(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::MUX)
    }
    #[doc = "LESENSE controls the input mux and the threshold value (VDDLEVEL) of ACMP1"]
    #[inline(always)]
    pub fn muxthres(self) -> &'a mut W {
        self.variant(ACMP1MODE_A::MUXTHRES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `ACMP0INV`"]
pub type ACMP0INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP0INV`"]
pub struct ACMP0INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0INV_W<'a> {
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
#[doc = "Reader of field `ACMP1INV`"]
pub type ACMP1INV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP1INV`"]
pub struct ACMP1INV_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1INV_W<'a> {
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
#[doc = "Reader of field `ACMP0HYSTEN`"]
pub type ACMP0HYSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP0HYSTEN`"]
pub struct ACMP0HYSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP0HYSTEN_W<'a> {
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
#[doc = "Reader of field `ACMP1HYSTEN`"]
pub type ACMP1HYSTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACMP1HYSTEN`"]
pub struct ACMP1HYSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACMP1HYSTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "ACMP and VDAC Duty Cycle Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WARMUPMODE_A {
    #[doc = "0: The analog comparators and VDAC are shut down when LESENSE is idle"]
    NORMAL,
    #[doc = "1: The analog comparators are kept powered up when LESENSE is idle"]
    KEEPACMPWARM,
    #[doc = "2: The VDAC is kept powered up when LESENSE is idle"]
    KEEPDACWARM,
    #[doc = "3: The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    KEEPACMPDACWARM,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        match variant {
            WARMUPMODE_A::NORMAL => 0,
            WARMUPMODE_A::KEEPACMPWARM => 1,
            WARMUPMODE_A::KEEPDACWARM => 2,
            WARMUPMODE_A::KEEPACMPDACWARM => 3,
        }
    }
}
#[doc = "Reader of field `WARMUPMODE`"]
pub type WARMUPMODE_R = crate::R<u8, WARMUPMODE_A>;
impl WARMUPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::KEEPACMPWARM,
            2 => WARMUPMODE_A::KEEPDACWARM,
            3 => WARMUPMODE_A::KEEPACMPDACWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPACMPWARM`"]
    #[inline(always)]
    pub fn is_keepacmpwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPACMPWARM
    }
    #[doc = "Checks if the value of the field is `KEEPDACWARM`"]
    #[inline(always)]
    pub fn is_keepdacwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPDACWARM
    }
    #[doc = "Checks if the value of the field is `KEEPACMPDACWARM`"]
    #[inline(always)]
    pub fn is_keepacmpdacwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPACMPDACWARM
    }
}
#[doc = "Write proxy for field `WARMUPMODE`"]
pub struct WARMUPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WARMUPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The analog comparators and VDAC are shut down when LESENSE is idle"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "The analog comparators are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPACMPWARM)
    }
    #[doc = "The VDAC is kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPDACWARM)
    }
    #[doc = "The analog comparators and VDAC are kept powered up when LESENSE is idle"]
    #[inline(always)]
    pub fn keepacmpdacwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPACMPDACWARM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline(always)]
    pub fn dacch0en(&self) -> DACCH0EN_R {
        DACCH0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline(always)]
    pub fn dacch1en(&self) -> DACCH1EN_R {
        DACCH1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline(always)]
    pub fn dacch0data(&self) -> DACCH0DATA_R {
        DACCH0DATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline(always)]
    pub fn dacch1data(&self) -> DACCH1DATA_R {
        DACCH1DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline(always)]
    pub fn dacstartup(&self) -> DACSTARTUP_R {
        DACSTARTUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&self) -> DACCONVTRIG_R {
        DACCONVTRIG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline(always)]
    pub fn acmp0mode(&self) -> ACMP0MODE_R {
        ACMP0MODE_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline(always)]
    pub fn acmp1mode(&self) -> ACMP1MODE_R {
        ACMP1MODE_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline(always)]
    pub fn acmp0inv(&self) -> ACMP0INV_R {
        ACMP0INV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline(always)]
    pub fn acmp1inv(&self) -> ACMP1INV_R {
        ACMP1INV_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp0hysten(&self) -> ACMP0HYSTEN_R {
        ACMP0HYSTEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp1hysten(&self) -> ACMP1HYSTEN_R {
        ACMP1HYSTEN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VDAC CH0 Enable"]
    #[inline(always)]
    pub fn dacch0en(&mut self) -> DACCH0EN_W {
        DACCH0EN_W { w: self }
    }
    #[doc = "Bit 1 - VDAC CH1 Enable"]
    #[inline(always)]
    pub fn dacch1en(&mut self) -> DACCH1EN_W {
        DACCH1EN_W { w: self }
    }
    #[doc = "Bit 2 - VDAC CH0 Data Selection"]
    #[inline(always)]
    pub fn dacch0data(&mut self) -> DACCH0DATA_W {
        DACCH0DATA_W { w: self }
    }
    #[doc = "Bit 3 - VDAC CH1 Data Selection"]
    #[inline(always)]
    pub fn dacch1data(&mut self) -> DACCH1DATA_W {
        DACCH1DATA_W { w: self }
    }
    #[doc = "Bit 6 - VDAC Startup Configuration"]
    #[inline(always)]
    pub fn dacstartup(&mut self) -> DACSTARTUP_W {
        DACSTARTUP_W { w: self }
    }
    #[doc = "Bit 8 - VDAC Conversion Trigger Configuration"]
    #[inline(always)]
    pub fn dacconvtrig(&mut self) -> DACCONVTRIG_W {
        DACCONVTRIG_W { w: self }
    }
    #[doc = "Bits 20:21 - ACMP0 Mode"]
    #[inline(always)]
    pub fn acmp0mode(&mut self) -> ACMP0MODE_W {
        ACMP0MODE_W { w: self }
    }
    #[doc = "Bits 22:23 - ACMP1 Mode"]
    #[inline(always)]
    pub fn acmp1mode(&mut self) -> ACMP1MODE_W {
        ACMP1MODE_W { w: self }
    }
    #[doc = "Bit 24 - Invert Analog Comparator 0 Output"]
    #[inline(always)]
    pub fn acmp0inv(&mut self) -> ACMP0INV_W {
        ACMP0INV_W { w: self }
    }
    #[doc = "Bit 25 - Invert Analog Comparator 1 Output"]
    #[inline(always)]
    pub fn acmp1inv(&mut self) -> ACMP1INV_W {
        ACMP1INV_W { w: self }
    }
    #[doc = "Bit 26 - ACMP0 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp0hysten(&mut self) -> ACMP0HYSTEN_W {
        ACMP0HYSTEN_W { w: self }
    }
    #[doc = "Bit 27 - ACMP1 Hysteresis Enable"]
    #[inline(always)]
    pub fn acmp1hysten(&mut self) -> ACMP1HYSTEN_W {
        ACMP1HYSTEN_W { w: self }
    }
    #[doc = "Bits 28:29 - ACMP and VDAC Duty Cycle Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W {
        WARMUPMODE_W { w: self }
    }
}
