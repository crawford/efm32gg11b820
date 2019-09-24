#[doc = "Reader of register EXTIPSELL"]
pub type R = crate::R<u32, super::EXTIPSELL>;
#[doc = "Writer for register EXTIPSELL"]
pub type W = crate::W<u32, super::EXTIPSELL>;
#[doc = "Register EXTIPSELL `reset()`'s with value 0"]
impl crate::ResetValue for super::EXTIPSELL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "External Interrupt 0 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL0_A {
    #[doc = "0: Port A group selected for external interrupt 0"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 0"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 0"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 0"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 0"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 0"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 0"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 0"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 0"]
    PORTI,
}
impl From<EXTIPSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL0_A) -> Self {
        match variant {
            EXTIPSEL0_A::PORTA => 0,
            EXTIPSEL0_A::PORTB => 1,
            EXTIPSEL0_A::PORTC => 2,
            EXTIPSEL0_A::PORTD => 3,
            EXTIPSEL0_A::PORTE => 4,
            EXTIPSEL0_A::PORTF => 5,
            EXTIPSEL0_A::PORTG => 6,
            EXTIPSEL0_A::PORTH => 7,
            EXTIPSEL0_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL0`"]
pub type EXTIPSEL0_R = crate::R<u8, EXTIPSEL0_A>;
impl EXTIPSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL0_A::PORTA),
            1 => Val(EXTIPSEL0_A::PORTB),
            2 => Val(EXTIPSEL0_A::PORTC),
            3 => Val(EXTIPSEL0_A::PORTD),
            4 => Val(EXTIPSEL0_A::PORTE),
            5 => Val(EXTIPSEL0_A::PORTF),
            6 => Val(EXTIPSEL0_A::PORTG),
            7 => Val(EXTIPSEL0_A::PORTH),
            8 => Val(EXTIPSEL0_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL0_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL0_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL0_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL0_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL0_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL0_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL0_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL0`"]
pub struct EXTIPSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 0"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 0"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "External Interrupt 1 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL1_A {
    #[doc = "0: Port A group selected for external interrupt 1"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 1"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 1"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 1"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 1"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 1"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 1"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 1"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 1"]
    PORTI,
}
impl From<EXTIPSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL1_A) -> Self {
        match variant {
            EXTIPSEL1_A::PORTA => 0,
            EXTIPSEL1_A::PORTB => 1,
            EXTIPSEL1_A::PORTC => 2,
            EXTIPSEL1_A::PORTD => 3,
            EXTIPSEL1_A::PORTE => 4,
            EXTIPSEL1_A::PORTF => 5,
            EXTIPSEL1_A::PORTG => 6,
            EXTIPSEL1_A::PORTH => 7,
            EXTIPSEL1_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL1`"]
pub type EXTIPSEL1_R = crate::R<u8, EXTIPSEL1_A>;
impl EXTIPSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL1_A::PORTA),
            1 => Val(EXTIPSEL1_A::PORTB),
            2 => Val(EXTIPSEL1_A::PORTC),
            3 => Val(EXTIPSEL1_A::PORTD),
            4 => Val(EXTIPSEL1_A::PORTE),
            5 => Val(EXTIPSEL1_A::PORTF),
            6 => Val(EXTIPSEL1_A::PORTG),
            7 => Val(EXTIPSEL1_A::PORTH),
            8 => Val(EXTIPSEL1_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL1_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL1_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL1_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL1_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL1_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL1_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL1_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL1_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL1_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL1`"]
pub struct EXTIPSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 1"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 1"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "External Interrupt 2 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL2_A {
    #[doc = "0: Port A group selected for external interrupt 2"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 2"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 2"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 2"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 2"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 2"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 2"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 2"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 2"]
    PORTI,
}
impl From<EXTIPSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL2_A) -> Self {
        match variant {
            EXTIPSEL2_A::PORTA => 0,
            EXTIPSEL2_A::PORTB => 1,
            EXTIPSEL2_A::PORTC => 2,
            EXTIPSEL2_A::PORTD => 3,
            EXTIPSEL2_A::PORTE => 4,
            EXTIPSEL2_A::PORTF => 5,
            EXTIPSEL2_A::PORTG => 6,
            EXTIPSEL2_A::PORTH => 7,
            EXTIPSEL2_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL2`"]
pub type EXTIPSEL2_R = crate::R<u8, EXTIPSEL2_A>;
impl EXTIPSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL2_A::PORTA),
            1 => Val(EXTIPSEL2_A::PORTB),
            2 => Val(EXTIPSEL2_A::PORTC),
            3 => Val(EXTIPSEL2_A::PORTD),
            4 => Val(EXTIPSEL2_A::PORTE),
            5 => Val(EXTIPSEL2_A::PORTF),
            6 => Val(EXTIPSEL2_A::PORTG),
            7 => Val(EXTIPSEL2_A::PORTH),
            8 => Val(EXTIPSEL2_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL2_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL2_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL2_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL2_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL2_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL2_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL2_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL2_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL2_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL2`"]
pub struct EXTIPSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 2"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 2"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "External Interrupt 3 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL3_A {
    #[doc = "0: Port A group selected for external interrupt 3"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 3"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 3"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 3"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 3"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 3"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 3"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 3"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 3"]
    PORTI,
}
impl From<EXTIPSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL3_A) -> Self {
        match variant {
            EXTIPSEL3_A::PORTA => 0,
            EXTIPSEL3_A::PORTB => 1,
            EXTIPSEL3_A::PORTC => 2,
            EXTIPSEL3_A::PORTD => 3,
            EXTIPSEL3_A::PORTE => 4,
            EXTIPSEL3_A::PORTF => 5,
            EXTIPSEL3_A::PORTG => 6,
            EXTIPSEL3_A::PORTH => 7,
            EXTIPSEL3_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL3`"]
pub type EXTIPSEL3_R = crate::R<u8, EXTIPSEL3_A>;
impl EXTIPSEL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL3_A::PORTA),
            1 => Val(EXTIPSEL3_A::PORTB),
            2 => Val(EXTIPSEL3_A::PORTC),
            3 => Val(EXTIPSEL3_A::PORTD),
            4 => Val(EXTIPSEL3_A::PORTE),
            5 => Val(EXTIPSEL3_A::PORTF),
            6 => Val(EXTIPSEL3_A::PORTG),
            7 => Val(EXTIPSEL3_A::PORTH),
            8 => Val(EXTIPSEL3_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL3_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL3_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL3_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL3_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL3_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL3_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL3_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL3_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL3_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL3`"]
pub struct EXTIPSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 3"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 3"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "External Interrupt 4 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL4_A {
    #[doc = "0: Port A group selected for external interrupt 4"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 4"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 4"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 4"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 4"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 4"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 4"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 4"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 4"]
    PORTI,
}
impl From<EXTIPSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL4_A) -> Self {
        match variant {
            EXTIPSEL4_A::PORTA => 0,
            EXTIPSEL4_A::PORTB => 1,
            EXTIPSEL4_A::PORTC => 2,
            EXTIPSEL4_A::PORTD => 3,
            EXTIPSEL4_A::PORTE => 4,
            EXTIPSEL4_A::PORTF => 5,
            EXTIPSEL4_A::PORTG => 6,
            EXTIPSEL4_A::PORTH => 7,
            EXTIPSEL4_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL4`"]
pub type EXTIPSEL4_R = crate::R<u8, EXTIPSEL4_A>;
impl EXTIPSEL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL4_A::PORTA),
            1 => Val(EXTIPSEL4_A::PORTB),
            2 => Val(EXTIPSEL4_A::PORTC),
            3 => Val(EXTIPSEL4_A::PORTD),
            4 => Val(EXTIPSEL4_A::PORTE),
            5 => Val(EXTIPSEL4_A::PORTF),
            6 => Val(EXTIPSEL4_A::PORTG),
            7 => Val(EXTIPSEL4_A::PORTH),
            8 => Val(EXTIPSEL4_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL4_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL4_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL4_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL4_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL4_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL4_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL4_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL4_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL4_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL4`"]
pub struct EXTIPSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 4"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 4"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "External Interrupt 5 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL5_A {
    #[doc = "0: Port A group selected for external interrupt 5"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 5"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 5"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 5"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 5"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 5"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 5"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 5"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 5"]
    PORTI,
}
impl From<EXTIPSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL5_A) -> Self {
        match variant {
            EXTIPSEL5_A::PORTA => 0,
            EXTIPSEL5_A::PORTB => 1,
            EXTIPSEL5_A::PORTC => 2,
            EXTIPSEL5_A::PORTD => 3,
            EXTIPSEL5_A::PORTE => 4,
            EXTIPSEL5_A::PORTF => 5,
            EXTIPSEL5_A::PORTG => 6,
            EXTIPSEL5_A::PORTH => 7,
            EXTIPSEL5_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL5`"]
pub type EXTIPSEL5_R = crate::R<u8, EXTIPSEL5_A>;
impl EXTIPSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL5_A::PORTA),
            1 => Val(EXTIPSEL5_A::PORTB),
            2 => Val(EXTIPSEL5_A::PORTC),
            3 => Val(EXTIPSEL5_A::PORTD),
            4 => Val(EXTIPSEL5_A::PORTE),
            5 => Val(EXTIPSEL5_A::PORTF),
            6 => Val(EXTIPSEL5_A::PORTG),
            7 => Val(EXTIPSEL5_A::PORTH),
            8 => Val(EXTIPSEL5_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL5_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL5_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL5_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL5_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL5_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL5_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL5_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL5_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL5_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL5`"]
pub struct EXTIPSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 5"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 5"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "External Interrupt 6 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL6_A {
    #[doc = "0: Port A group selected for external interrupt 6"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 6"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 6"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 6"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 6"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 6"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 6"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 6"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 6"]
    PORTI,
}
impl From<EXTIPSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL6_A) -> Self {
        match variant {
            EXTIPSEL6_A::PORTA => 0,
            EXTIPSEL6_A::PORTB => 1,
            EXTIPSEL6_A::PORTC => 2,
            EXTIPSEL6_A::PORTD => 3,
            EXTIPSEL6_A::PORTE => 4,
            EXTIPSEL6_A::PORTF => 5,
            EXTIPSEL6_A::PORTG => 6,
            EXTIPSEL6_A::PORTH => 7,
            EXTIPSEL6_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL6`"]
pub type EXTIPSEL6_R = crate::R<u8, EXTIPSEL6_A>;
impl EXTIPSEL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL6_A::PORTA),
            1 => Val(EXTIPSEL6_A::PORTB),
            2 => Val(EXTIPSEL6_A::PORTC),
            3 => Val(EXTIPSEL6_A::PORTD),
            4 => Val(EXTIPSEL6_A::PORTE),
            5 => Val(EXTIPSEL6_A::PORTF),
            6 => Val(EXTIPSEL6_A::PORTG),
            7 => Val(EXTIPSEL6_A::PORTH),
            8 => Val(EXTIPSEL6_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL6_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL6_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL6_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL6_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL6_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL6_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL6_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL6_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL6_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL6`"]
pub struct EXTIPSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 6"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 6"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "External Interrupt 7 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL7_A {
    #[doc = "0: Port A group selected for external interrupt 7"]
    PORTA,
    #[doc = "1: Port B group selected for external interrupt 7"]
    PORTB,
    #[doc = "2: Port C group selected for external interrupt 7"]
    PORTC,
    #[doc = "3: Port D group selected for external interrupt 7"]
    PORTD,
    #[doc = "4: Port E group selected for external interrupt 7"]
    PORTE,
    #[doc = "5: Port F group selected for external interrupt 7"]
    PORTF,
    #[doc = "6: Port G group selected for external interrupt 7"]
    PORTG,
    #[doc = "7: Port H group selected for external interrupt 7"]
    PORTH,
    #[doc = "8: Port I group selected for external interrupt 7"]
    PORTI,
}
impl From<EXTIPSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL7_A) -> Self {
        match variant {
            EXTIPSEL7_A::PORTA => 0,
            EXTIPSEL7_A::PORTB => 1,
            EXTIPSEL7_A::PORTC => 2,
            EXTIPSEL7_A::PORTD => 3,
            EXTIPSEL7_A::PORTE => 4,
            EXTIPSEL7_A::PORTF => 5,
            EXTIPSEL7_A::PORTG => 6,
            EXTIPSEL7_A::PORTH => 7,
            EXTIPSEL7_A::PORTI => 8,
        }
    }
}
#[doc = "Reader of field `EXTIPSEL7`"]
pub type EXTIPSEL7_R = crate::R<u8, EXTIPSEL7_A>;
impl EXTIPSEL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EXTIPSEL7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EXTIPSEL7_A::PORTA),
            1 => Val(EXTIPSEL7_A::PORTB),
            2 => Val(EXTIPSEL7_A::PORTC),
            3 => Val(EXTIPSEL7_A::PORTD),
            4 => Val(EXTIPSEL7_A::PORTE),
            5 => Val(EXTIPSEL7_A::PORTF),
            6 => Val(EXTIPSEL7_A::PORTG),
            7 => Val(EXTIPSEL7_A::PORTH),
            8 => Val(EXTIPSEL7_A::PORTI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL7_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL7_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL7_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL7_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL7_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL7_A::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline(always)]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL7_A::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline(always)]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL7_A::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline(always)]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL7_A::PORTI
    }
}
#[doc = "Write proxy for field `EXTIPSEL7`"]
pub struct EXTIPSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 7"]
    #[inline(always)]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 7"]
    #[inline(always)]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> EXTIPSEL0_R {
        EXTIPSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> EXTIPSEL1_R {
        EXTIPSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> EXTIPSEL2_R {
        EXTIPSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> EXTIPSEL3_R {
        EXTIPSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> EXTIPSEL4_R {
        EXTIPSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> EXTIPSEL5_R {
        EXTIPSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> EXTIPSEL6_R {
        EXTIPSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> EXTIPSEL7_R {
        EXTIPSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&mut self) -> EXTIPSEL0_W {
        EXTIPSEL0_W { w: self }
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&mut self) -> EXTIPSEL1_W {
        EXTIPSEL1_W { w: self }
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&mut self) -> EXTIPSEL2_W {
        EXTIPSEL2_W { w: self }
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&mut self) -> EXTIPSEL3_W {
        EXTIPSEL3_W { w: self }
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&mut self) -> EXTIPSEL4_W {
        EXTIPSEL4_W { w: self }
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&mut self) -> EXTIPSEL5_W {
        EXTIPSEL5_W { w: self }
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&mut self) -> EXTIPSEL6_W {
        EXTIPSEL6_W { w: self }
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&mut self) -> EXTIPSEL7_W {
        EXTIPSEL7_W { w: self }
    }
}
