#[doc = "Reader of register ETMAUTHSTATUS"]
pub type R = crate::R<u32, super::ETMAUTHSTATUS>;
#[doc = "Reader of field `NONSECINVDBG`"]
pub type NONSECINVDBG_R = crate::R<u8, u8>;
#[doc = "Non-secure non-invasive Debug Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NONSECNONINVDBG_A {
    #[doc = "2: Non-secure non-invasive debug disable"]
    DISABLE,
    #[doc = "3: Non-secure non-invasive debug enable"]
    ENABLE,
}
impl From<NONSECNONINVDBG_A> for u8 {
    #[inline(always)]
    fn from(variant: NONSECNONINVDBG_A) -> Self {
        match variant {
            NONSECNONINVDBG_A::DISABLE => 2,
            NONSECNONINVDBG_A::ENABLE => 3,
        }
    }
}
#[doc = "Reader of field `NONSECNONINVDBG`"]
pub type NONSECNONINVDBG_R = crate::R<u8, NONSECNONINVDBG_A>;
impl NONSECNONINVDBG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NONSECNONINVDBG_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(NONSECNONINVDBG_A::DISABLE),
            3 => Val(NONSECNONINVDBG_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NONSECNONINVDBG_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NONSECNONINVDBG_A::ENABLE
    }
}
#[doc = "Reader of field `SECINVDBG`"]
pub type SECINVDBG_R = crate::R<u8, u8>;
#[doc = "Reader of field `SECNONINVDBG`"]
pub type SECNONINVDBG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Non-secure invasive Debug Status"]
    #[inline(always)]
    pub fn nonsecinvdbg(&self) -> NONSECINVDBG_R {
        NONSECINVDBG_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Non-secure non-invasive Debug Status"]
    #[inline(always)]
    pub fn nonsecnoninvdbg(&self) -> NONSECNONINVDBG_R {
        NONSECNONINVDBG_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Secure invasive Debug Status"]
    #[inline(always)]
    pub fn secinvdbg(&self) -> SECINVDBG_R {
        SECINVDBG_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Secure non-invasive Debug Status"]
    #[inline(always)]
    pub fn secnoninvdbg(&self) -> SECNONINVDBG_R {
        SECNONINVDBG_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
