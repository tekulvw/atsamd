#[doc = "Reader of register CPUSEL"]
pub type R = crate::R<u8, super::CPUSEL>;
#[doc = "Writer for register CPUSEL"]
pub type W = crate::W<u8, super::CPUSEL>;
#[doc = "Register CPUSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUSEL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CPU Prescaler Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUDIV_A {
    #[doc = "0: Divide by 1"]
    DIV1,
    #[doc = "1: Divide by 2"]
    DIV2,
    #[doc = "2: Divide by 4"]
    DIV4,
    #[doc = "3: Divide by 8"]
    DIV8,
    #[doc = "4: Divide by 16"]
    DIV16,
    #[doc = "5: Divide by 32"]
    DIV32,
    #[doc = "6: Divide by 64"]
    DIV64,
    #[doc = "7: Divide by 128"]
    DIV128,
}
impl From<CPUDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CPUDIV_A) -> Self {
        match variant {
            CPUDIV_A::DIV1 => 0,
            CPUDIV_A::DIV2 => 1,
            CPUDIV_A::DIV4 => 2,
            CPUDIV_A::DIV8 => 3,
            CPUDIV_A::DIV16 => 4,
            CPUDIV_A::DIV32 => 5,
            CPUDIV_A::DIV64 => 6,
            CPUDIV_A::DIV128 => 7,
        }
    }
}
#[doc = "Reader of field `CPUDIV`"]
pub type CPUDIV_R = crate::R<u8, CPUDIV_A>;
impl CPUDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUDIV_A {
        match self.bits {
            0 => CPUDIV_A::DIV1,
            1 => CPUDIV_A::DIV2,
            2 => CPUDIV_A::DIV4,
            3 => CPUDIV_A::DIV8,
            4 => CPUDIV_A::DIV16,
            5 => CPUDIV_A::DIV32,
            6 => CPUDIV_A::DIV64,
            7 => CPUDIV_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CPUDIV_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CPUDIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CPUDIV_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CPUDIV_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CPUDIV_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CPUDIV_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CPUDIV_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CPUDIV_A::DIV128
    }
}
#[doc = "Write proxy for field `CPUDIV`"]
pub struct CPUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPUDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CPUDIV_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u8) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CPUDIV_R {
        CPUDIV_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CPU Prescaler Selection"]
    #[inline(always)]
    pub fn cpudiv(&mut self) -> CPUDIV_W {
        CPUDIV_W { w: self }
    }
}
