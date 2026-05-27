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
#[doc = "Field `PLLCLK_TO_ADC` reader - ADC clock source select"]
pub type PLLCLK_TO_ADC_R = crate::BitReader;
#[doc = "Field `PLLCLK_TO_ADC` writer - ADC clock source select"]
pub type PLLCLK_TO_ADC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT_SEL2` reader - Clock output select2"]
pub type CLKOUT_SEL2_R = crate::FieldReader;
#[doc = "Field `CLKOUT_SEL2` writer - Clock output select2"]
pub type CLKOUT_SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLKOUTDIV2` reader - Clock output division2"]
pub type CLKOUTDIV2_R = crate::FieldReader;
#[doc = "Field `CLKOUTDIV2` writer - Clock output division2"]
pub type CLKOUTDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    #[doc = "Bit 15 - ADC clock source select"]
    #[inline(always)]
    pub fn pllclk_to_adc(&self) -> PLLCLK_TO_ADC_R {
        PLLCLK_TO_ADC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Clock output select2"]
    #[inline(always)]
    pub fn clkout_sel2(&self) -> CLKOUT_SEL2_R {
        CLKOUT_SEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Clock output division2"]
    #[inline(always)]
    pub fn clkoutdiv2(&self) -> CLKOUTDIV2_R {
        CLKOUTDIV2_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC1")
            .field("hickcal_key", &self.hickcal_key())
            .field("hickdiv", &self.hickdiv())
            .field("hick_to_usb", &self.hick_to_usb())
            .field("hick_to_sclk", &self.hick_to_sclk())
            .field("pllclk_to_adc", &self.pllclk_to_adc())
            .field("clkout_sel2", &self.clkout_sel2())
            .field("clkoutdiv2", &self.clkoutdiv2())
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
    #[doc = "Bit 15 - ADC clock source select"]
    #[inline(always)]
    pub fn pllclk_to_adc(&mut self) -> PLLCLK_TO_ADC_W<'_, MISC1_SPEC> {
        PLLCLK_TO_ADC_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Clock output select2"]
    #[inline(always)]
    pub fn clkout_sel2(&mut self) -> CLKOUT_SEL2_W<'_, MISC1_SPEC> {
        CLKOUT_SEL2_W::new(self, 16)
    }
    #[doc = "Bits 28:31 - Clock output division2"]
    #[inline(always)]
    pub fn clkoutdiv2(&mut self) -> CLKOUTDIV2_W<'_, MISC1_SPEC> {
        CLKOUTDIV2_W::new(self, 28)
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
