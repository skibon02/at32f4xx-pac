#[doc = "Register `MISC1` reader"]
pub type R = crate::R<MISC1_SPEC>;
#[doc = "Register `MISC1` writer"]
pub type W = crate::W<MISC1_SPEC>;
#[doc = "HICKCAL write key value\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HICKCAL_KEY_A {
    #[doc = "0: Invalid key (0). Write to HICKCAL is ignored"]
    Invalid = 0,
    #[doc = "90: Unlock key (0x5A). HICKCAL can be written"]
    Unlock = 90,
}
impl From<HICKCAL_KEY_A> for u8 {
    #[inline(always)]
    fn from(variant: HICKCAL_KEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HICKCAL_KEY_A {
    type Ux = u8;
}
impl crate::IsEnum for HICKCAL_KEY_A {}
#[doc = "Field `HICKCAL_KEY` reader - HICKCAL write key value"]
pub type HICKCAL_KEY_R = crate::FieldReader<HICKCAL_KEY_A>;
impl HICKCAL_KEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HICKCAL_KEY_A> {
        match self.bits {
            0 => Some(HICKCAL_KEY_A::Invalid),
            90 => Some(HICKCAL_KEY_A::Unlock),
            _ => None,
        }
    }
    #[doc = "Invalid key (0). Write to HICKCAL is ignored"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == HICKCAL_KEY_A::Invalid
    }
    #[doc = "Unlock key (0x5A). HICKCAL can be written"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == HICKCAL_KEY_A::Unlock
    }
}
#[doc = "Field `HICKCAL_KEY` writer - HICKCAL write key value"]
pub type HICKCAL_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 8, HICKCAL_KEY_A>;
impl<'a, REG> HICKCAL_KEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Invalid key (0). Write to HICKCAL is ignored"]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut crate::W<REG> {
        self.variant(HICKCAL_KEY_A::Invalid)
    }
    #[doc = "Unlock key (0x5A). HICKCAL can be written"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(HICKCAL_KEY_A::Unlock)
    }
}
#[doc = "HICK 6 divider selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICKDIV_A {
    #[doc = "0: HICK is divided by 6 to generate the SCLK and USB clock when HICK_TO_SCLK or HICK_TO_USB is set to HICKDiv"]
    Div6 = 0,
    #[doc = "1: HICK is not divided"]
    Div1 = 1,
}
impl From<HICKDIV_A> for bool {
    #[inline(always)]
    fn from(variant: HICKDIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICKDIV` reader - HICK 6 divider selection"]
pub type HICKDIV_R = crate::BitReader<HICKDIV_A>;
impl HICKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HICKDIV_A {
        match self.bits {
            false => HICKDIV_A::Div6,
            true => HICKDIV_A::Div1,
        }
    }
    #[doc = "HICK is divided by 6 to generate the SCLK and USB clock when HICK_TO_SCLK or HICK_TO_USB is set to HICKDiv"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == HICKDIV_A::Div6
    }
    #[doc = "HICK is not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HICKDIV_A::Div1
    }
}
#[doc = "Field `HICKDIV` writer - HICK 6 divider selection"]
pub type HICKDIV_W<'a, REG> = crate::BitWriter<'a, REG, HICKDIV_A>;
impl<'a, REG> HICKDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HICK is divided by 6 to generate the SCLK and USB clock when HICK_TO_SCLK or HICK_TO_USB is set to HICKDiv"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(HICKDIV_A::Div6)
    }
    #[doc = "HICK is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(HICKDIV_A::Div1)
    }
}
#[doc = "HICK to usb clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICK_TO_USB_A {
    #[doc = "0: PLL or PLL division"]
    Plldiv = 0,
    #[doc = "1: 48 MHz or 8 MHz, depending on the HICKDIV"]
    Hickdiv = 1,
}
impl From<HICK_TO_USB_A> for bool {
    #[inline(always)]
    fn from(variant: HICK_TO_USB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICK_TO_USB` reader - HICK to usb clock"]
pub type HICK_TO_USB_R = crate::BitReader<HICK_TO_USB_A>;
impl HICK_TO_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HICK_TO_USB_A {
        match self.bits {
            false => HICK_TO_USB_A::Plldiv,
            true => HICK_TO_USB_A::Hickdiv,
        }
    }
    #[doc = "PLL or PLL division"]
    #[inline(always)]
    pub fn is_plldiv(&self) -> bool {
        *self == HICK_TO_USB_A::Plldiv
    }
    #[doc = "48 MHz or 8 MHz, depending on the HICKDIV"]
    #[inline(always)]
    pub fn is_hickdiv(&self) -> bool {
        *self == HICK_TO_USB_A::Hickdiv
    }
}
#[doc = "Field `HICK_TO_USB` writer - HICK to usb clock"]
pub type HICK_TO_USB_W<'a, REG> = crate::BitWriter<'a, REG, HICK_TO_USB_A>;
impl<'a, REG> HICK_TO_USB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PLL or PLL division"]
    #[inline(always)]
    pub fn plldiv(self) -> &'a mut crate::W<REG> {
        self.variant(HICK_TO_USB_A::Plldiv)
    }
    #[doc = "48 MHz or 8 MHz, depending on the HICKDIV"]
    #[inline(always)]
    pub fn hickdiv(self) -> &'a mut crate::W<REG> {
        self.variant(HICK_TO_USB_A::Hickdiv)
    }
}
#[doc = "HICK to system clock\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICK_TO_SCLK_A {
    #[doc = "0: Fixed 8 MHz, that is, HICK/6"]
    Hickdiv6 = 0,
    #[doc = "1: 48 MHz or 8 MHz, depending on the HICKDIV"]
    Hickdiv = 1,
}
impl From<HICK_TO_SCLK_A> for bool {
    #[inline(always)]
    fn from(variant: HICK_TO_SCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICK_TO_SCLK` reader - HICK to system clock"]
pub type HICK_TO_SCLK_R = crate::BitReader<HICK_TO_SCLK_A>;
impl HICK_TO_SCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HICK_TO_SCLK_A {
        match self.bits {
            false => HICK_TO_SCLK_A::Hickdiv6,
            true => HICK_TO_SCLK_A::Hickdiv,
        }
    }
    #[doc = "Fixed 8 MHz, that is, HICK/6"]
    #[inline(always)]
    pub fn is_hickdiv6(&self) -> bool {
        *self == HICK_TO_SCLK_A::Hickdiv6
    }
    #[doc = "48 MHz or 8 MHz, depending on the HICKDIV"]
    #[inline(always)]
    pub fn is_hickdiv(&self) -> bool {
        *self == HICK_TO_SCLK_A::Hickdiv
    }
}
#[doc = "Field `HICK_TO_SCLK` writer - HICK to system clock"]
pub type HICK_TO_SCLK_W<'a, REG> = crate::BitWriter<'a, REG, HICK_TO_SCLK_A>;
impl<'a, REG> HICK_TO_SCLK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed 8 MHz, that is, HICK/6"]
    #[inline(always)]
    pub fn hickdiv6(self) -> &'a mut crate::W<REG> {
        self.variant(HICK_TO_SCLK_A::Hickdiv6)
    }
    #[doc = "48 MHz or 8 MHz, depending on the HICKDIV"]
    #[inline(always)]
    pub fn hickdiv(self) -> &'a mut crate::W<REG> {
        self.variant(HICK_TO_SCLK_A::Hickdiv)
    }
}
#[doc = "Clock output2 select2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT2_SEL2_A {
    #[doc = "0: USB clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    Usb = 0,
    #[doc = "1: ADC clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    Adc = 1,
    #[doc = "2: HICK (after divider) clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    Hickdiv = 2,
    #[doc = "3: LICK clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    Lick = 3,
    #[doc = "4: LEXT clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    Lext = 4,
}
impl From<CLKOUT2_SEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT2_SEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT2_SEL2_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT2_SEL2_A {}
#[doc = "Field `CLKOUT2_SEL2` reader - Clock output2 select2"]
pub type CLKOUT2_SEL2_R = crate::FieldReader<CLKOUT2_SEL2_A>;
impl CLKOUT2_SEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CLKOUT2_SEL2_A> {
        match self.bits {
            0 => Some(CLKOUT2_SEL2_A::Usb),
            1 => Some(CLKOUT2_SEL2_A::Adc),
            2 => Some(CLKOUT2_SEL2_A::Hickdiv),
            3 => Some(CLKOUT2_SEL2_A::Lick),
            4 => Some(CLKOUT2_SEL2_A::Lext),
            _ => None,
        }
    }
    #[doc = "USB clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        *self == CLKOUT2_SEL2_A::Usb
    }
    #[doc = "ADC clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == CLKOUT2_SEL2_A::Adc
    }
    #[doc = "HICK (after divider) clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn is_hickdiv(&self) -> bool {
        *self == CLKOUT2_SEL2_A::Hickdiv
    }
    #[doc = "LICK clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn is_lick(&self) -> bool {
        *self == CLKOUT2_SEL2_A::Lick
    }
    #[doc = "LEXT clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn is_lext(&self) -> bool {
        *self == CLKOUT2_SEL2_A::Lext
    }
}
#[doc = "Field `CLKOUT2_SEL2` writer - Clock output2 select2"]
pub type CLKOUT2_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKOUT2_SEL2_A>;
impl<'a, REG> CLKOUT2_SEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL2_A::Usb)
    }
    #[doc = "ADC clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL2_A::Adc)
    }
    #[doc = "HICK (after divider) clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn hickdiv(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL2_A::Hickdiv)
    }
    #[doc = "LICK clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn lick(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL2_A::Lick)
    }
    #[doc = "LEXT clock selected as CLKOUT2 when CLKOUT2_SEL1 is set to Secondary"]
    #[inline(always)]
    pub fn lext(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT2_SEL2_A::Lext)
    }
}
#[doc = "Clock output1 division2\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT1DIV2_A {
    #[doc = "8: Clock output is divided by 2"]
    Div2 = 8,
    #[doc = "9: Clock output is divided by 4"]
    Div4 = 9,
    #[doc = "10: Clock output is divided by 8"]
    Div8 = 10,
    #[doc = "11: Clock output is divided by 16"]
    Div16 = 11,
    #[doc = "12: Clock output is divided by 64"]
    Div64 = 12,
    #[doc = "13: Clock output is divided by 128"]
    Div128 = 13,
    #[doc = "14: Clock output is divided by 256"]
    Div256 = 14,
    #[doc = "15: Clock output is divided by 512"]
    Div512 = 15,
    #[doc = "0: Clock output is divided by 1"]
    Div1 = 0,
}
impl From<CLKOUT1DIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT1DIV2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKOUT1DIV2_A {
    type Ux = u8;
}
impl crate::IsEnum for CLKOUT1DIV2_A {}
#[doc = "Field `CLKOUT1DIV2` reader - Clock output1 division2"]
pub type CLKOUT1DIV2_R = crate::FieldReader<CLKOUT1DIV2_A>;
impl CLKOUT1DIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CLKOUT1DIV2_A {
        match self.bits {
            8 => CLKOUT1DIV2_A::Div2,
            9 => CLKOUT1DIV2_A::Div4,
            10 => CLKOUT1DIV2_A::Div8,
            11 => CLKOUT1DIV2_A::Div16,
            12 => CLKOUT1DIV2_A::Div64,
            13 => CLKOUT1DIV2_A::Div128,
            14 => CLKOUT1DIV2_A::Div256,
            15 => CLKOUT1DIV2_A::Div512,
            _ => CLKOUT1DIV2_A::Div1,
        }
    }
    #[doc = "Clock output is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div2
    }
    #[doc = "Clock output is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div4
    }
    #[doc = "Clock output is divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div8
    }
    #[doc = "Clock output is divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div16
    }
    #[doc = "Clock output is divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div64
    }
    #[doc = "Clock output is divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div128
    }
    #[doc = "Clock output is divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div256
    }
    #[doc = "Clock output is divided by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CLKOUT1DIV2_A::Div512
    }
    #[doc = "Clock output is divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        matches!(self.variant(), CLKOUT1DIV2_A::Div1)
    }
}
#[doc = "Field `CLKOUT1DIV2` writer - Clock output1 division2"]
pub type CLKOUT1DIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 4, CLKOUT1DIV2_A, crate::Safe>;
impl<'a, REG> CLKOUT1DIV2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock output is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div2)
    }
    #[doc = "Clock output is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div4)
    }
    #[doc = "Clock output is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div8)
    }
    #[doc = "Clock output is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div16)
    }
    #[doc = "Clock output is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div64)
    }
    #[doc = "Clock output is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div128)
    }
    #[doc = "Clock output is divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div256)
    }
    #[doc = "Clock output is divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div512)
    }
    #[doc = "Clock output is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKOUT1DIV2_A::Div1)
    }
}
#[doc = "Field `CLKOUT2DIV2` reader - Clock output2 division2"]
pub use CLKOUT1DIV2_R as CLKOUT2DIV2_R;
#[doc = "Field `CLKOUT2DIV2` writer - Clock output2 division2"]
pub use CLKOUT1DIV2_W as CLKOUT2DIV2_W;
impl R {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    pub fn hickcal_key(&self) -> HICKCAL_KEY_R {
        HICKCAL_KEY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - HICK 6 divider selection"]
    #[inline(always)]
    pub fn hickdiv(&self) -> HICKDIV_R {
        HICKDIV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HICK to usb clock"]
    #[inline(always)]
    pub fn hick_to_usb(&self) -> HICK_TO_USB_R {
        HICK_TO_USB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&self) -> HICK_TO_SCLK_R {
        HICK_TO_SCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Clock output2 select2"]
    #[inline(always)]
    pub fn clkout2_sel2(&self) -> CLKOUT2_SEL2_R {
        CLKOUT2_SEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Clock output1 division2"]
    #[inline(always)]
    pub fn clkout1div2(&self) -> CLKOUT1DIV2_R {
        CLKOUT1DIV2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Clock output2 division2"]
    #[inline(always)]
    pub fn clkout2div2(&self) -> CLKOUT2DIV2_R {
        CLKOUT2DIV2_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC1")
            .field("hickcal_key", &self.hickcal_key())
            .field("hickdiv", &self.hickdiv())
            .field("hick_to_usb", &self.hick_to_usb())
            .field("hick_to_sclk", &self.hick_to_sclk())
            .field("clkout2_sel2", &self.clkout2_sel2())
            .field("clkout1div2", &self.clkout1div2())
            .field("clkout2div2", &self.clkout2div2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - HICKCAL write key value"]
    #[inline(always)]
    pub fn hickcal_key(&mut self) -> HICKCAL_KEY_W<'_, MISC1_SPEC> {
        HICKCAL_KEY_W::new(self, 0)
    }
    #[doc = "Bit 12 - HICK 6 divider selection"]
    #[inline(always)]
    pub fn hickdiv(&mut self) -> HICKDIV_W<'_, MISC1_SPEC> {
        HICKDIV_W::new(self, 12)
    }
    #[doc = "Bit 13 - HICK to usb clock"]
    #[inline(always)]
    pub fn hick_to_usb(&mut self) -> HICK_TO_USB_W<'_, MISC1_SPEC> {
        HICK_TO_USB_W::new(self, 13)
    }
    #[doc = "Bit 14 - HICK to system clock"]
    #[inline(always)]
    pub fn hick_to_sclk(&mut self) -> HICK_TO_SCLK_W<'_, MISC1_SPEC> {
        HICK_TO_SCLK_W::new(self, 14)
    }
    #[doc = "Bits 16:19 - Clock output2 select2"]
    #[inline(always)]
    pub fn clkout2_sel2(&mut self) -> CLKOUT2_SEL2_W<'_, MISC1_SPEC> {
        CLKOUT2_SEL2_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Clock output1 division2"]
    #[inline(always)]
    pub fn clkout1div2(&mut self) -> CLKOUT1DIV2_W<'_, MISC1_SPEC> {
        CLKOUT1DIV2_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Clock output2 division2"]
    #[inline(always)]
    pub fn clkout2div2(&mut self) -> CLKOUT2DIV2_W<'_, MISC1_SPEC> {
        CLKOUT2DIV2_W::new(self, 28)
    }
}
#[doc = "Miscellaneous register1\n\nYou can [`read`](crate::Reg::read) this register and get [`misc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC1_SPEC;
impl crate::RegisterSpec for MISC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc1::R`](R) reader structure"]
impl crate::Readable for MISC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc1::W`](W) writer structure"]
impl crate::Writable for MISC1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC1 to value 0"]
impl crate::Resettable for MISC1_SPEC {}
