#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EXTIPSELL {
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
#[doc = "Possible values of the field `EXTIPSEL0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL0R {
    #[doc = "Port A group selected for external interrupt 0"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 0"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 0"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 0"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 0"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 0"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 0"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 0"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 0"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL0R::PORTA => 0,
            EXTIPSEL0R::PORTB => 1,
            EXTIPSEL0R::PORTC => 2,
            EXTIPSEL0R::PORTD => 3,
            EXTIPSEL0R::PORTE => 4,
            EXTIPSEL0R::PORTF => 5,
            EXTIPSEL0R::PORTG => 6,
            EXTIPSEL0R::PORTH => 7,
            EXTIPSEL0R::PORTI => 8,
            EXTIPSEL0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL0R {
        match value {
            0 => EXTIPSEL0R::PORTA,
            1 => EXTIPSEL0R::PORTB,
            2 => EXTIPSEL0R::PORTC,
            3 => EXTIPSEL0R::PORTD,
            4 => EXTIPSEL0R::PORTE,
            5 => EXTIPSEL0R::PORTF,
            6 => EXTIPSEL0R::PORTG,
            7 => EXTIPSEL0R::PORTH,
            8 => EXTIPSEL0R::PORTI,
            i => EXTIPSEL0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL0R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL0R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL0R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL0R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL0R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL0R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL0R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL0R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL0R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL1R {
    #[doc = "Port A group selected for external interrupt 1"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 1"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 1"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 1"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 1"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 1"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 1"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 1"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 1"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL1R::PORTA => 0,
            EXTIPSEL1R::PORTB => 1,
            EXTIPSEL1R::PORTC => 2,
            EXTIPSEL1R::PORTD => 3,
            EXTIPSEL1R::PORTE => 4,
            EXTIPSEL1R::PORTF => 5,
            EXTIPSEL1R::PORTG => 6,
            EXTIPSEL1R::PORTH => 7,
            EXTIPSEL1R::PORTI => 8,
            EXTIPSEL1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL1R {
        match value {
            0 => EXTIPSEL1R::PORTA,
            1 => EXTIPSEL1R::PORTB,
            2 => EXTIPSEL1R::PORTC,
            3 => EXTIPSEL1R::PORTD,
            4 => EXTIPSEL1R::PORTE,
            5 => EXTIPSEL1R::PORTF,
            6 => EXTIPSEL1R::PORTG,
            7 => EXTIPSEL1R::PORTH,
            8 => EXTIPSEL1R::PORTI,
            i => EXTIPSEL1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL1R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL1R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL1R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL1R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL1R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL1R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL1R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL1R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL1R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL2R {
    #[doc = "Port A group selected for external interrupt 2"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 2"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 2"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 2"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 2"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 2"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 2"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 2"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 2"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL2R::PORTA => 0,
            EXTIPSEL2R::PORTB => 1,
            EXTIPSEL2R::PORTC => 2,
            EXTIPSEL2R::PORTD => 3,
            EXTIPSEL2R::PORTE => 4,
            EXTIPSEL2R::PORTF => 5,
            EXTIPSEL2R::PORTG => 6,
            EXTIPSEL2R::PORTH => 7,
            EXTIPSEL2R::PORTI => 8,
            EXTIPSEL2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL2R {
        match value {
            0 => EXTIPSEL2R::PORTA,
            1 => EXTIPSEL2R::PORTB,
            2 => EXTIPSEL2R::PORTC,
            3 => EXTIPSEL2R::PORTD,
            4 => EXTIPSEL2R::PORTE,
            5 => EXTIPSEL2R::PORTF,
            6 => EXTIPSEL2R::PORTG,
            7 => EXTIPSEL2R::PORTH,
            8 => EXTIPSEL2R::PORTI,
            i => EXTIPSEL2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL2R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL2R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL2R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL2R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL2R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL2R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL2R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL2R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL2R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL3R {
    #[doc = "Port A group selected for external interrupt 3"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 3"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 3"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 3"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 3"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 3"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 3"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 3"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 3"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL3R::PORTA => 0,
            EXTIPSEL3R::PORTB => 1,
            EXTIPSEL3R::PORTC => 2,
            EXTIPSEL3R::PORTD => 3,
            EXTIPSEL3R::PORTE => 4,
            EXTIPSEL3R::PORTF => 5,
            EXTIPSEL3R::PORTG => 6,
            EXTIPSEL3R::PORTH => 7,
            EXTIPSEL3R::PORTI => 8,
            EXTIPSEL3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL3R {
        match value {
            0 => EXTIPSEL3R::PORTA,
            1 => EXTIPSEL3R::PORTB,
            2 => EXTIPSEL3R::PORTC,
            3 => EXTIPSEL3R::PORTD,
            4 => EXTIPSEL3R::PORTE,
            5 => EXTIPSEL3R::PORTF,
            6 => EXTIPSEL3R::PORTG,
            7 => EXTIPSEL3R::PORTH,
            8 => EXTIPSEL3R::PORTI,
            i => EXTIPSEL3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL3R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL3R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL3R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL3R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL3R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL3R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL3R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL3R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL3R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL4R {
    #[doc = "Port A group selected for external interrupt 4"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 4"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 4"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 4"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 4"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 4"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 4"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 4"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 4"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL4R::PORTA => 0,
            EXTIPSEL4R::PORTB => 1,
            EXTIPSEL4R::PORTC => 2,
            EXTIPSEL4R::PORTD => 3,
            EXTIPSEL4R::PORTE => 4,
            EXTIPSEL4R::PORTF => 5,
            EXTIPSEL4R::PORTG => 6,
            EXTIPSEL4R::PORTH => 7,
            EXTIPSEL4R::PORTI => 8,
            EXTIPSEL4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL4R {
        match value {
            0 => EXTIPSEL4R::PORTA,
            1 => EXTIPSEL4R::PORTB,
            2 => EXTIPSEL4R::PORTC,
            3 => EXTIPSEL4R::PORTD,
            4 => EXTIPSEL4R::PORTE,
            5 => EXTIPSEL4R::PORTF,
            6 => EXTIPSEL4R::PORTG,
            7 => EXTIPSEL4R::PORTH,
            8 => EXTIPSEL4R::PORTI,
            i => EXTIPSEL4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL4R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL4R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL4R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL4R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL4R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL4R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL4R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL4R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL4R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL5R {
    #[doc = "Port A group selected for external interrupt 5"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 5"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 5"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 5"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 5"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 5"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 5"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 5"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 5"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL5R::PORTA => 0,
            EXTIPSEL5R::PORTB => 1,
            EXTIPSEL5R::PORTC => 2,
            EXTIPSEL5R::PORTD => 3,
            EXTIPSEL5R::PORTE => 4,
            EXTIPSEL5R::PORTF => 5,
            EXTIPSEL5R::PORTG => 6,
            EXTIPSEL5R::PORTH => 7,
            EXTIPSEL5R::PORTI => 8,
            EXTIPSEL5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL5R {
        match value {
            0 => EXTIPSEL5R::PORTA,
            1 => EXTIPSEL5R::PORTB,
            2 => EXTIPSEL5R::PORTC,
            3 => EXTIPSEL5R::PORTD,
            4 => EXTIPSEL5R::PORTE,
            5 => EXTIPSEL5R::PORTF,
            6 => EXTIPSEL5R::PORTG,
            7 => EXTIPSEL5R::PORTH,
            8 => EXTIPSEL5R::PORTI,
            i => EXTIPSEL5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL5R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL5R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL5R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL5R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL5R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL5R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL5R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL5R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL5R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL6R {
    #[doc = "Port A group selected for external interrupt 6"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 6"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 6"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 6"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 6"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 6"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 6"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 6"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 6"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL6R::PORTA => 0,
            EXTIPSEL6R::PORTB => 1,
            EXTIPSEL6R::PORTC => 2,
            EXTIPSEL6R::PORTD => 3,
            EXTIPSEL6R::PORTE => 4,
            EXTIPSEL6R::PORTF => 5,
            EXTIPSEL6R::PORTG => 6,
            EXTIPSEL6R::PORTH => 7,
            EXTIPSEL6R::PORTI => 8,
            EXTIPSEL6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL6R {
        match value {
            0 => EXTIPSEL6R::PORTA,
            1 => EXTIPSEL6R::PORTB,
            2 => EXTIPSEL6R::PORTC,
            3 => EXTIPSEL6R::PORTD,
            4 => EXTIPSEL6R::PORTE,
            5 => EXTIPSEL6R::PORTF,
            6 => EXTIPSEL6R::PORTG,
            7 => EXTIPSEL6R::PORTH,
            8 => EXTIPSEL6R::PORTI,
            i => EXTIPSEL6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL6R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL6R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL6R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL6R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL6R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL6R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL6R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL6R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL6R::PORTI
    }
}
#[doc = "Possible values of the field `EXTIPSEL7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPSEL7R {
    #[doc = "Port A group selected for external interrupt 7"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 7"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 7"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 7"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 7"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 7"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 7"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 7"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 7"]
    PORTI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EXTIPSEL7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EXTIPSEL7R::PORTA => 0,
            EXTIPSEL7R::PORTB => 1,
            EXTIPSEL7R::PORTC => 2,
            EXTIPSEL7R::PORTD => 3,
            EXTIPSEL7R::PORTE => 4,
            EXTIPSEL7R::PORTF => 5,
            EXTIPSEL7R::PORTG => 6,
            EXTIPSEL7R::PORTH => 7,
            EXTIPSEL7R::PORTI => 8,
            EXTIPSEL7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EXTIPSEL7R {
        match value {
            0 => EXTIPSEL7R::PORTA,
            1 => EXTIPSEL7R::PORTB,
            2 => EXTIPSEL7R::PORTC,
            3 => EXTIPSEL7R::PORTD,
            4 => EXTIPSEL7R::PORTE,
            5 => EXTIPSEL7R::PORTF,
            6 => EXTIPSEL7R::PORTG,
            7 => EXTIPSEL7R::PORTH,
            8 => EXTIPSEL7R::PORTI,
            i => EXTIPSEL7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline]
    pub fn is_porta(&self) -> bool {
        *self == EXTIPSEL7R::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline]
    pub fn is_portb(&self) -> bool {
        *self == EXTIPSEL7R::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline]
    pub fn is_portc(&self) -> bool {
        *self == EXTIPSEL7R::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline]
    pub fn is_portd(&self) -> bool {
        *self == EXTIPSEL7R::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline]
    pub fn is_porte(&self) -> bool {
        *self == EXTIPSEL7R::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline]
    pub fn is_portf(&self) -> bool {
        *self == EXTIPSEL7R::PORTF
    }
    #[doc = "Checks if the value of the field is `PORTG`"]
    #[inline]
    pub fn is_portg(&self) -> bool {
        *self == EXTIPSEL7R::PORTG
    }
    #[doc = "Checks if the value of the field is `PORTH`"]
    #[inline]
    pub fn is_porth(&self) -> bool {
        *self == EXTIPSEL7R::PORTH
    }
    #[doc = "Checks if the value of the field is `PORTI`"]
    #[inline]
    pub fn is_porti(&self) -> bool {
        *self == EXTIPSEL7R::PORTI
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL0`"]
pub enum EXTIPSEL0W {
    #[doc = "Port A group selected for external interrupt 0"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 0"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 0"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 0"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 0"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 0"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 0"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 0"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 0"]
    PORTI,
}
impl EXTIPSEL0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL0W::PORTA => 0,
            EXTIPSEL0W::PORTB => 1,
            EXTIPSEL0W::PORTC => 2,
            EXTIPSEL0W::PORTD => 3,
            EXTIPSEL0W::PORTE => 4,
            EXTIPSEL0W::PORTF => 5,
            EXTIPSEL0W::PORTG => 6,
            EXTIPSEL0W::PORTH => 7,
            EXTIPSEL0W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL0W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 0"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 0"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 0"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 0"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 0"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 0"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 0"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 0"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 0"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL0W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL1`"]
pub enum EXTIPSEL1W {
    #[doc = "Port A group selected for external interrupt 1"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 1"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 1"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 1"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 1"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 1"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 1"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 1"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 1"]
    PORTI,
}
impl EXTIPSEL1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL1W::PORTA => 0,
            EXTIPSEL1W::PORTB => 1,
            EXTIPSEL1W::PORTC => 2,
            EXTIPSEL1W::PORTD => 3,
            EXTIPSEL1W::PORTE => 4,
            EXTIPSEL1W::PORTF => 5,
            EXTIPSEL1W::PORTG => 6,
            EXTIPSEL1W::PORTH => 7,
            EXTIPSEL1W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL1W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 1"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 1"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 1"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 1"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 1"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 1"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 1"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 1"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 1"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL1W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL2`"]
pub enum EXTIPSEL2W {
    #[doc = "Port A group selected for external interrupt 2"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 2"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 2"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 2"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 2"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 2"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 2"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 2"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 2"]
    PORTI,
}
impl EXTIPSEL2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL2W::PORTA => 0,
            EXTIPSEL2W::PORTB => 1,
            EXTIPSEL2W::PORTC => 2,
            EXTIPSEL2W::PORTD => 3,
            EXTIPSEL2W::PORTE => 4,
            EXTIPSEL2W::PORTF => 5,
            EXTIPSEL2W::PORTG => 6,
            EXTIPSEL2W::PORTH => 7,
            EXTIPSEL2W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL2W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 2"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 2"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 2"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 2"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 2"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 2"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 2"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 2"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 2"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL2W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL3`"]
pub enum EXTIPSEL3W {
    #[doc = "Port A group selected for external interrupt 3"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 3"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 3"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 3"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 3"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 3"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 3"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 3"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 3"]
    PORTI,
}
impl EXTIPSEL3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL3W::PORTA => 0,
            EXTIPSEL3W::PORTB => 1,
            EXTIPSEL3W::PORTC => 2,
            EXTIPSEL3W::PORTD => 3,
            EXTIPSEL3W::PORTE => 4,
            EXTIPSEL3W::PORTF => 5,
            EXTIPSEL3W::PORTG => 6,
            EXTIPSEL3W::PORTH => 7,
            EXTIPSEL3W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL3W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 3"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 3"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 3"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 3"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 3"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 3"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 3"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 3"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 3"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL3W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL4`"]
pub enum EXTIPSEL4W {
    #[doc = "Port A group selected for external interrupt 4"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 4"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 4"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 4"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 4"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 4"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 4"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 4"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 4"]
    PORTI,
}
impl EXTIPSEL4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL4W::PORTA => 0,
            EXTIPSEL4W::PORTB => 1,
            EXTIPSEL4W::PORTC => 2,
            EXTIPSEL4W::PORTD => 3,
            EXTIPSEL4W::PORTE => 4,
            EXTIPSEL4W::PORTF => 5,
            EXTIPSEL4W::PORTG => 6,
            EXTIPSEL4W::PORTH => 7,
            EXTIPSEL4W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 4"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 4"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 4"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 4"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 4"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 4"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 4"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 4"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 4"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL4W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL5`"]
pub enum EXTIPSEL5W {
    #[doc = "Port A group selected for external interrupt 5"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 5"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 5"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 5"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 5"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 5"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 5"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 5"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 5"]
    PORTI,
}
impl EXTIPSEL5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL5W::PORTA => 0,
            EXTIPSEL5W::PORTB => 1,
            EXTIPSEL5W::PORTC => 2,
            EXTIPSEL5W::PORTD => 3,
            EXTIPSEL5W::PORTE => 4,
            EXTIPSEL5W::PORTF => 5,
            EXTIPSEL5W::PORTG => 6,
            EXTIPSEL5W::PORTH => 7,
            EXTIPSEL5W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL5W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 5"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 5"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 5"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 5"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 5"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 5"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 5"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 5"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 5"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL5W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL6`"]
pub enum EXTIPSEL6W {
    #[doc = "Port A group selected for external interrupt 6"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 6"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 6"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 6"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 6"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 6"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 6"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 6"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 6"]
    PORTI,
}
impl EXTIPSEL6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL6W::PORTA => 0,
            EXTIPSEL6W::PORTB => 1,
            EXTIPSEL6W::PORTC => 2,
            EXTIPSEL6W::PORTD => 3,
            EXTIPSEL6W::PORTE => 4,
            EXTIPSEL6W::PORTF => 5,
            EXTIPSEL6W::PORTG => 6,
            EXTIPSEL6W::PORTH => 7,
            EXTIPSEL6W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL6W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 6"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 6"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 6"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 6"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 6"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 6"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 6"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 6"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 6"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL6W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `EXTIPSEL7`"]
pub enum EXTIPSEL7W {
    #[doc = "Port A group selected for external interrupt 7"]
    PORTA,
    #[doc = "Port B group selected for external interrupt 7"]
    PORTB,
    #[doc = "Port C group selected for external interrupt 7"]
    PORTC,
    #[doc = "Port D group selected for external interrupt 7"]
    PORTD,
    #[doc = "Port E group selected for external interrupt 7"]
    PORTE,
    #[doc = "Port F group selected for external interrupt 7"]
    PORTF,
    #[doc = "Port G group selected for external interrupt 7"]
    PORTG,
    #[doc = "Port H group selected for external interrupt 7"]
    PORTH,
    #[doc = "Port I group selected for external interrupt 7"]
    PORTI,
}
impl EXTIPSEL7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EXTIPSEL7W::PORTA => 0,
            EXTIPSEL7W::PORTB => 1,
            EXTIPSEL7W::PORTC => 2,
            EXTIPSEL7W::PORTD => 3,
            EXTIPSEL7W::PORTE => 4,
            EXTIPSEL7W::PORTF => 5,
            EXTIPSEL7W::PORTG => 6,
            EXTIPSEL7W::PORTH => 7,
            EXTIPSEL7W::PORTI => 8,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EXTIPSEL7W<'a> {
    w: &'a mut W,
}
impl<'a> _EXTIPSEL7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EXTIPSEL7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Port A group selected for external interrupt 7"]
    #[inline]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTA)
    }
    #[doc = "Port B group selected for external interrupt 7"]
    #[inline]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTB)
    }
    #[doc = "Port C group selected for external interrupt 7"]
    #[inline]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTC)
    }
    #[doc = "Port D group selected for external interrupt 7"]
    #[inline]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTD)
    }
    #[doc = "Port E group selected for external interrupt 7"]
    #[inline]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTE)
    }
    #[doc = "Port F group selected for external interrupt 7"]
    #[inline]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTF)
    }
    #[doc = "Port G group selected for external interrupt 7"]
    #[inline]
    pub fn portg(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTG)
    }
    #[doc = "Port H group selected for external interrupt 7"]
    #[inline]
    pub fn porth(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTH)
    }
    #[doc = "Port I group selected for external interrupt 7"]
    #[inline]
    pub fn porti(self) -> &'a mut W {
        self.variant(EXTIPSEL7W::PORTI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
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
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline]
    pub fn extipsel0(&self) -> EXTIPSEL0R {
        EXTIPSEL0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline]
    pub fn extipsel1(&self) -> EXTIPSEL1R {
        EXTIPSEL1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline]
    pub fn extipsel2(&self) -> EXTIPSEL2R {
        EXTIPSEL2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline]
    pub fn extipsel3(&self) -> EXTIPSEL3R {
        EXTIPSEL3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline]
    pub fn extipsel4(&self) -> EXTIPSEL4R {
        EXTIPSEL4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline]
    pub fn extipsel5(&self) -> EXTIPSEL5R {
        EXTIPSEL5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline]
    pub fn extipsel6(&self) -> EXTIPSEL6R {
        EXTIPSEL6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline]
    pub fn extipsel7(&self) -> EXTIPSEL7R {
        EXTIPSEL7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - External Interrupt 0 Port Select"]
    #[inline]
    pub fn extipsel0(&mut self) -> _EXTIPSEL0W {
        _EXTIPSEL0W { w: self }
    }
    #[doc = "Bits 4:7 - External Interrupt 1 Port Select"]
    #[inline]
    pub fn extipsel1(&mut self) -> _EXTIPSEL1W {
        _EXTIPSEL1W { w: self }
    }
    #[doc = "Bits 8:11 - External Interrupt 2 Port Select"]
    #[inline]
    pub fn extipsel2(&mut self) -> _EXTIPSEL2W {
        _EXTIPSEL2W { w: self }
    }
    #[doc = "Bits 12:15 - External Interrupt 3 Port Select"]
    #[inline]
    pub fn extipsel3(&mut self) -> _EXTIPSEL3W {
        _EXTIPSEL3W { w: self }
    }
    #[doc = "Bits 16:19 - External Interrupt 4 Port Select"]
    #[inline]
    pub fn extipsel4(&mut self) -> _EXTIPSEL4W {
        _EXTIPSEL4W { w: self }
    }
    #[doc = "Bits 20:23 - External Interrupt 5 Port Select"]
    #[inline]
    pub fn extipsel5(&mut self) -> _EXTIPSEL5W {
        _EXTIPSEL5W { w: self }
    }
    #[doc = "Bits 24:27 - External Interrupt 6 Port Select"]
    #[inline]
    pub fn extipsel6(&mut self) -> _EXTIPSEL6W {
        _EXTIPSEL6W { w: self }
    }
    #[doc = "Bits 28:31 - External Interrupt 7 Port Select"]
    #[inline]
    pub fn extipsel7(&mut self) -> _EXTIPSEL7W {
        _EXTIPSEL7W { w: self }
    }
}
