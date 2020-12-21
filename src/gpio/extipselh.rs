#[doc = "Reader of register EXTIPSELH"]
pub type R = crate::R<u32, super::EXTIPSELH>;
#[doc = "Writer for register EXTIPSELH"]
pub type W = crate::W<u32, super::EXTIPSELH>;
#[doc = "Register EXTIPSELH `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTIPSELH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Interrupt 8 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL8_A {
    #[doc = "0: Port A group selected for external interrupt 8"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 8"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 8"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 8"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 8"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 8"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 8"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 8"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 8"]
    PORTI = 8,
}
impl From<EXTIPSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL8`"]
pub type EXTIPSEL8_R = crate::R<u8, EXTIPSEL8_A>;
impl EXTIPSEL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL8_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL8_A::PORTA),
            1 => Val(EXTIPSEL8_A::PORTB),
            2 => Val(EXTIPSEL8_A::PORTC),
            3 => Val(EXTIPSEL8_A::PORTD),
            4 => Val(EXTIPSEL8_A::PORTE),
            5 => Val(EXTIPSEL8_A::PORTF),
            6 => Val(EXTIPSEL8_A::PORTG),
            7 => Val(EXTIPSEL8_A::PORTH),
            8 => Val(EXTIPSEL8_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL8_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL8_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL8_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL8_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL8_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL8_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL8_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL8_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL8_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL8`"]
pub struct EXTIPSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL8_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 8"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 8"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 8"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 8"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 8"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL8_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "External Interrupt 9 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL9_A {
    #[doc = "0: Port A group selected for external interrupt 9"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 9"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 9"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 9"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 9"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 9"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 9"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 9"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 9"]
    PORTI = 8,
}
impl From<EXTIPSEL9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL9`"]
pub type EXTIPSEL9_R = crate::R<u8, EXTIPSEL9_A>;
impl EXTIPSEL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL9_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL9_A::PORTA),
            1 => Val(EXTIPSEL9_A::PORTB),
            2 => Val(EXTIPSEL9_A::PORTC),
            3 => Val(EXTIPSEL9_A::PORTD),
            4 => Val(EXTIPSEL9_A::PORTE),
            5 => Val(EXTIPSEL9_A::PORTF),
            6 => Val(EXTIPSEL9_A::PORTG),
            7 => Val(EXTIPSEL9_A::PORTH),
            8 => Val(EXTIPSEL9_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL9_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL9_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL9_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL9_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL9_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL9_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL9_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL9_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL9_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL9`"]
pub struct EXTIPSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL9_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 9"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 9"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 9"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 9"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 9"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL9_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "External Interrupt 10 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL10_A {
    #[doc = "0: Port A group selected for external interrupt 10"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 10"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 10"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 10"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 10"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 10"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 10"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 10"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 10"]
    PORTI = 8,
}
impl From<EXTIPSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL10`"]
pub type EXTIPSEL10_R = crate::R<u8, EXTIPSEL10_A>;
impl EXTIPSEL10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL10_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL10_A::PORTA),
            1 => Val(EXTIPSEL10_A::PORTB),
            2 => Val(EXTIPSEL10_A::PORTC),
            3 => Val(EXTIPSEL10_A::PORTD),
            4 => Val(EXTIPSEL10_A::PORTE),
            5 => Val(EXTIPSEL10_A::PORTF),
            6 => Val(EXTIPSEL10_A::PORTG),
            7 => Val(EXTIPSEL10_A::PORTH),
            8 => Val(EXTIPSEL10_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL10_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL10_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL10_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL10_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL10_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL10_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL10_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL10_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL10_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL10`"]
pub struct EXTIPSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL10_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 10"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 10"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 10"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 10"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 10"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL10_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "External Interrupt 11 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL11_A {
    #[doc = "0: Port A group selected for external interrupt 11"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 11"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 11"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 11"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 11"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 11"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 11"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 11"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 11"]
    PORTI = 8,
}
impl From<EXTIPSEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL11`"]
pub type EXTIPSEL11_R = crate::R<u8, EXTIPSEL11_A>;
impl EXTIPSEL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL11_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL11_A::PORTA),
            1 => Val(EXTIPSEL11_A::PORTB),
            2 => Val(EXTIPSEL11_A::PORTC),
            3 => Val(EXTIPSEL11_A::PORTD),
            4 => Val(EXTIPSEL11_A::PORTE),
            5 => Val(EXTIPSEL11_A::PORTF),
            6 => Val(EXTIPSEL11_A::PORTG),
            7 => Val(EXTIPSEL11_A::PORTH),
            8 => Val(EXTIPSEL11_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL11_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL11_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL11_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL11_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL11_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL11_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL11_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL11_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL11_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL11`"]
pub struct EXTIPSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL11_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 11"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 11"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 11"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 11"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 11"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL11_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "External Interrupt 12 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL12_A {
    #[doc = "0: Port A group selected for external interrupt 12"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 12"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 12"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 12"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 12"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 12"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 12"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 12"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 12"]
    PORTI = 8,
}
impl From<EXTIPSEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL12`"]
pub type EXTIPSEL12_R = crate::R<u8, EXTIPSEL12_A>;
impl EXTIPSEL12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL12_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL12_A::PORTA),
            1 => Val(EXTIPSEL12_A::PORTB),
            2 => Val(EXTIPSEL12_A::PORTC),
            3 => Val(EXTIPSEL12_A::PORTD),
            4 => Val(EXTIPSEL12_A::PORTE),
            5 => Val(EXTIPSEL12_A::PORTF),
            6 => Val(EXTIPSEL12_A::PORTG),
            7 => Val(EXTIPSEL12_A::PORTH),
            8 => Val(EXTIPSEL12_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL12_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL12_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL12_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL12_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL12_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL12_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL12_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL12_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL12_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL12`"]
pub struct EXTIPSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL12_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 12"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 12"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 12"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 12"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 12"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL12_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "External Interrupt 13 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL13_A {
    #[doc = "0: Port A group selected for external interrupt 13"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 13"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 13"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 13"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 13"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 13"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 13"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 13"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 13"]
    PORTI = 8,
}
impl From<EXTIPSEL13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL13`"]
pub type EXTIPSEL13_R = crate::R<u8, EXTIPSEL13_A>;
impl EXTIPSEL13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL13_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL13_A::PORTA),
            1 => Val(EXTIPSEL13_A::PORTB),
            2 => Val(EXTIPSEL13_A::PORTC),
            3 => Val(EXTIPSEL13_A::PORTD),
            4 => Val(EXTIPSEL13_A::PORTE),
            5 => Val(EXTIPSEL13_A::PORTF),
            6 => Val(EXTIPSEL13_A::PORTG),
            7 => Val(EXTIPSEL13_A::PORTH),
            8 => Val(EXTIPSEL13_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL13_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL13_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL13_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL13_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL13_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL13_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL13_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL13_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL13_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL13`"]
pub struct EXTIPSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL13_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 13"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 13"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 13"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 13"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 13"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL13_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "External Interrupt 14 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL14_A {
    #[doc = "0: Port A group selected for external interrupt 14"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 14"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 14"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 14"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 14"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 14"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 14"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 14"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 14"]
    PORTI = 8,
}
impl From<EXTIPSEL14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL14`"]
pub type EXTIPSEL14_R = crate::R<u8, EXTIPSEL14_A>;
impl EXTIPSEL14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL14_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL14_A::PORTA),
            1 => Val(EXTIPSEL14_A::PORTB),
            2 => Val(EXTIPSEL14_A::PORTC),
            3 => Val(EXTIPSEL14_A::PORTD),
            4 => Val(EXTIPSEL14_A::PORTE),
            5 => Val(EXTIPSEL14_A::PORTF),
            6 => Val(EXTIPSEL14_A::PORTG),
            7 => Val(EXTIPSEL14_A::PORTH),
            8 => Val(EXTIPSEL14_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL14_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL14_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL14_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL14_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL14_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL14_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL14_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL14_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL14_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL14`"]
pub struct EXTIPSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL14_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 14"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 14"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 14"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 14"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 14"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL14_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "External Interrupt 15 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL15_A {
    #[doc = "0: Port A group selected for external interrupt 15"]
    PORTA = 0,
    #[doc = "1: Port B group selected for external interrupt 15"]
    PORTB = 1,
    #[doc = "2: Port C group selected for external interrupt 15"]
    PORTC = 2,
    #[doc = "3: Port D group selected for external interrupt 15"]
    PORTD = 3,
    #[doc = "4: Port E group selected for external interrupt 15"]
    PORTE = 4,
    #[doc = "5: Port F group selected for external interrupt 15"]
    PORTF = 5,
    #[doc = "6: Port G group selected for external interrupt 15"]
    PORTG = 6,
    #[doc = "7: Port H group selected for external interrupt 15"]
    PORTH = 7,
    #[doc = "8: Port I group selected for external interrupt 15"]
    PORTI = 8,
}
impl From<EXTIPSEL15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPSEL15`"]
pub type EXTIPSEL15_R = crate::R<u8, EXTIPSEL15_A>;
impl EXTIPSEL15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL15_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL15_A::PORTA),
            1 => Val(EXTIPSEL15_A::PORTB),
            2 => Val(EXTIPSEL15_A::PORTC),
            3 => Val(EXTIPSEL15_A::PORTD),
            4 => Val(EXTIPSEL15_A::PORTE),
            5 => Val(EXTIPSEL15_A::PORTF),
            6 => Val(EXTIPSEL15_A::PORTG),
            7 => Val(EXTIPSEL15_A::PORTH),
            8 => Val(EXTIPSEL15_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL15_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL15_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL15_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL15_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL15_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL15_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL15_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL15_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL15_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL15`"]
pub struct EXTIPSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL15_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 15"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 15"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 15"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 15"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 15"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL15_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - External Interrupt 8 Port Select"]
    #[inline(always)]
    pub fn extipsel8(&self) -> EXTIPSEL8_R {
        EXTIPSEL8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Interrupt 9 Port Select"]
    #[inline(always)]
    pub fn extipsel9(&self) -> EXTIPSEL9_R {
        EXTIPSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Interrupt 10 Port Select"]
    #[inline(always)]
    pub fn extipsel10(&self) -> EXTIPSEL10_R {
        EXTIPSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External Interrupt 11 Port Select"]
    #[inline(always)]
    pub fn extipsel11(&self) -> EXTIPSEL11_R {
        EXTIPSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Interrupt 12 Port Select"]
    #[inline(always)]
    pub fn extipsel12(&self) -> EXTIPSEL12_R {
        EXTIPSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External Interrupt 13 Port Select"]
    #[inline(always)]
    pub fn extipsel13(&self) -> EXTIPSEL13_R {
        EXTIPSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External Interrupt 14 Port Select"]
    #[inline(always)]
    pub fn extipsel14(&self) -> EXTIPSEL14_R {
        EXTIPSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External Interrupt 15 Port Select"]
    #[inline(always)]
    pub fn extipsel15(&self) -> EXTIPSEL15_R {
        EXTIPSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Interrupt 8 Port Select"]
    #[inline(always)]
    pub fn extipsel8(&mut self) -> EXTIPSEL8_W {
        EXTIPSEL8_W { w: self }
    }
    #[doc = "Bits 4:7 - External Interrupt 9 Port Select"]
    #[inline(always)]
    pub fn extipsel9(&mut self) -> EXTIPSEL9_W {
        EXTIPSEL9_W { w: self }
    }
    #[doc = "Bits 8:11 - External Interrupt 10 Port Select"]
    #[inline(always)]
    pub fn extipsel10(&mut self) -> EXTIPSEL10_W {
        EXTIPSEL10_W { w: self }
    }
    #[doc = "Bits 12:15 - External Interrupt 11 Port Select"]
    #[inline(always)]
    pub fn extipsel11(&mut self) -> EXTIPSEL11_W {
        EXTIPSEL11_W { w: self }
    }
    #[doc = "Bits 16:19 - External Interrupt 12 Port Select"]
    #[inline(always)]
    pub fn extipsel12(&mut self) -> EXTIPSEL12_W {
        EXTIPSEL12_W { w: self }
    }
    #[doc = "Bits 20:23 - External Interrupt 13 Port Select"]
    #[inline(always)]
    pub fn extipsel13(&mut self) -> EXTIPSEL13_W {
        EXTIPSEL13_W { w: self }
    }
    #[doc = "Bits 24:27 - External Interrupt 14 Port Select"]
    #[inline(always)]
    pub fn extipsel14(&mut self) -> EXTIPSEL14_W {
        EXTIPSEL14_W { w: self }
    }
    #[doc = "Bits 28:31 - External Interrupt 15 Port Select"]
    #[inline(always)]
    pub fn extipsel15(&mut self) -> EXTIPSEL15_W {
        EXTIPSEL15_W { w: self }
    }
}
