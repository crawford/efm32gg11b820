#[doc = "Reader of register FEVTERRSTAT"]
pub type R = crate::R<u32, super::FEVTERRSTAT>;
#[doc = "Writer for register FEVTERRSTAT"]
pub type W = crate::W<u32, super::FEVTERRSTAT>;
#[doc = "Register FEVTERRSTAT `reset()`'s with value 0"]
impl crate::ResetValue for super::FEVTERRSTAT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AC12NEX`"]
pub type AC12NEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC12NEX`"]
pub struct AC12NEX_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12NEX_W<'a> {
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
#[doc = "Reader of field `AC12TOE`"]
pub type AC12TOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC12TOE`"]
pub struct AC12TOE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12TOE_W<'a> {
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
#[doc = "Reader of field `AC12CRCE`"]
pub type AC12CRCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC12CRCE`"]
pub struct AC12CRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12CRCE_W<'a> {
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
#[doc = "Reader of field `AC12EBE`"]
pub type AC12EBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC12EBE`"]
pub struct AC12EBE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12EBE_W<'a> {
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
#[doc = "Reader of field `AC12INDXE`"]
pub type AC12INDXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC12INDXE`"]
pub struct AC12INDXE_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12INDXE_W<'a> {
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
#[doc = "Reader of field `CNIBAC12E`"]
pub type CNIBAC12E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CNIBAC12E`"]
pub struct CNIBAC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> CNIBAC12E_W<'a> {
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
#[doc = "Reader of field `CMDTOE`"]
pub type CMDTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDTOE`"]
pub struct CMDTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `CMDCRCE`"]
pub type CMDCRCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDCRCE`"]
pub struct CMDCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCRCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `CMDEBE`"]
pub type CMDEBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDEBE`"]
pub struct CMDEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `CMDINDXE`"]
pub type CMDINDXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDINDXE`"]
pub struct CMDINDXE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDINDXE_W<'a> {
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
#[doc = "Reader of field `DATTOE`"]
pub type DATTOE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATTOE`"]
pub struct DATTOE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATTOE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DATCRCE`"]
pub type DATCRCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATCRCE`"]
pub struct DATCRCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATCRCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DATEBE`"]
pub type DATEBE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DATEBE`"]
pub struct DATEBE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATEBE_W<'a> {
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
#[doc = "Reader of field `CURLIMITE`"]
pub type CURLIMITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CURLIMITE`"]
pub struct CURLIMITE_W<'a> {
    w: &'a mut W,
}
impl<'a> CURLIMITE_W<'a> {
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
#[doc = "Reader of field `AC12E`"]
pub type AC12E_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AC12E`"]
pub struct AC12E_W<'a> {
    w: &'a mut W,
}
impl<'a> AC12E_W<'a> {
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
#[doc = "Reader of field `ADMAE`"]
pub type ADMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADMAE`"]
pub struct ADMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADMAE_W<'a> {
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
#[doc = "Reader of field `TUNINGE`"]
pub type TUNINGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `VENSPECE`"]
pub type VENSPECE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Force Event for Command Not Issued By Auto CM12 Not Executed"]
    #[inline(always)]
    pub fn ac12nex(&self) -> AC12NEX_R {
        AC12NEX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crce(&self) -> AC12CRCE_R {
        AC12CRCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> AC12EBE_R {
        AC12EBE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indxe(&self) -> AC12INDXE_R {
        AC12INDXE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> CNIBAC12E_R {
        CNIBAC12E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtoe(&self) -> CMDTOE_R {
        CMDTOE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrce(&self) -> CMDCRCE_R {
        CMDCRCE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdebe(&self) -> CMDEBE_R {
        CMDEBE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdindxe(&self) -> CMDINDXE_R {
        CMDINDXE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn dattoe(&self) -> DATTOE_R {
        DATTOE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrce(&self) -> DATCRCE_R {
        DATCRCE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datebe(&self) -> DATEBE_R {
        DATEBE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlimite(&self) -> CURLIMITE_R {
        CURLIMITE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn ac12e(&self) -> AC12E_R {
        AC12E_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn admae(&self) -> ADMAE_R {
        ADMAE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Force Event for Tuning Errro"]
    #[inline(always)]
    pub fn tuninge(&self) -> TUNINGE_R {
        TUNINGE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Force Event for Vendox Specific Error Status"]
    #[inline(always)]
    pub fn venspece(&self) -> VENSPECE_R {
        VENSPECE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force Event for Command Not Issued By Auto CM12 Not Executed"]
    #[inline(always)]
    pub fn ac12nex(&mut self) -> AC12NEX_W {
        AC12NEX_W { w: self }
    }
    #[doc = "Bit 1 - Force Event for Auto CMD Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&mut self) -> AC12TOE_W {
        AC12TOE_W { w: self }
    }
    #[doc = "Bit 2 - Force Event for Auto CMD CRC Error"]
    #[inline(always)]
    pub fn ac12crce(&mut self) -> AC12CRCE_W {
        AC12CRCE_W { w: self }
    }
    #[doc = "Bit 3 - Force Event for Auto CMD End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&mut self) -> AC12EBE_W {
        AC12EBE_W { w: self }
    }
    #[doc = "Bit 4 - Force Event for Auto CMD Index Error"]
    #[inline(always)]
    pub fn ac12indxe(&mut self) -> AC12INDXE_W {
        AC12INDXE_W { w: self }
    }
    #[doc = "Bit 7 - Force Event for Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&mut self) -> CNIBAC12E_W {
        CNIBAC12E_W { w: self }
    }
    #[doc = "Bit 16 - Force Event for Command Timeout Error"]
    #[inline(always)]
    pub fn cmdtoe(&mut self) -> CMDTOE_W {
        CMDTOE_W { w: self }
    }
    #[doc = "Bit 17 - Force Event for Command CRC Error"]
    #[inline(always)]
    pub fn cmdcrce(&mut self) -> CMDCRCE_W {
        CMDCRCE_W { w: self }
    }
    #[doc = "Bit 18 - Force Event for Command End Bit Error"]
    #[inline(always)]
    pub fn cmdebe(&mut self) -> CMDEBE_W {
        CMDEBE_W { w: self }
    }
    #[doc = "Bit 19 - Force Event for Command Index Error"]
    #[inline(always)]
    pub fn cmdindxe(&mut self) -> CMDINDXE_W {
        CMDINDXE_W { w: self }
    }
    #[doc = "Bit 20 - Force Event for Data Timeout Error"]
    #[inline(always)]
    pub fn dattoe(&mut self) -> DATTOE_W {
        DATTOE_W { w: self }
    }
    #[doc = "Bit 21 - Force Event for Data CRC Error"]
    #[inline(always)]
    pub fn datcrce(&mut self) -> DATCRCE_W {
        DATCRCE_W { w: self }
    }
    #[doc = "Bit 22 - Force Event for Data End Bit Error"]
    #[inline(always)]
    pub fn datebe(&mut self) -> DATEBE_W {
        DATEBE_W { w: self }
    }
    #[doc = "Bit 23 - Force Event for Current Limit Error"]
    #[inline(always)]
    pub fn curlimite(&mut self) -> CURLIMITE_W {
        CURLIMITE_W { w: self }
    }
    #[doc = "Bit 24 - Force Event for Auto CMD Error"]
    #[inline(always)]
    pub fn ac12e(&mut self) -> AC12E_W {
        AC12E_W { w: self }
    }
    #[doc = "Bit 25 - Force Event for ADMA Error"]
    #[inline(always)]
    pub fn admae(&mut self) -> ADMAE_W {
        ADMAE_W { w: self }
    }
}
