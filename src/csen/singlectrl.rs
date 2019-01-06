#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SINGLECTRL {
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
        R { bits: self.register.get() }
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
#[doc = "Possible values of the field `SINGLESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SINGLESELR {
    #[doc = "undocumented"]
    APORT1XCH0,
    #[doc = "undocumented"]
    APORT1YCH1,
    #[doc = "undocumented"]
    APORT1XCH2,
    #[doc = "undocumented"]
    APORT1YCH3,
    #[doc = "undocumented"]
    APORT1XCH4,
    #[doc = "undocumented"]
    APORT1YCH5,
    #[doc = "undocumented"]
    APORT1XCH6,
    #[doc = "undocumented"]
    APORT1YCH7,
    #[doc = "undocumented"]
    APORT1XCH8,
    #[doc = "undocumented"]
    APORT1YCH9,
    #[doc = "undocumented"]
    APORT1XCH10,
    #[doc = "undocumented"]
    APORT1YCH11,
    #[doc = "undocumented"]
    APORT1XCH12,
    #[doc = "undocumented"]
    APORT1YCH13,
    #[doc = "undocumented"]
    APORT1XCH14,
    #[doc = "undocumented"]
    APORT1YCH15,
    #[doc = "undocumented"]
    APORT1XCH16,
    #[doc = "undocumented"]
    APORT1YCH17,
    #[doc = "undocumented"]
    APORT1XCH18,
    #[doc = "undocumented"]
    APORT1YCH19,
    #[doc = "undocumented"]
    APORT1XCH20,
    #[doc = "undocumented"]
    APORT1YCH21,
    #[doc = "undocumented"]
    APORT1XCH22,
    #[doc = "undocumented"]
    APORT1YCH23,
    #[doc = "undocumented"]
    APORT1XCH24,
    #[doc = "undocumented"]
    APORT1YCH25,
    #[doc = "undocumented"]
    APORT1XCH26,
    #[doc = "undocumented"]
    APORT1YCH27,
    #[doc = "undocumented"]
    APORT1XCH28,
    #[doc = "undocumented"]
    APORT1YCH29,
    #[doc = "undocumented"]
    APORT1XCH30,
    #[doc = "undocumented"]
    APORT1YCH31,
    #[doc = "undocumented"]
    APORT3XCH0,
    #[doc = "undocumented"]
    APORT3YCH1,
    #[doc = "undocumented"]
    APORT3XCH2,
    #[doc = "undocumented"]
    APORT3YCH3,
    #[doc = "undocumented"]
    APORT3XCH4,
    #[doc = "undocumented"]
    APORT3YCH5,
    #[doc = "undocumented"]
    APORT3XCH6,
    #[doc = "undocumented"]
    APORT3YCH7,
    #[doc = "undocumented"]
    APORT3XCH8,
    #[doc = "undocumented"]
    APORT3YCH9,
    #[doc = "undocumented"]
    APORT3XCH10,
    #[doc = "undocumented"]
    APORT3YCH11,
    #[doc = "undocumented"]
    APORT3XCH12,
    #[doc = "undocumented"]
    APORT3YCH13,
    #[doc = "undocumented"]
    APORT3XCH14,
    #[doc = "undocumented"]
    APORT3YCH15,
    #[doc = "undocumented"]
    APORT3XCH16,
    #[doc = "undocumented"]
    APORT3YCH17,
    #[doc = "undocumented"]
    APORT3XCH18,
    #[doc = "undocumented"]
    APORT3YCH19,
    #[doc = "undocumented"]
    APORT3XCH20,
    #[doc = "undocumented"]
    APORT3YCH21,
    #[doc = "undocumented"]
    APORT3XCH22,
    #[doc = "undocumented"]
    APORT3YCH23,
    #[doc = "undocumented"]
    APORT3XCH24,
    #[doc = "undocumented"]
    APORT3YCH25,
    #[doc = "undocumented"]
    APORT3XCH26,
    #[doc = "undocumented"]
    APORT3YCH27,
    #[doc = "undocumented"]
    APORT3XCH28,
    #[doc = "undocumented"]
    APORT3YCH29,
    #[doc = "undocumented"]
    APORT3XCH30,
    #[doc = "undocumented"]
    APORT3YCH31,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SINGLESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SINGLESELR::APORT1XCH0 => 32,
            SINGLESELR::APORT1YCH1 => 33,
            SINGLESELR::APORT1XCH2 => 34,
            SINGLESELR::APORT1YCH3 => 35,
            SINGLESELR::APORT1XCH4 => 36,
            SINGLESELR::APORT1YCH5 => 37,
            SINGLESELR::APORT1XCH6 => 38,
            SINGLESELR::APORT1YCH7 => 39,
            SINGLESELR::APORT1XCH8 => 40,
            SINGLESELR::APORT1YCH9 => 41,
            SINGLESELR::APORT1XCH10 => 42,
            SINGLESELR::APORT1YCH11 => 43,
            SINGLESELR::APORT1XCH12 => 44,
            SINGLESELR::APORT1YCH13 => 45,
            SINGLESELR::APORT1XCH14 => 46,
            SINGLESELR::APORT1YCH15 => 47,
            SINGLESELR::APORT1XCH16 => 48,
            SINGLESELR::APORT1YCH17 => 49,
            SINGLESELR::APORT1XCH18 => 50,
            SINGLESELR::APORT1YCH19 => 51,
            SINGLESELR::APORT1XCH20 => 52,
            SINGLESELR::APORT1YCH21 => 53,
            SINGLESELR::APORT1XCH22 => 54,
            SINGLESELR::APORT1YCH23 => 55,
            SINGLESELR::APORT1XCH24 => 56,
            SINGLESELR::APORT1YCH25 => 57,
            SINGLESELR::APORT1XCH26 => 58,
            SINGLESELR::APORT1YCH27 => 59,
            SINGLESELR::APORT1XCH28 => 60,
            SINGLESELR::APORT1YCH29 => 61,
            SINGLESELR::APORT1XCH30 => 62,
            SINGLESELR::APORT1YCH31 => 63,
            SINGLESELR::APORT3XCH0 => 96,
            SINGLESELR::APORT3YCH1 => 97,
            SINGLESELR::APORT3XCH2 => 98,
            SINGLESELR::APORT3YCH3 => 99,
            SINGLESELR::APORT3XCH4 => 100,
            SINGLESELR::APORT3YCH5 => 101,
            SINGLESELR::APORT3XCH6 => 102,
            SINGLESELR::APORT3YCH7 => 103,
            SINGLESELR::APORT3XCH8 => 104,
            SINGLESELR::APORT3YCH9 => 105,
            SINGLESELR::APORT3XCH10 => 106,
            SINGLESELR::APORT3YCH11 => 107,
            SINGLESELR::APORT3XCH12 => 108,
            SINGLESELR::APORT3YCH13 => 109,
            SINGLESELR::APORT3XCH14 => 110,
            SINGLESELR::APORT3YCH15 => 111,
            SINGLESELR::APORT3XCH16 => 112,
            SINGLESELR::APORT3YCH17 => 113,
            SINGLESELR::APORT3XCH18 => 114,
            SINGLESELR::APORT3YCH19 => 115,
            SINGLESELR::APORT3XCH20 => 116,
            SINGLESELR::APORT3YCH21 => 117,
            SINGLESELR::APORT3XCH22 => 118,
            SINGLESELR::APORT3YCH23 => 119,
            SINGLESELR::APORT3XCH24 => 120,
            SINGLESELR::APORT3YCH25 => 121,
            SINGLESELR::APORT3XCH26 => 122,
            SINGLESELR::APORT3YCH27 => 123,
            SINGLESELR::APORT3XCH28 => 124,
            SINGLESELR::APORT3YCH29 => 125,
            SINGLESELR::APORT3XCH30 => 126,
            SINGLESELR::APORT3YCH31 => 127,
            SINGLESELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SINGLESELR {
        match value {
            32 => SINGLESELR::APORT1XCH0,
            33 => SINGLESELR::APORT1YCH1,
            34 => SINGLESELR::APORT1XCH2,
            35 => SINGLESELR::APORT1YCH3,
            36 => SINGLESELR::APORT1XCH4,
            37 => SINGLESELR::APORT1YCH5,
            38 => SINGLESELR::APORT1XCH6,
            39 => SINGLESELR::APORT1YCH7,
            40 => SINGLESELR::APORT1XCH8,
            41 => SINGLESELR::APORT1YCH9,
            42 => SINGLESELR::APORT1XCH10,
            43 => SINGLESELR::APORT1YCH11,
            44 => SINGLESELR::APORT1XCH12,
            45 => SINGLESELR::APORT1YCH13,
            46 => SINGLESELR::APORT1XCH14,
            47 => SINGLESELR::APORT1YCH15,
            48 => SINGLESELR::APORT1XCH16,
            49 => SINGLESELR::APORT1YCH17,
            50 => SINGLESELR::APORT1XCH18,
            51 => SINGLESELR::APORT1YCH19,
            52 => SINGLESELR::APORT1XCH20,
            53 => SINGLESELR::APORT1YCH21,
            54 => SINGLESELR::APORT1XCH22,
            55 => SINGLESELR::APORT1YCH23,
            56 => SINGLESELR::APORT1XCH24,
            57 => SINGLESELR::APORT1YCH25,
            58 => SINGLESELR::APORT1XCH26,
            59 => SINGLESELR::APORT1YCH27,
            60 => SINGLESELR::APORT1XCH28,
            61 => SINGLESELR::APORT1YCH29,
            62 => SINGLESELR::APORT1XCH30,
            63 => SINGLESELR::APORT1YCH31,
            96 => SINGLESELR::APORT3XCH0,
            97 => SINGLESELR::APORT3YCH1,
            98 => SINGLESELR::APORT3XCH2,
            99 => SINGLESELR::APORT3YCH3,
            100 => SINGLESELR::APORT3XCH4,
            101 => SINGLESELR::APORT3YCH5,
            102 => SINGLESELR::APORT3XCH6,
            103 => SINGLESELR::APORT3YCH7,
            104 => SINGLESELR::APORT3XCH8,
            105 => SINGLESELR::APORT3YCH9,
            106 => SINGLESELR::APORT3XCH10,
            107 => SINGLESELR::APORT3YCH11,
            108 => SINGLESELR::APORT3XCH12,
            109 => SINGLESELR::APORT3YCH13,
            110 => SINGLESELR::APORT3XCH14,
            111 => SINGLESELR::APORT3YCH15,
            112 => SINGLESELR::APORT3XCH16,
            113 => SINGLESELR::APORT3YCH17,
            114 => SINGLESELR::APORT3XCH18,
            115 => SINGLESELR::APORT3YCH19,
            116 => SINGLESELR::APORT3XCH20,
            117 => SINGLESELR::APORT3YCH21,
            118 => SINGLESELR::APORT3XCH22,
            119 => SINGLESELR::APORT3YCH23,
            120 => SINGLESELR::APORT3XCH24,
            121 => SINGLESELR::APORT3YCH25,
            122 => SINGLESELR::APORT3XCH26,
            123 => SINGLESELR::APORT3YCH27,
            124 => SINGLESELR::APORT3XCH28,
            125 => SINGLESELR::APORT3YCH29,
            126 => SINGLESELR::APORT3XCH30,
            127 => SINGLESELR::APORT3YCH31,
            i => SINGLESELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1XCH0`"]
    #[inline]
    pub fn is_aport1xch0(&self) -> bool {
        *self == SINGLESELR::APORT1XCH0
    }
    #[doc = "Checks if the value of the field is `APORT1YCH1`"]
    #[inline]
    pub fn is_aport1ych1(&self) -> bool {
        *self == SINGLESELR::APORT1YCH1
    }
    #[doc = "Checks if the value of the field is `APORT1XCH2`"]
    #[inline]
    pub fn is_aport1xch2(&self) -> bool {
        *self == SINGLESELR::APORT1XCH2
    }
    #[doc = "Checks if the value of the field is `APORT1YCH3`"]
    #[inline]
    pub fn is_aport1ych3(&self) -> bool {
        *self == SINGLESELR::APORT1YCH3
    }
    #[doc = "Checks if the value of the field is `APORT1XCH4`"]
    #[inline]
    pub fn is_aport1xch4(&self) -> bool {
        *self == SINGLESELR::APORT1XCH4
    }
    #[doc = "Checks if the value of the field is `APORT1YCH5`"]
    #[inline]
    pub fn is_aport1ych5(&self) -> bool {
        *self == SINGLESELR::APORT1YCH5
    }
    #[doc = "Checks if the value of the field is `APORT1XCH6`"]
    #[inline]
    pub fn is_aport1xch6(&self) -> bool {
        *self == SINGLESELR::APORT1XCH6
    }
    #[doc = "Checks if the value of the field is `APORT1YCH7`"]
    #[inline]
    pub fn is_aport1ych7(&self) -> bool {
        *self == SINGLESELR::APORT1YCH7
    }
    #[doc = "Checks if the value of the field is `APORT1XCH8`"]
    #[inline]
    pub fn is_aport1xch8(&self) -> bool {
        *self == SINGLESELR::APORT1XCH8
    }
    #[doc = "Checks if the value of the field is `APORT1YCH9`"]
    #[inline]
    pub fn is_aport1ych9(&self) -> bool {
        *self == SINGLESELR::APORT1YCH9
    }
    #[doc = "Checks if the value of the field is `APORT1XCH10`"]
    #[inline]
    pub fn is_aport1xch10(&self) -> bool {
        *self == SINGLESELR::APORT1XCH10
    }
    #[doc = "Checks if the value of the field is `APORT1YCH11`"]
    #[inline]
    pub fn is_aport1ych11(&self) -> bool {
        *self == SINGLESELR::APORT1YCH11
    }
    #[doc = "Checks if the value of the field is `APORT1XCH12`"]
    #[inline]
    pub fn is_aport1xch12(&self) -> bool {
        *self == SINGLESELR::APORT1XCH12
    }
    #[doc = "Checks if the value of the field is `APORT1YCH13`"]
    #[inline]
    pub fn is_aport1ych13(&self) -> bool {
        *self == SINGLESELR::APORT1YCH13
    }
    #[doc = "Checks if the value of the field is `APORT1XCH14`"]
    #[inline]
    pub fn is_aport1xch14(&self) -> bool {
        *self == SINGLESELR::APORT1XCH14
    }
    #[doc = "Checks if the value of the field is `APORT1YCH15`"]
    #[inline]
    pub fn is_aport1ych15(&self) -> bool {
        *self == SINGLESELR::APORT1YCH15
    }
    #[doc = "Checks if the value of the field is `APORT1XCH16`"]
    #[inline]
    pub fn is_aport1xch16(&self) -> bool {
        *self == SINGLESELR::APORT1XCH16
    }
    #[doc = "Checks if the value of the field is `APORT1YCH17`"]
    #[inline]
    pub fn is_aport1ych17(&self) -> bool {
        *self == SINGLESELR::APORT1YCH17
    }
    #[doc = "Checks if the value of the field is `APORT1XCH18`"]
    #[inline]
    pub fn is_aport1xch18(&self) -> bool {
        *self == SINGLESELR::APORT1XCH18
    }
    #[doc = "Checks if the value of the field is `APORT1YCH19`"]
    #[inline]
    pub fn is_aport1ych19(&self) -> bool {
        *self == SINGLESELR::APORT1YCH19
    }
    #[doc = "Checks if the value of the field is `APORT1XCH20`"]
    #[inline]
    pub fn is_aport1xch20(&self) -> bool {
        *self == SINGLESELR::APORT1XCH20
    }
    #[doc = "Checks if the value of the field is `APORT1YCH21`"]
    #[inline]
    pub fn is_aport1ych21(&self) -> bool {
        *self == SINGLESELR::APORT1YCH21
    }
    #[doc = "Checks if the value of the field is `APORT1XCH22`"]
    #[inline]
    pub fn is_aport1xch22(&self) -> bool {
        *self == SINGLESELR::APORT1XCH22
    }
    #[doc = "Checks if the value of the field is `APORT1YCH23`"]
    #[inline]
    pub fn is_aport1ych23(&self) -> bool {
        *self == SINGLESELR::APORT1YCH23
    }
    #[doc = "Checks if the value of the field is `APORT1XCH24`"]
    #[inline]
    pub fn is_aport1xch24(&self) -> bool {
        *self == SINGLESELR::APORT1XCH24
    }
    #[doc = "Checks if the value of the field is `APORT1YCH25`"]
    #[inline]
    pub fn is_aport1ych25(&self) -> bool {
        *self == SINGLESELR::APORT1YCH25
    }
    #[doc = "Checks if the value of the field is `APORT1XCH26`"]
    #[inline]
    pub fn is_aport1xch26(&self) -> bool {
        *self == SINGLESELR::APORT1XCH26
    }
    #[doc = "Checks if the value of the field is `APORT1YCH27`"]
    #[inline]
    pub fn is_aport1ych27(&self) -> bool {
        *self == SINGLESELR::APORT1YCH27
    }
    #[doc = "Checks if the value of the field is `APORT1XCH28`"]
    #[inline]
    pub fn is_aport1xch28(&self) -> bool {
        *self == SINGLESELR::APORT1XCH28
    }
    #[doc = "Checks if the value of the field is `APORT1YCH29`"]
    #[inline]
    pub fn is_aport1ych29(&self) -> bool {
        *self == SINGLESELR::APORT1YCH29
    }
    #[doc = "Checks if the value of the field is `APORT1XCH30`"]
    #[inline]
    pub fn is_aport1xch30(&self) -> bool {
        *self == SINGLESELR::APORT1XCH30
    }
    #[doc = "Checks if the value of the field is `APORT1YCH31`"]
    #[inline]
    pub fn is_aport1ych31(&self) -> bool {
        *self == SINGLESELR::APORT1YCH31
    }
    #[doc = "Checks if the value of the field is `APORT3XCH0`"]
    #[inline]
    pub fn is_aport3xch0(&self) -> bool {
        *self == SINGLESELR::APORT3XCH0
    }
    #[doc = "Checks if the value of the field is `APORT3YCH1`"]
    #[inline]
    pub fn is_aport3ych1(&self) -> bool {
        *self == SINGLESELR::APORT3YCH1
    }
    #[doc = "Checks if the value of the field is `APORT3XCH2`"]
    #[inline]
    pub fn is_aport3xch2(&self) -> bool {
        *self == SINGLESELR::APORT3XCH2
    }
    #[doc = "Checks if the value of the field is `APORT3YCH3`"]
    #[inline]
    pub fn is_aport3ych3(&self) -> bool {
        *self == SINGLESELR::APORT3YCH3
    }
    #[doc = "Checks if the value of the field is `APORT3XCH4`"]
    #[inline]
    pub fn is_aport3xch4(&self) -> bool {
        *self == SINGLESELR::APORT3XCH4
    }
    #[doc = "Checks if the value of the field is `APORT3YCH5`"]
    #[inline]
    pub fn is_aport3ych5(&self) -> bool {
        *self == SINGLESELR::APORT3YCH5
    }
    #[doc = "Checks if the value of the field is `APORT3XCH6`"]
    #[inline]
    pub fn is_aport3xch6(&self) -> bool {
        *self == SINGLESELR::APORT3XCH6
    }
    #[doc = "Checks if the value of the field is `APORT3YCH7`"]
    #[inline]
    pub fn is_aport3ych7(&self) -> bool {
        *self == SINGLESELR::APORT3YCH7
    }
    #[doc = "Checks if the value of the field is `APORT3XCH8`"]
    #[inline]
    pub fn is_aport3xch8(&self) -> bool {
        *self == SINGLESELR::APORT3XCH8
    }
    #[doc = "Checks if the value of the field is `APORT3YCH9`"]
    #[inline]
    pub fn is_aport3ych9(&self) -> bool {
        *self == SINGLESELR::APORT3YCH9
    }
    #[doc = "Checks if the value of the field is `APORT3XCH10`"]
    #[inline]
    pub fn is_aport3xch10(&self) -> bool {
        *self == SINGLESELR::APORT3XCH10
    }
    #[doc = "Checks if the value of the field is `APORT3YCH11`"]
    #[inline]
    pub fn is_aport3ych11(&self) -> bool {
        *self == SINGLESELR::APORT3YCH11
    }
    #[doc = "Checks if the value of the field is `APORT3XCH12`"]
    #[inline]
    pub fn is_aport3xch12(&self) -> bool {
        *self == SINGLESELR::APORT3XCH12
    }
    #[doc = "Checks if the value of the field is `APORT3YCH13`"]
    #[inline]
    pub fn is_aport3ych13(&self) -> bool {
        *self == SINGLESELR::APORT3YCH13
    }
    #[doc = "Checks if the value of the field is `APORT3XCH14`"]
    #[inline]
    pub fn is_aport3xch14(&self) -> bool {
        *self == SINGLESELR::APORT3XCH14
    }
    #[doc = "Checks if the value of the field is `APORT3YCH15`"]
    #[inline]
    pub fn is_aport3ych15(&self) -> bool {
        *self == SINGLESELR::APORT3YCH15
    }
    #[doc = "Checks if the value of the field is `APORT3XCH16`"]
    #[inline]
    pub fn is_aport3xch16(&self) -> bool {
        *self == SINGLESELR::APORT3XCH16
    }
    #[doc = "Checks if the value of the field is `APORT3YCH17`"]
    #[inline]
    pub fn is_aport3ych17(&self) -> bool {
        *self == SINGLESELR::APORT3YCH17
    }
    #[doc = "Checks if the value of the field is `APORT3XCH18`"]
    #[inline]
    pub fn is_aport3xch18(&self) -> bool {
        *self == SINGLESELR::APORT3XCH18
    }
    #[doc = "Checks if the value of the field is `APORT3YCH19`"]
    #[inline]
    pub fn is_aport3ych19(&self) -> bool {
        *self == SINGLESELR::APORT3YCH19
    }
    #[doc = "Checks if the value of the field is `APORT3XCH20`"]
    #[inline]
    pub fn is_aport3xch20(&self) -> bool {
        *self == SINGLESELR::APORT3XCH20
    }
    #[doc = "Checks if the value of the field is `APORT3YCH21`"]
    #[inline]
    pub fn is_aport3ych21(&self) -> bool {
        *self == SINGLESELR::APORT3YCH21
    }
    #[doc = "Checks if the value of the field is `APORT3XCH22`"]
    #[inline]
    pub fn is_aport3xch22(&self) -> bool {
        *self == SINGLESELR::APORT3XCH22
    }
    #[doc = "Checks if the value of the field is `APORT3YCH23`"]
    #[inline]
    pub fn is_aport3ych23(&self) -> bool {
        *self == SINGLESELR::APORT3YCH23
    }
    #[doc = "Checks if the value of the field is `APORT3XCH24`"]
    #[inline]
    pub fn is_aport3xch24(&self) -> bool {
        *self == SINGLESELR::APORT3XCH24
    }
    #[doc = "Checks if the value of the field is `APORT3YCH25`"]
    #[inline]
    pub fn is_aport3ych25(&self) -> bool {
        *self == SINGLESELR::APORT3YCH25
    }
    #[doc = "Checks if the value of the field is `APORT3XCH26`"]
    #[inline]
    pub fn is_aport3xch26(&self) -> bool {
        *self == SINGLESELR::APORT3XCH26
    }
    #[doc = "Checks if the value of the field is `APORT3YCH27`"]
    #[inline]
    pub fn is_aport3ych27(&self) -> bool {
        *self == SINGLESELR::APORT3YCH27
    }
    #[doc = "Checks if the value of the field is `APORT3XCH28`"]
    #[inline]
    pub fn is_aport3xch28(&self) -> bool {
        *self == SINGLESELR::APORT3XCH28
    }
    #[doc = "Checks if the value of the field is `APORT3YCH29`"]
    #[inline]
    pub fn is_aport3ych29(&self) -> bool {
        *self == SINGLESELR::APORT3YCH29
    }
    #[doc = "Checks if the value of the field is `APORT3XCH30`"]
    #[inline]
    pub fn is_aport3xch30(&self) -> bool {
        *self == SINGLESELR::APORT3XCH30
    }
    #[doc = "Checks if the value of the field is `APORT3YCH31`"]
    #[inline]
    pub fn is_aport3ych31(&self) -> bool {
        *self == SINGLESELR::APORT3YCH31
    }
}
#[doc = "Values that can be written to the field `SINGLESEL`"]
pub enum SINGLESELW {
    #[doc = "`100000`"]
    APORT1XCH0,
    #[doc = "`100001`"]
    APORT1YCH1,
    #[doc = "`100010`"]
    APORT1XCH2,
    #[doc = "`100011`"]
    APORT1YCH3,
    #[doc = "`100100`"]
    APORT1XCH4,
    #[doc = "`100101`"]
    APORT1YCH5,
    #[doc = "`100110`"]
    APORT1XCH6,
    #[doc = "`100111`"]
    APORT1YCH7,
    #[doc = "`101000`"]
    APORT1XCH8,
    #[doc = "`101001`"]
    APORT1YCH9,
    #[doc = "`101010`"]
    APORT1XCH10,
    #[doc = "`101011`"]
    APORT1YCH11,
    #[doc = "`101100`"]
    APORT1XCH12,
    #[doc = "`101101`"]
    APORT1YCH13,
    #[doc = "`101110`"]
    APORT1XCH14,
    #[doc = "`101111`"]
    APORT1YCH15,
    #[doc = "`110000`"]
    APORT1XCH16,
    #[doc = "`110001`"]
    APORT1YCH17,
    #[doc = "`110010`"]
    APORT1XCH18,
    #[doc = "`110011`"]
    APORT1YCH19,
    #[doc = "`110100`"]
    APORT1XCH20,
    #[doc = "`110101`"]
    APORT1YCH21,
    #[doc = "`110110`"]
    APORT1XCH22,
    #[doc = "`110111`"]
    APORT1YCH23,
    #[doc = "`111000`"]
    APORT1XCH24,
    #[doc = "`111001`"]
    APORT1YCH25,
    #[doc = "`111010`"]
    APORT1XCH26,
    #[doc = "`111011`"]
    APORT1YCH27,
    #[doc = "`111100`"]
    APORT1XCH28,
    #[doc = "`111101`"]
    APORT1YCH29,
    #[doc = "`111110`"]
    APORT1XCH30,
    #[doc = "`111111`"]
    APORT1YCH31,
    #[doc = "`1100000`"]
    APORT3XCH0,
    #[doc = "`1100001`"]
    APORT3YCH1,
    #[doc = "`1100010`"]
    APORT3XCH2,
    #[doc = "`1100011`"]
    APORT3YCH3,
    #[doc = "`1100100`"]
    APORT3XCH4,
    #[doc = "`1100101`"]
    APORT3YCH5,
    #[doc = "`1100110`"]
    APORT3XCH6,
    #[doc = "`1100111`"]
    APORT3YCH7,
    #[doc = "`1101000`"]
    APORT3XCH8,
    #[doc = "`1101001`"]
    APORT3YCH9,
    #[doc = "`1101010`"]
    APORT3XCH10,
    #[doc = "`1101011`"]
    APORT3YCH11,
    #[doc = "`1101100`"]
    APORT3XCH12,
    #[doc = "`1101101`"]
    APORT3YCH13,
    #[doc = "`1101110`"]
    APORT3XCH14,
    #[doc = "`1101111`"]
    APORT3YCH15,
    #[doc = "`1110000`"]
    APORT3XCH16,
    #[doc = "`1110001`"]
    APORT3YCH17,
    #[doc = "`1110010`"]
    APORT3XCH18,
    #[doc = "`1110011`"]
    APORT3YCH19,
    #[doc = "`1110100`"]
    APORT3XCH20,
    #[doc = "`1110101`"]
    APORT3YCH21,
    #[doc = "`1110110`"]
    APORT3XCH22,
    #[doc = "`1110111`"]
    APORT3YCH23,
    #[doc = "`1111000`"]
    APORT3XCH24,
    #[doc = "`1111001`"]
    APORT3YCH25,
    #[doc = "`1111010`"]
    APORT3XCH26,
    #[doc = "`1111011`"]
    APORT3YCH27,
    #[doc = "`1111100`"]
    APORT3XCH28,
    #[doc = "`1111101`"]
    APORT3YCH29,
    #[doc = "`1111110`"]
    APORT3XCH30,
    #[doc = "`1111111`"]
    APORT3YCH31,
}
impl SINGLESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SINGLESELW::APORT1XCH0 => 32,
            SINGLESELW::APORT1YCH1 => 33,
            SINGLESELW::APORT1XCH2 => 34,
            SINGLESELW::APORT1YCH3 => 35,
            SINGLESELW::APORT1XCH4 => 36,
            SINGLESELW::APORT1YCH5 => 37,
            SINGLESELW::APORT1XCH6 => 38,
            SINGLESELW::APORT1YCH7 => 39,
            SINGLESELW::APORT1XCH8 => 40,
            SINGLESELW::APORT1YCH9 => 41,
            SINGLESELW::APORT1XCH10 => 42,
            SINGLESELW::APORT1YCH11 => 43,
            SINGLESELW::APORT1XCH12 => 44,
            SINGLESELW::APORT1YCH13 => 45,
            SINGLESELW::APORT1XCH14 => 46,
            SINGLESELW::APORT1YCH15 => 47,
            SINGLESELW::APORT1XCH16 => 48,
            SINGLESELW::APORT1YCH17 => 49,
            SINGLESELW::APORT1XCH18 => 50,
            SINGLESELW::APORT1YCH19 => 51,
            SINGLESELW::APORT1XCH20 => 52,
            SINGLESELW::APORT1YCH21 => 53,
            SINGLESELW::APORT1XCH22 => 54,
            SINGLESELW::APORT1YCH23 => 55,
            SINGLESELW::APORT1XCH24 => 56,
            SINGLESELW::APORT1YCH25 => 57,
            SINGLESELW::APORT1XCH26 => 58,
            SINGLESELW::APORT1YCH27 => 59,
            SINGLESELW::APORT1XCH28 => 60,
            SINGLESELW::APORT1YCH29 => 61,
            SINGLESELW::APORT1XCH30 => 62,
            SINGLESELW::APORT1YCH31 => 63,
            SINGLESELW::APORT3XCH0 => 96,
            SINGLESELW::APORT3YCH1 => 97,
            SINGLESELW::APORT3XCH2 => 98,
            SINGLESELW::APORT3YCH3 => 99,
            SINGLESELW::APORT3XCH4 => 100,
            SINGLESELW::APORT3YCH5 => 101,
            SINGLESELW::APORT3XCH6 => 102,
            SINGLESELW::APORT3YCH7 => 103,
            SINGLESELW::APORT3XCH8 => 104,
            SINGLESELW::APORT3YCH9 => 105,
            SINGLESELW::APORT3XCH10 => 106,
            SINGLESELW::APORT3YCH11 => 107,
            SINGLESELW::APORT3XCH12 => 108,
            SINGLESELW::APORT3YCH13 => 109,
            SINGLESELW::APORT3XCH14 => 110,
            SINGLESELW::APORT3YCH15 => 111,
            SINGLESELW::APORT3XCH16 => 112,
            SINGLESELW::APORT3YCH17 => 113,
            SINGLESELW::APORT3XCH18 => 114,
            SINGLESELW::APORT3YCH19 => 115,
            SINGLESELW::APORT3XCH20 => 116,
            SINGLESELW::APORT3YCH21 => 117,
            SINGLESELW::APORT3XCH22 => 118,
            SINGLESELW::APORT3YCH23 => 119,
            SINGLESELW::APORT3XCH24 => 120,
            SINGLESELW::APORT3YCH25 => 121,
            SINGLESELW::APORT3XCH26 => 122,
            SINGLESELW::APORT3YCH27 => 123,
            SINGLESELW::APORT3XCH28 => 124,
            SINGLESELW::APORT3YCH29 => 125,
            SINGLESELW::APORT3XCH30 => 126,
            SINGLESELW::APORT3YCH31 => 127,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SINGLESELW<'a> {
    w: &'a mut W,
}
impl<'a> _SINGLESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SINGLESELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "`100000`"]
    #[inline]
    pub fn aport1xch0(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH0)
    }
    #[doc = "`100001`"]
    #[inline]
    pub fn aport1ych1(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH1)
    }
    #[doc = "`100010`"]
    #[inline]
    pub fn aport1xch2(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH2)
    }
    #[doc = "`100011`"]
    #[inline]
    pub fn aport1ych3(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH3)
    }
    #[doc = "`100100`"]
    #[inline]
    pub fn aport1xch4(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH4)
    }
    #[doc = "`100101`"]
    #[inline]
    pub fn aport1ych5(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH5)
    }
    #[doc = "`100110`"]
    #[inline]
    pub fn aport1xch6(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH6)
    }
    #[doc = "`100111`"]
    #[inline]
    pub fn aport1ych7(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH7)
    }
    #[doc = "`101000`"]
    #[inline]
    pub fn aport1xch8(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH8)
    }
    #[doc = "`101001`"]
    #[inline]
    pub fn aport1ych9(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH9)
    }
    #[doc = "`101010`"]
    #[inline]
    pub fn aport1xch10(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH10)
    }
    #[doc = "`101011`"]
    #[inline]
    pub fn aport1ych11(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH11)
    }
    #[doc = "`101100`"]
    #[inline]
    pub fn aport1xch12(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH12)
    }
    #[doc = "`101101`"]
    #[inline]
    pub fn aport1ych13(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH13)
    }
    #[doc = "`101110`"]
    #[inline]
    pub fn aport1xch14(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH14)
    }
    #[doc = "`101111`"]
    #[inline]
    pub fn aport1ych15(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH15)
    }
    #[doc = "`110000`"]
    #[inline]
    pub fn aport1xch16(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH16)
    }
    #[doc = "`110001`"]
    #[inline]
    pub fn aport1ych17(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH17)
    }
    #[doc = "`110010`"]
    #[inline]
    pub fn aport1xch18(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH18)
    }
    #[doc = "`110011`"]
    #[inline]
    pub fn aport1ych19(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH19)
    }
    #[doc = "`110100`"]
    #[inline]
    pub fn aport1xch20(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH20)
    }
    #[doc = "`110101`"]
    #[inline]
    pub fn aport1ych21(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH21)
    }
    #[doc = "`110110`"]
    #[inline]
    pub fn aport1xch22(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH22)
    }
    #[doc = "`110111`"]
    #[inline]
    pub fn aport1ych23(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH23)
    }
    #[doc = "`111000`"]
    #[inline]
    pub fn aport1xch24(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH24)
    }
    #[doc = "`111001`"]
    #[inline]
    pub fn aport1ych25(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH25)
    }
    #[doc = "`111010`"]
    #[inline]
    pub fn aport1xch26(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH26)
    }
    #[doc = "`111011`"]
    #[inline]
    pub fn aport1ych27(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH27)
    }
    #[doc = "`111100`"]
    #[inline]
    pub fn aport1xch28(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH28)
    }
    #[doc = "`111101`"]
    #[inline]
    pub fn aport1ych29(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH29)
    }
    #[doc = "`111110`"]
    #[inline]
    pub fn aport1xch30(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1XCH30)
    }
    #[doc = "`111111`"]
    #[inline]
    pub fn aport1ych31(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT1YCH31)
    }
    #[doc = "`1100000`"]
    #[inline]
    pub fn aport3xch0(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH0)
    }
    #[doc = "`1100001`"]
    #[inline]
    pub fn aport3ych1(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH1)
    }
    #[doc = "`1100010`"]
    #[inline]
    pub fn aport3xch2(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH2)
    }
    #[doc = "`1100011`"]
    #[inline]
    pub fn aport3ych3(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH3)
    }
    #[doc = "`1100100`"]
    #[inline]
    pub fn aport3xch4(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH4)
    }
    #[doc = "`1100101`"]
    #[inline]
    pub fn aport3ych5(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH5)
    }
    #[doc = "`1100110`"]
    #[inline]
    pub fn aport3xch6(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH6)
    }
    #[doc = "`1100111`"]
    #[inline]
    pub fn aport3ych7(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH7)
    }
    #[doc = "`1101000`"]
    #[inline]
    pub fn aport3xch8(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH8)
    }
    #[doc = "`1101001`"]
    #[inline]
    pub fn aport3ych9(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH9)
    }
    #[doc = "`1101010`"]
    #[inline]
    pub fn aport3xch10(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH10)
    }
    #[doc = "`1101011`"]
    #[inline]
    pub fn aport3ych11(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH11)
    }
    #[doc = "`1101100`"]
    #[inline]
    pub fn aport3xch12(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH12)
    }
    #[doc = "`1101101`"]
    #[inline]
    pub fn aport3ych13(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH13)
    }
    #[doc = "`1101110`"]
    #[inline]
    pub fn aport3xch14(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH14)
    }
    #[doc = "`1101111`"]
    #[inline]
    pub fn aport3ych15(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH15)
    }
    #[doc = "`1110000`"]
    #[inline]
    pub fn aport3xch16(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH16)
    }
    #[doc = "`1110001`"]
    #[inline]
    pub fn aport3ych17(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH17)
    }
    #[doc = "`1110010`"]
    #[inline]
    pub fn aport3xch18(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH18)
    }
    #[doc = "`1110011`"]
    #[inline]
    pub fn aport3ych19(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH19)
    }
    #[doc = "`1110100`"]
    #[inline]
    pub fn aport3xch20(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH20)
    }
    #[doc = "`1110101`"]
    #[inline]
    pub fn aport3ych21(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH21)
    }
    #[doc = "`1110110`"]
    #[inline]
    pub fn aport3xch22(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH22)
    }
    #[doc = "`1110111`"]
    #[inline]
    pub fn aport3ych23(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH23)
    }
    #[doc = "`1111000`"]
    #[inline]
    pub fn aport3xch24(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH24)
    }
    #[doc = "`1111001`"]
    #[inline]
    pub fn aport3ych25(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH25)
    }
    #[doc = "`1111010`"]
    #[inline]
    pub fn aport3xch26(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH26)
    }
    #[doc = "`1111011`"]
    #[inline]
    pub fn aport3ych27(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH27)
    }
    #[doc = "`1111100`"]
    #[inline]
    pub fn aport3xch28(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH28)
    }
    #[doc = "`1111101`"]
    #[inline]
    pub fn aport3ych29(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH29)
    }
    #[doc = "`1111110`"]
    #[inline]
    pub fn aport3xch30(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3XCH30)
    }
    #[doc = "`1111111`"]
    #[inline]
    pub fn aport3ych31(self) -> &'a mut W {
        self.variant(SINGLESELW::APORT3YCH31)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 4;
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
    #[doc = "Bits 4:10 - Single Channel Input Select"]
    #[inline]
    pub fn singlesel(&self) -> SINGLESELR {
        SINGLESELR::_from({
            const MASK: u8 = 127;
            const OFFSET: u8 = 4;
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
    #[doc = "Bits 4:10 - Single Channel Input Select"]
    #[inline]
    pub fn singlesel(&mut self) -> _SINGLESELW {
        _SINGLESELW { w: self }
    }
}
