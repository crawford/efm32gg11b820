#[doc = "Reader of register AC12ERRSTAT"]
pub type R = crate::R<u32, super::AC12ERRSTAT>;
#[doc = "Writer for register AC12ERRSTAT"]
pub type W = crate::W<u32, super::AC12ERRSTAT>;
#[doc = "Register AC12ERRSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::AC12ERRSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AC12NOTEXE`"]
pub type AC12NOTEXE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AC12TOE`"]
pub type AC12TOE_R = crate::R<bool, bool>;
#[doc = "Reader of field `AC12CRCERR`"]
pub type AC12CRCERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `AC12ENDBITERR`"]
pub type AC12ENDBITERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `AC12INDEXERR`"]
pub type AC12INDEXERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `CNIBAC12ERR`"]
pub type CNIBAC12ERR_R = crate::R<bool, bool>;
#[doc = "UHS Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UHSMODESEL_A {
    #[doc = "0: SDR12"]
    SDR12,
    #[doc = "1: SDR25"]
    SDR25,
    #[doc = "2: SDR50"]
    SDR50,
    #[doc = "3: SDR104"]
    SDR104,
    #[doc = "4: DDR50"]
    DDR50,
}
impl From<UHSMODESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UHSMODESEL_A) -> Self {
        match variant {
            UHSMODESEL_A::SDR12 => 0,
            UHSMODESEL_A::SDR25 => 1,
            UHSMODESEL_A::SDR50 => 2,
            UHSMODESEL_A::SDR104 => 3,
            UHSMODESEL_A::DDR50 => 4,
        }
    }
}
#[doc = "Reader of field `UHSMODESEL`"]
pub type UHSMODESEL_R = crate::R<u8, UHSMODESEL_A>;
impl UHSMODESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UHSMODESEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UHSMODESEL_A::SDR12),
            1 => Val(UHSMODESEL_A::SDR25),
            2 => Val(UHSMODESEL_A::SDR50),
            3 => Val(UHSMODESEL_A::SDR104),
            4 => Val(UHSMODESEL_A::DDR50),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SDR12`"]
    #[inline(always)]
    pub fn is_sdr12(&self) -> bool {
        *self == UHSMODESEL_A::SDR12
    }
    #[doc = "Checks if the value of the field is `SDR25`"]
    #[inline(always)]
    pub fn is_sdr25(&self) -> bool {
        *self == UHSMODESEL_A::SDR25
    }
    #[doc = "Checks if the value of the field is `SDR50`"]
    #[inline(always)]
    pub fn is_sdr50(&self) -> bool {
        *self == UHSMODESEL_A::SDR50
    }
    #[doc = "Checks if the value of the field is `SDR104`"]
    #[inline(always)]
    pub fn is_sdr104(&self) -> bool {
        *self == UHSMODESEL_A::SDR104
    }
    #[doc = "Checks if the value of the field is `DDR50`"]
    #[inline(always)]
    pub fn is_ddr50(&self) -> bool {
        *self == UHSMODESEL_A::DDR50
    }
}
#[doc = "Write proxy for field `UHSMODESEL`"]
pub struct UHSMODESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UHSMODESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UHSMODESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SDR12"]
    #[inline(always)]
    pub fn sdr12(self) -> &'a mut W {
        self.variant(UHSMODESEL_A::SDR12)
    }
    #[doc = "SDR25"]
    #[inline(always)]
    pub fn sdr25(self) -> &'a mut W {
        self.variant(UHSMODESEL_A::SDR25)
    }
    #[doc = "SDR50"]
    #[inline(always)]
    pub fn sdr50(self) -> &'a mut W {
        self.variant(UHSMODESEL_A::SDR50)
    }
    #[doc = "SDR104"]
    #[inline(always)]
    pub fn sdr104(self) -> &'a mut W {
        self.variant(UHSMODESEL_A::SDR104)
    }
    #[doc = "DDR50"]
    #[inline(always)]
    pub fn ddr50(self) -> &'a mut W {
        self.variant(UHSMODESEL_A::DDR50)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `SIGEN1P8V`"]
pub type SIGEN1P8V_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIGEN1P8V`"]
pub struct SIGEN1P8V_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGEN1P8V_W<'a> {
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
#[doc = "Driver Strength Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DRVSTNSEL_A {
    #[doc = "0: Driver Type B is selected (Default)"]
    TYPEB,
    #[doc = "1: Driver Type A is selected"]
    TYPEA,
    #[doc = "2: Driver Type C is selected"]
    TYPEC,
    #[doc = "3: Driver Type D is selected"]
    TYPED,
}
impl From<DRVSTNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DRVSTNSEL_A) -> Self {
        match variant {
            DRVSTNSEL_A::TYPEB => 0,
            DRVSTNSEL_A::TYPEA => 1,
            DRVSTNSEL_A::TYPEC => 2,
            DRVSTNSEL_A::TYPED => 3,
        }
    }
}
#[doc = "Reader of field `DRVSTNSEL`"]
pub type DRVSTNSEL_R = crate::R<u8, DRVSTNSEL_A>;
impl DRVSTNSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRVSTNSEL_A {
        match self.bits {
            0 => DRVSTNSEL_A::TYPEB,
            1 => DRVSTNSEL_A::TYPEA,
            2 => DRVSTNSEL_A::TYPEC,
            3 => DRVSTNSEL_A::TYPED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TYPEB`"]
    #[inline(always)]
    pub fn is_typeb(&self) -> bool {
        *self == DRVSTNSEL_A::TYPEB
    }
    #[doc = "Checks if the value of the field is `TYPEA`"]
    #[inline(always)]
    pub fn is_typea(&self) -> bool {
        *self == DRVSTNSEL_A::TYPEA
    }
    #[doc = "Checks if the value of the field is `TYPEC`"]
    #[inline(always)]
    pub fn is_typec(&self) -> bool {
        *self == DRVSTNSEL_A::TYPEC
    }
    #[doc = "Checks if the value of the field is `TYPED`"]
    #[inline(always)]
    pub fn is_typed(&self) -> bool {
        *self == DRVSTNSEL_A::TYPED
    }
}
#[doc = "Write proxy for field `DRVSTNSEL`"]
pub struct DRVSTNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DRVSTNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DRVSTNSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Driver Type B is selected (Default)"]
    #[inline(always)]
    pub fn typeb(self) -> &'a mut W {
        self.variant(DRVSTNSEL_A::TYPEB)
    }
    #[doc = "Driver Type A is selected"]
    #[inline(always)]
    pub fn typea(self) -> &'a mut W {
        self.variant(DRVSTNSEL_A::TYPEA)
    }
    #[doc = "Driver Type C is selected"]
    #[inline(always)]
    pub fn typec(self) -> &'a mut W {
        self.variant(DRVSTNSEL_A::TYPEC)
    }
    #[doc = "Driver Type D is selected"]
    #[inline(always)]
    pub fn typed(self) -> &'a mut W {
        self.variant(DRVSTNSEL_A::TYPED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `EXETUNING`"]
pub type EXETUNING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXETUNING`"]
pub struct EXETUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> EXETUNING_W<'a> {
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
#[doc = "Reader of field `SAMPCLKSEL`"]
pub type SAMPCLKSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAMPCLKSEL`"]
pub struct SAMPCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPCLKSEL_W<'a> {
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
#[doc = "Reader of field `ASYNCINTEN`"]
pub type ASYNCINTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCINTEN`"]
pub struct ASYNCINTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCINTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `PRSTVALEN`"]
pub type PRSTVALEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRSTVALEN`"]
pub struct PRSTVALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRSTVALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn ac12notexe(&self) -> AC12NOTEXE_R {
        AC12NOTEXE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crcerr(&self) -> AC12CRCERR_R {
        AC12CRCERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12endbiterr(&self) -> AC12ENDBITERR_R {
        AC12ENDBITERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indexerr(&self) -> AC12INDEXERR_R {
        AC12INDEXERR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12err(&self) -> CNIBAC12ERR_R {
        CNIBAC12ERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsmodesel(&self) -> UHSMODESEL_R {
        UHSMODESEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline(always)]
    pub fn sigen1p8v(&self) -> SIGEN1P8V_R {
        SIGEN1P8V_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvstnsel(&self) -> DRVSTNSEL_R {
        DRVSTNSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn exetuning(&self) -> EXETUNING_R {
        EXETUNING_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline(always)]
    pub fn sampclksel(&self) -> SAMPCLKSEL_R {
        SAMPCLKSEL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asyncinten(&self) -> ASYNCINTEN_R {
        ASYNCINTEN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline(always)]
    pub fn prstvalen(&self) -> PRSTVALEN_R {
        PRSTVALEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18 - UHS Mode Select"]
    #[inline(always)]
    pub fn uhsmodesel(&mut self) -> UHSMODESEL_W {
        UHSMODESEL_W { w: self }
    }
    #[doc = "Bit 19 - Voltage 1.8V Signal Enable"]
    #[inline(always)]
    pub fn sigen1p8v(&mut self) -> SIGEN1P8V_W {
        SIGEN1P8V_W { w: self }
    }
    #[doc = "Bits 20:21 - Driver Strength Select"]
    #[inline(always)]
    pub fn drvstnsel(&mut self) -> DRVSTNSEL_W {
        DRVSTNSEL_W { w: self }
    }
    #[doc = "Bit 22 - Execute Tuning"]
    #[inline(always)]
    pub fn exetuning(&mut self) -> EXETUNING_W {
        EXETUNING_W { w: self }
    }
    #[doc = "Bit 23 - Sampling Clock Select"]
    #[inline(always)]
    pub fn sampclksel(&mut self) -> SAMPCLKSEL_W {
        SAMPCLKSEL_W { w: self }
    }
    #[doc = "Bit 30 - Asynchronous Interrupt Enable"]
    #[inline(always)]
    pub fn asyncinten(&mut self) -> ASYNCINTEN_W {
        ASYNCINTEN_W { w: self }
    }
    #[doc = "Bit 31 - Preset Value Enable"]
    #[inline(always)]
    pub fn prstvalen(&mut self) -> PRSTVALEN_W {
        PRSTVALEN_W { w: self }
    }
}
