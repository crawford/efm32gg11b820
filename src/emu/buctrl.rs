#[doc = "Reader of register BUCTRL"]
pub type R = crate::R<u32, super::BUCTRL>;
#[doc = "Writer for register BUCTRL"]
pub type W = crate::W<u32, super::BUCTRL>;
#[doc = "Register BUCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BUCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `STATEN`"]
pub type STATEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STATEN`"]
pub struct STATEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STATEN_W<'a> {
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
#[doc = "Reader of field `BUVINPROBEEN`"]
pub type BUVINPROBEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BUVINPROBEEN`"]
pub struct BUVINPROBEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BUVINPROBEEN_W<'a> {
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
#[doc = "Possible values of the field `VOUTRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VOUTRES_A {
    #[doc = "BU_VOUT is not connected"]
    DIS,
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    WEAK,
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    MED,
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    STRONG,
}
impl crate::ToBits<u8> for VOUTRES_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            VOUTRES_A::DIS => 0,
            VOUTRES_A::WEAK => 1,
            VOUTRES_A::MED => 2,
            VOUTRES_A::STRONG => 3,
        }
    }
}
#[doc = "Reader of field `VOUTRES`"]
pub type VOUTRES_R = crate::R<u8, VOUTRES_A>;
impl VOUTRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUTRES_A {
        match self.bits {
            0 => VOUTRES_A::DIS,
            1 => VOUTRES_A::WEAK,
            2 => VOUTRES_A::MED,
            3 => VOUTRES_A::STRONG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VOUTRES_A::DIS
    }
    #[doc = "Checks if the value of the field is `WEAK`"]
    #[inline(always)]
    pub fn is_weak(&self) -> bool {
        *self == VOUTRES_A::WEAK
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == VOUTRES_A::MED
    }
    #[doc = "Checks if the value of the field is `STRONG`"]
    #[inline(always)]
    pub fn is_strong(&self) -> bool {
        *self == VOUTRES_A::STRONG
    }
}
#[doc = "Write proxy for field `VOUTRES`"]
pub struct VOUTRES_W<'a> {
    w: &'a mut W,
}
impl<'a> VOUTRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VOUTRES_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "BU_VOUT is not connected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VOUTRES_A::DIS)
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn weak(self) -> &'a mut W {
        self.variant(VOUTRES_A::WEAK)
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(VOUTRES_A::MED)
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn strong(self) -> &'a mut W {
        self.variant(VOUTRES_A::STRONG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `PWRRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWRRES_A {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    RES0,
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    RES1,
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    RES2,
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    RES3,
}
impl crate::ToBits<u8> for PWRRES_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PWRRES_A::RES0 => 0,
            PWRRES_A::RES1 => 1,
            PWRRES_A::RES2 => 2,
            PWRRES_A::RES3 => 3,
        }
    }
}
#[doc = "Reader of field `PWRRES`"]
pub type PWRRES_R = crate::R<u8, PWRRES_A>;
impl PWRRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRRES_A {
        match self.bits {
            0 => PWRRES_A::RES0,
            1 => PWRRES_A::RES1,
            2 => PWRRES_A::RES2,
            3 => PWRRES_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == PWRRES_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == PWRRES_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == PWRRES_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == PWRRES_A::RES3
    }
}
#[doc = "Write proxy for field `PWRRES`"]
pub struct PWRRES_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWRRES_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(PWRRES_A::RES0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(PWRRES_A::RES1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(PWRRES_A::RES2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(PWRRES_A::RES3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `BUACTPWRCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUACTPWRCON_A {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl crate::ToBits<u8> for BUACTPWRCON_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BUACTPWRCON_A::NONE => 0,
            BUACTPWRCON_A::MAINBU => 1,
            BUACTPWRCON_A::BUMAIN => 2,
            BUACTPWRCON_A::NODIODE => 3,
        }
    }
}
#[doc = "Reader of field `BUACTPWRCON`"]
pub type BUACTPWRCON_R = crate::R<u8, BUACTPWRCON_A>;
impl BUACTPWRCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUACTPWRCON_A {
        match self.bits {
            0 => BUACTPWRCON_A::NONE,
            1 => BUACTPWRCON_A::MAINBU,
            2 => BUACTPWRCON_A::BUMAIN,
            3 => BUACTPWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUACTPWRCON_A::NONE
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == BUACTPWRCON_A::MAINBU
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == BUACTPWRCON_A::BUMAIN
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == BUACTPWRCON_A::NODIODE
    }
}
#[doc = "Write proxy for field `BUACTPWRCON`"]
pub struct BUACTPWRCON_W<'a> {
    w: &'a mut W,
}
impl<'a> BUACTPWRCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUACTPWRCON_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::NODIODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `BUINACTPWRCON`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUINACTPWRCON_A {
    #[doc = "No connection."]
    NONE,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU,
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN,
    #[doc = "Main power and backup power are connected without diode."]
    NODIODE,
}
impl crate::ToBits<u8> for BUINACTPWRCON_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            BUINACTPWRCON_A::NONE => 0,
            BUINACTPWRCON_A::MAINBU => 1,
            BUINACTPWRCON_A::BUMAIN => 2,
            BUINACTPWRCON_A::NODIODE => 3,
        }
    }
}
#[doc = "Reader of field `BUINACTPWRCON`"]
pub type BUINACTPWRCON_R = crate::R<u8, BUINACTPWRCON_A>;
impl BUINACTPWRCON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUINACTPWRCON_A {
        match self.bits {
            0 => BUINACTPWRCON_A::NONE,
            1 => BUINACTPWRCON_A::MAINBU,
            2 => BUINACTPWRCON_A::BUMAIN,
            3 => BUINACTPWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUINACTPWRCON_A::NONE
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == BUINACTPWRCON_A::MAINBU
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == BUINACTPWRCON_A::BUMAIN
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == BUINACTPWRCON_A::NODIODE
    }
}
#[doc = "Write proxy for field `BUINACTPWRCON`"]
pub struct BUINACTPWRCON_W<'a> {
    w: &'a mut W,
}
impl<'a> BUINACTPWRCON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BUINACTPWRCON_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::NODIODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DISMAXCOMP`"]
pub type DISMAXCOMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISMAXCOMP`"]
pub struct DISMAXCOMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DISMAXCOMP_W<'a> {
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
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&self) -> STATEN_R {
        STATEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&self) -> BUVINPROBEEN_R {
        BUVINPROBEEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&self) -> VOUTRES_R {
        VOUTRES_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&self) -> PWRRES_R {
        PWRRES_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&self) -> BUACTPWRCON_R {
        BUACTPWRCON_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&self) -> BUINACTPWRCON_R {
        BUINACTPWRCON_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&self) -> DISMAXCOMP_R {
        DISMAXCOMP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&mut self) -> STATEN_W {
        STATEN_W { w: self }
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&mut self) -> BUVINPROBEEN_W {
        BUVINPROBEEN_W { w: self }
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&mut self) -> VOUTRES_W {
        VOUTRES_W { w: self }
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&mut self) -> PWRRES_W {
        PWRRES_W { w: self }
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&mut self) -> BUACTPWRCON_W {
        BUACTPWRCON_W { w: self }
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&mut self) -> BUINACTPWRCON_W {
        BUINACTPWRCON_W { w: self }
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&mut self) -> DISMAXCOMP_W {
        DISMAXCOMP_W { w: self }
    }
}
