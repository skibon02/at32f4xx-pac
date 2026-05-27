#[doc = "Register `CFG` reader"]
pub type R = crate::R<CFG_SPEC>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CFG_SPEC>;
#[doc = "System clock select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCLKSEL_A {
    #[doc = "0: System clock from HICK"]
    Hick = 0,
    #[doc = "1: System clock from HEXT"]
    Hext = 1,
    #[doc = "2: System clock from PLL"]
    Pll = 2,
}
impl From<SCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCLKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCLKSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for SCLKSEL_A {}
#[doc = "Field `SCLKSEL` reader - System clock select"]
pub type SCLKSEL_R = crate::FieldReader<SCLKSEL_A>;
impl SCLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SCLKSEL_A> {
        match self.bits {
            0 => Some(SCLKSEL_A::Hick),
            1 => Some(SCLKSEL_A::Hext),
            2 => Some(SCLKSEL_A::Pll),
            _ => None,
        }
    }
    #[doc = "System clock from HICK"]
    #[inline(always)]
    pub fn is_hick(&self) -> bool {
        *self == SCLKSEL_A::Hick
    }
    #[doc = "System clock from HEXT"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == SCLKSEL_A::Hext
    }
    #[doc = "System clock from PLL"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SCLKSEL_A::Pll
    }
}
#[doc = "Field `SCLKSEL` writer - System clock select"]
pub type SCLKSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SCLKSEL_A>;
impl<'a, REG> SCLKSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System clock from HICK"]
    #[inline(always)]
    pub fn hick(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::Hick)
    }
    #[doc = "System clock from HEXT"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::Hext)
    }
    #[doc = "System clock from PLL"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(SCLKSEL_A::Pll)
    }
}
#[doc = "Field `SCLKSTS` reader - System Clock select Status"]
pub use SCLKSEL_R as SCLKSTS_R;
#[doc = "AHB division\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AHBDIV_A {
    #[doc = "8: SCLK divided by 2"]
    Div2 = 8,
    #[doc = "9: SCLK divided by 4"]
    Div4 = 9,
    #[doc = "10: SCLK divided by 8"]
    Div8 = 10,
    #[doc = "11: SCLK divided by 16"]
    Div16 = 11,
    #[doc = "12: SCLK divided by 64"]
    Div64 = 12,
    #[doc = "13: SCLK divided by 128"]
    Div128 = 13,
    #[doc = "14: SCLK divided by 256"]
    Div256 = 14,
    #[doc = "15: SCLK divided by 512"]
    Div512 = 15,
    #[doc = "0: The SCLK is used as AHB clock"]
    Sclk = 0,
}
impl From<AHBDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: AHBDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AHBDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for AHBDIV_A {}
#[doc = "Field `AHBDIV` reader - AHB division"]
pub type AHBDIV_R = crate::FieldReader<AHBDIV_A>;
impl AHBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AHBDIV_A {
        match self.bits {
            8 => AHBDIV_A::Div2,
            9 => AHBDIV_A::Div4,
            10 => AHBDIV_A::Div8,
            11 => AHBDIV_A::Div16,
            12 => AHBDIV_A::Div64,
            13 => AHBDIV_A::Div128,
            14 => AHBDIV_A::Div256,
            15 => AHBDIV_A::Div512,
            _ => AHBDIV_A::Sclk,
        }
    }
    #[doc = "SCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AHBDIV_A::Div2
    }
    #[doc = "SCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AHBDIV_A::Div4
    }
    #[doc = "SCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AHBDIV_A::Div8
    }
    #[doc = "SCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == AHBDIV_A::Div16
    }
    #[doc = "SCLK divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == AHBDIV_A::Div64
    }
    #[doc = "SCLK divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == AHBDIV_A::Div128
    }
    #[doc = "SCLK divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == AHBDIV_A::Div256
    }
    #[doc = "SCLK divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == AHBDIV_A::Div512
    }
    #[doc = "The SCLK is used as AHB clock"]
    #[inline(always)]
    pub fn is_sclk(&self) -> bool {
        matches!(self.variant(), AHBDIV_A::Sclk)
    }
}
#[doc = "Field `AHBDIV` writer - AHB division"]
pub type AHBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, AHBDIV_A, crate::Safe>;
impl<'a, REG> AHBDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div2)
    }
    #[doc = "SCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div4)
    }
    #[doc = "SCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div8)
    }
    #[doc = "SCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div16)
    }
    #[doc = "SCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div64)
    }
    #[doc = "SCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div128)
    }
    #[doc = "SCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div256)
    }
    #[doc = "SCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Div512)
    }
    #[doc = "The SCLK is used as AHB clock"]
    #[inline(always)]
    pub fn sclk(self) -> &'a mut crate::W<REG> {
        self.variant(AHBDIV_A::Sclk)
    }
}
#[doc = "APB1 division\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum APB1DIV_A {
    #[doc = "4: HCLK divided by 2"]
    Div2 = 4,
    #[doc = "5: HCLK divided by 4"]
    Div4 = 5,
    #[doc = "6: HCLK divided by 8"]
    Div8 = 6,
    #[doc = "7: HCLK divided by 16"]
    Div16 = 7,
    #[doc = "0: The divided HCLK is used as APB clock"]
    Hclk = 0,
}
impl From<APB1DIV_A> for u8 {
    #[inline(always)]
    fn from(variant: APB1DIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for APB1DIV_A {
    type Ux = u8;
}
impl crate::IsEnum for APB1DIV_A {}
#[doc = "Field `APB1DIV` reader - APB1 division"]
pub type APB1DIV_R = crate::FieldReader<APB1DIV_A>;
impl APB1DIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> APB1DIV_A {
        match self.bits {
            4 => APB1DIV_A::Div2,
            5 => APB1DIV_A::Div4,
            6 => APB1DIV_A::Div8,
            7 => APB1DIV_A::Div16,
            _ => APB1DIV_A::Hclk,
        }
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == APB1DIV_A::Div2
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == APB1DIV_A::Div4
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == APB1DIV_A::Div8
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == APB1DIV_A::Div16
    }
    #[doc = "The divided HCLK is used as APB clock"]
    #[inline(always)]
    pub fn is_hclk(&self) -> bool {
        matches!(self.variant(), APB1DIV_A::Hclk)
    }
}
#[doc = "Field `APB1DIV` writer - APB1 division"]
pub type APB1DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3, APB1DIV_A, crate::Safe>;
impl<'a, REG> APB1DIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Div16)
    }
    #[doc = "The divided HCLK is used as APB clock"]
    #[inline(always)]
    pub fn hclk(self) -> &'a mut crate::W<REG> {
        self.variant(APB1DIV_A::Hclk)
    }
}
#[doc = "Field `APB2DIV` reader - APB2 division"]
pub use APB1DIV_R as APB2DIV_R;
#[doc = "Field `APB2DIV` writer - APB2 division"]
pub use APB1DIV_W as APB2DIV_W;
#[doc = "Field `ERTCDIV` reader - HEXT division for ERTC clock (2-31)"]
pub type ERTCDIV_R = crate::FieldReader;
#[doc = "Field `ERTCDIV` writer - HEXT division for ERTC clock (2-31)"]
pub type ERTCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Clock output1 selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT1_SEL_A {
    #[doc = "0: HICK clock selected as CLKOUT1"]
    Hick = 0,
    #[doc = "1: LEXT clock selected as CLKOUT1"]
    Lext = 1,
    #[doc = "2: HEXT clock selected as CLKOUT1"]
    Hext = 2,
    #[doc = "3: PLL clock selected as CLKOUT1"]
    Pll = 3,
}
impl From<CLKOUT1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT1_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT1_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT1_SEL_A {}
#[doc = "Field `CLKOUT1_SEL` reader - Clock output1 selection"]
pub type CLKOUT1_SEL_R = crate::FieldReader<CLKOUT1_SEL_A>;
impl CLKOUT1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKOUT1_SEL_A {
        match self.bits {
            0 => CLKOUT1_SEL_A::Hick,
            1 => CLKOUT1_SEL_A::Lext,
            2 => CLKOUT1_SEL_A::Hext,
            3 => CLKOUT1_SEL_A::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "HICK clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn is_hick(&self) -> bool {
        *self == CLKOUT1_SEL_A::Hick
    }
    #[doc = "LEXT clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn is_lext(&self) -> bool {
        *self == CLKOUT1_SEL_A::Lext
    }
    #[doc = "HEXT clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == CLKOUT1_SEL_A::Hext
    }
    #[doc = "PLL clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CLKOUT1_SEL_A::Pll
    }
}
#[doc = "Field `CLKOUT1_SEL` writer - Clock output1 selection"]
pub type CLKOUT1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKOUT1_SEL_A, crate::Safe>;
impl<'a, REG> CLKOUT1_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HICK clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn hick(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1_SEL_A::Hick)
    }
    #[doc = "LEXT clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn lext(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1_SEL_A::Lext)
    }
    #[doc = "HEXT clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1_SEL_A::Hext)
    }
    #[doc = "PLL clock selected as CLKOUT1"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1_SEL_A::Pll)
    }
}
#[doc = "Clock output1 division1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT1DIV1_A {
    #[doc = "4: Clock output is divided by 2"]
    Div2 = 4,
    #[doc = "5: Clock output is divided by 3"]
    Div3 = 5,
    #[doc = "6: Clock output is divided by 4"]
    Div4 = 6,
    #[doc = "7: Clock output is divided by 5"]
    Div5 = 7,
    #[doc = "0: Clock output is divided by 1"]
    Div1 = 0,
}
impl From<CLKOUT1DIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT1DIV1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT1DIV1_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT1DIV1_A {}
#[doc = "Field `CLKOUT1DIV1` reader - Clock output1 division1"]
pub type CLKOUT1DIV1_R = crate::FieldReader<CLKOUT1DIV1_A>;
impl CLKOUT1DIV1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKOUT1DIV1_A {
        match self.bits {
            4 => CLKOUT1DIV1_A::Div2,
            5 => CLKOUT1DIV1_A::Div3,
            6 => CLKOUT1DIV1_A::Div4,
            7 => CLKOUT1DIV1_A::Div5,
            _ => CLKOUT1DIV1_A::Div1,
        }
    }
    #[doc = "Clock output is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKOUT1DIV1_A::Div2
    }
    #[doc = "Clock output is divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == CLKOUT1DIV1_A::Div3
    }
    #[doc = "Clock output is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKOUT1DIV1_A::Div4
    }
    #[doc = "Clock output is divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == CLKOUT1DIV1_A::Div5
    }
    #[doc = "Clock output is divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), CLKOUT1DIV1_A::Div1)
    }
}
#[doc = "Field `CLKOUT1DIV1` writer - Clock output1 division1"]
pub type CLKOUT1DIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CLKOUT1DIV1_A, crate::Safe>;
impl<'a, REG> CLKOUT1DIV1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock output is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV1_A::Div2)
    }
    #[doc = "Clock output is divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV1_A::Div3)
    }
    #[doc = "Clock output is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV1_A::Div4)
    }
    #[doc = "Clock output is divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV1_A::Div5)
    }
    #[doc = "Clock output is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV1_A::Div1)
    }
}
#[doc = "Field `CLKOUT2DIV1` reader - Clock output2 division1"]
pub use CLKOUT1DIV1_R as CLKOUT2DIV1_R;
#[doc = "Field `CLKOUT2DIV1` writer - Clock output2 division1"]
pub use CLKOUT1DIV1_W as CLKOUT2DIV1_W;
#[doc = "Clock output2 selection1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT2_SEL1_A {
    #[doc = "0: SCLK clock selected as CLKOUT2"]
    Sclk = 0,
    #[doc = "1: Secondary clock selected as CLKOUT2 (controlled by CLKOUT2_SEL2 bit in the CRM_MISC1 register)"]
    Secondary = 1,
    #[doc = "2: HEXT clock selected as CLKOUT2"]
    Hext = 2,
    #[doc = "3: PLL clock selected as CLKOUT2"]
    Pll = 3,
}
impl From<CLKOUT2_SEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT2_SEL1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT2_SEL1_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT2_SEL1_A {}
#[doc = "Field `CLKOUT2_SEL1` reader - Clock output2 selection1"]
pub type CLKOUT2_SEL1_R = crate::FieldReader<CLKOUT2_SEL1_A>;
impl CLKOUT2_SEL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKOUT2_SEL1_A {
        match self.bits {
            0 => CLKOUT2_SEL1_A::Sclk,
            1 => CLKOUT2_SEL1_A::Secondary,
            2 => CLKOUT2_SEL1_A::Hext,
            3 => CLKOUT2_SEL1_A::Pll,
            _ => unreachable!(),
        }
    }
    #[doc = "SCLK clock selected as CLKOUT2"]
    #[inline(always)]
    pub fn is_sclk(&self) -> bool {
        *self == CLKOUT2_SEL1_A::Sclk
    }
    #[doc = "Secondary clock selected as CLKOUT2 (controlled by CLKOUT2_SEL2 bit in the CRM_MISC1 register)"]
    #[inline(always)]
    pub fn is_secondary(&self) -> bool {
        *self == CLKOUT2_SEL1_A::Secondary
    }
    #[doc = "HEXT clock selected as CLKOUT2"]
    #[inline(always)]
    pub fn is_hext(&self) -> bool {
        *self == CLKOUT2_SEL1_A::Hext
    }
    #[doc = "PLL clock selected as CLKOUT2"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == CLKOUT2_SEL1_A::Pll
    }
}
#[doc = "Field `CLKOUT2_SEL1` writer - Clock output2 selection1"]
pub type CLKOUT2_SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CLKOUT2_SEL1_A, crate::Safe>;
impl<'a, REG> CLKOUT2_SEL1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCLK clock selected as CLKOUT2"]
    #[inline(always)]
    pub fn sclk(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL1_A::Sclk)
    }
    #[doc = "Secondary clock selected as CLKOUT2 (controlled by CLKOUT2_SEL2 bit in the CRM_MISC1 register)"]
    #[inline(always)]
    pub fn secondary(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL1_A::Secondary)
    }
    #[doc = "HEXT clock selected as CLKOUT2"]
    #[inline(always)]
    pub fn hext(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL1_A::Hext)
    }
    #[doc = "PLL clock selected as CLKOUT2"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL1_A::Pll)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    pub fn sclksel(&self) -> SCLKSEL_R {
        SCLKSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock select Status"]
    #[inline(always)]
    pub fn sclksts(&self) -> SCLKSTS_R {
        SCLKSTS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    pub fn ahbdiv(&self) -> AHBDIV_R {
        AHBDIV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&self) -> APB1DIV_R {
        APB1DIV_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&self) -> APB2DIV_R {
        APB2DIV_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock (2-31)"]
    #[inline(always)]
    pub fn ertcdiv(&self) -> ERTCDIV_R {
        ERTCDIV_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Clock output1 selection"]
    #[inline(always)]
    pub fn clkout1_sel(&self) -> CLKOUT1_SEL_R {
        CLKOUT1_SEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Clock output1 division1"]
    #[inline(always)]
    pub fn clkout1div1(&self) -> CLKOUT1DIV1_R {
        CLKOUT1DIV1_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Clock output2 division1"]
    #[inline(always)]
    pub fn clkout2div1(&self) -> CLKOUT2DIV1_R {
        CLKOUT2DIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Clock output2 selection1"]
    #[inline(always)]
    pub fn clkout2_sel1(&self) -> CLKOUT2_SEL1_R {
        CLKOUT2_SEL1_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG")
            .field("sclksel", &self.sclksel())
            .field("sclksts", &self.sclksts())
            .field("ahbdiv", &self.ahbdiv())
            .field("apb1div", &self.apb1div())
            .field("apb2div", &self.apb2div())
            .field("ertcdiv", &self.ertcdiv())
            .field("clkout1_sel", &self.clkout1_sel())
            .field("clkout1div1", &self.clkout1div1())
            .field("clkout2div1", &self.clkout2div1())
            .field("clkout2_sel1", &self.clkout2_sel1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock select"]
    #[inline(always)]
    pub fn sclksel(&mut self) -> SCLKSEL_W<'_, CFG_SPEC> {
        SCLKSEL_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AHB division"]
    #[inline(always)]
    pub fn ahbdiv(&mut self) -> AHBDIV_W<'_, CFG_SPEC> {
        AHBDIV_W::new(self, 4)
    }
    #[doc = "Bits 10:12 - APB1 division"]
    #[inline(always)]
    pub fn apb1div(&mut self) -> APB1DIV_W<'_, CFG_SPEC> {
        APB1DIV_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - APB2 division"]
    #[inline(always)]
    pub fn apb2div(&mut self) -> APB2DIV_W<'_, CFG_SPEC> {
        APB2DIV_W::new(self, 13)
    }
    #[doc = "Bits 16:20 - HEXT division for ERTC clock (2-31)"]
    #[inline(always)]
    pub fn ertcdiv(&mut self) -> ERTCDIV_W<'_, CFG_SPEC> {
        ERTCDIV_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Clock output1 selection"]
    #[inline(always)]
    pub fn clkout1_sel(&mut self) -> CLKOUT1_SEL_W<'_, CFG_SPEC> {
        CLKOUT1_SEL_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Clock output1 division1"]
    #[inline(always)]
    pub fn clkout1div1(&mut self) -> CLKOUT1DIV1_W<'_, CFG_SPEC> {
        CLKOUT1DIV1_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Clock output2 division1"]
    #[inline(always)]
    pub fn clkout2div1(&mut self) -> CLKOUT2DIV1_W<'_, CFG_SPEC> {
        CLKOUT2DIV1_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - Clock output2 selection1"]
    #[inline(always)]
    pub fn clkout2_sel1(&mut self) -> CLKOUT2_SEL1_W<'_, CFG_SPEC> {
        CLKOUT2_SEL1_W::new(self, 30)
    }
}
#[doc = "Clock configuration register(CRM_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {}
