#[doc = "Register `MISC2` reader"]
pub type R = crate::R<MISC2_SPEC>;
#[doc = "Register `MISC2` writer"]
pub type W = crate::W<MISC2_SPEC>;
#[doc = "AUTO_STEP_EN\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AUTO_STEP_EN_A {
    #[doc = "0: Auto step mode disabled"]
    Disabled = 0,
    #[doc = "3: Auto step mode enabled. When AHBDIV or SCLKSEL is modified, the auto step-by-step system clock switch is activated automatically."]
    Enabled = 3,
}
impl From<AUTO_STEP_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: AUTO_STEP_EN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AUTO_STEP_EN_A {
    type Ux = u8;
}
impl crate::IsEnum for AUTO_STEP_EN_A {}
#[doc = "Field `AUTO_STEP_EN` reader - AUTO_STEP_EN"]
pub type AUTO_STEP_EN_R = crate::FieldReader<AUTO_STEP_EN_A>;
impl AUTO_STEP_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AUTO_STEP_EN_A> {
        match self.bits {
            0 => Some(AUTO_STEP_EN_A::Disabled),
            3 => Some(AUTO_STEP_EN_A::Enabled),
            _ => None,
        }
    }
    #[doc = "Auto step mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTO_STEP_EN_A::Disabled
    }
    #[doc = "Auto step mode enabled. When AHBDIV or SCLKSEL is modified, the auto step-by-step system clock switch is activated automatically."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTO_STEP_EN_A::Enabled
    }
}
#[doc = "Field `AUTO_STEP_EN` writer - AUTO_STEP_EN"]
pub type AUTO_STEP_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, AUTO_STEP_EN_A>;
impl<'a, REG> AUTO_STEP_EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Auto step mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTO_STEP_EN_A::Disabled)
    }
    #[doc = "Auto step mode enabled. When AHBDIV or SCLKSEL is modified, the auto step-by-step system clock switch is activated automatically."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AUTO_STEP_EN_A::Enabled)
    }
}
#[doc = "USB division\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USBDIV_A {
    #[doc = "0: USB clock is divided by 1.5"]
    Div1_5 = 0,
    #[doc = "1: USB clock is not divided"]
    Div1 = 1,
    #[doc = "2: USB clock is divided by 2.5"]
    Div2_5 = 2,
    #[doc = "3: USB clock is divided by 2"]
    Div2 = 3,
    #[doc = "4: USB clock is divided by 3.5"]
    Div3_5 = 4,
    #[doc = "5: USB clock is divided by 3"]
    Div3 = 5,
    #[doc = "6: USB clock is divided by 4.5"]
    Div4_5 = 6,
    #[doc = "7: USB clock is divided by 4"]
    Div4 = 7,
    #[doc = "8: USB clock is divided by 5.5"]
    Div5_5 = 8,
    #[doc = "9: USB clock is divided by 5"]
    Div5 = 9,
    #[doc = "10: USB clock is divided by 6.5"]
    Div6_5 = 10,
    #[doc = "11: USB clock is divided by 6"]
    Div6 = 11,
    #[doc = "12: USB clock is divided by 7"]
    Div7 = 12,
}
impl From<USBDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: USBDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for USBDIV_A {
    type Ux = u8;
}
impl crate::IsEnum for USBDIV_A {}
#[doc = "Field `USBDIV` reader - USB division"]
pub type USBDIV_R = crate::FieldReader<USBDIV_A>;
impl USBDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<USBDIV_A> {
        match self.bits {
            0 => Some(USBDIV_A::Div1_5),
            1 => Some(USBDIV_A::Div1),
            2 => Some(USBDIV_A::Div2_5),
            3 => Some(USBDIV_A::Div2),
            4 => Some(USBDIV_A::Div3_5),
            5 => Some(USBDIV_A::Div3),
            6 => Some(USBDIV_A::Div4_5),
            7 => Some(USBDIV_A::Div4),
            8 => Some(USBDIV_A::Div5_5),
            9 => Some(USBDIV_A::Div5),
            10 => Some(USBDIV_A::Div6_5),
            11 => Some(USBDIV_A::Div6),
            12 => Some(USBDIV_A::Div7),
            _ => None,
        }
    }
    #[doc = "USB clock is divided by 1.5"]
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == USBDIV_A::Div1_5
    }
    #[doc = "USB clock is not divided"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == USBDIV_A::Div1
    }
    #[doc = "USB clock is divided by 2.5"]
    #[inline(always)]
    pub fn is_div2_5(&self) -> bool {
        *self == USBDIV_A::Div2_5
    }
    #[doc = "USB clock is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == USBDIV_A::Div2
    }
    #[doc = "USB clock is divided by 3.5"]
    #[inline(always)]
    pub fn is_div3_5(&self) -> bool {
        *self == USBDIV_A::Div3_5
    }
    #[doc = "USB clock is divided by 3"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == USBDIV_A::Div3
    }
    #[doc = "USB clock is divided by 4.5"]
    #[inline(always)]
    pub fn is_div4_5(&self) -> bool {
        *self == USBDIV_A::Div4_5
    }
    #[doc = "USB clock is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == USBDIV_A::Div4
    }
    #[doc = "USB clock is divided by 5.5"]
    #[inline(always)]
    pub fn is_div5_5(&self) -> bool {
        *self == USBDIV_A::Div5_5
    }
    #[doc = "USB clock is divided by 5"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == USBDIV_A::Div5
    }
    #[doc = "USB clock is divided by 6.5"]
    #[inline(always)]
    pub fn is_div6_5(&self) -> bool {
        *self == USBDIV_A::Div6_5
    }
    #[doc = "USB clock is divided by 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == USBDIV_A::Div6
    }
    #[doc = "USB clock is divided by 7"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == USBDIV_A::Div7
    }
}
#[doc = "Field `USBDIV` writer - USB division"]
pub type USBDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, USBDIV_A>;
impl<'a, REG> USBDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB clock is divided by 1.5"]
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div1_5)
    }
    #[doc = "USB clock is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div1)
    }
    #[doc = "USB clock is divided by 2.5"]
    #[inline(always)]
    pub fn div2_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div2_5)
    }
    #[doc = "USB clock is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div2)
    }
    #[doc = "USB clock is divided by 3.5"]
    #[inline(always)]
    pub fn div3_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div3_5)
    }
    #[doc = "USB clock is divided by 3"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div3)
    }
    #[doc = "USB clock is divided by 4.5"]
    #[inline(always)]
    pub fn div4_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div4_5)
    }
    #[doc = "USB clock is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div4)
    }
    #[doc = "USB clock is divided by 5.5"]
    #[inline(always)]
    pub fn div5_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div5_5)
    }
    #[doc = "USB clock is divided by 5"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div5)
    }
    #[doc = "USB clock is divided by 6.5"]
    #[inline(always)]
    pub fn div6_5(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div6_5)
    }
    #[doc = "USB clock is divided by 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div6)
    }
    #[doc = "USB clock is divided by 7"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(USBDIV_A::Div7)
    }
}
#[doc = "Field `HICK_TO_SCLK_DIV` reader - HICK as system clock frequency division"]
pub type HICK_TO_SCLK_DIV_R = crate::FieldReader;
#[doc = "Field `HICK_TO_SCLK_DIV` writer - HICK as system clock frequency division"]
pub type HICK_TO_SCLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HEXT_TO_SCLK_DIV` reader - HEXT as system clock frequency division"]
pub type HEXT_TO_SCLK_DIV_R = crate::FieldReader;
#[doc = "Field `HEXT_TO_SCLK_DIV` writer - HEXT as system clock frequency division"]
pub type HEXT_TO_SCLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&self) -> AUTO_STEP_EN_R {
        AUTO_STEP_EN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:15 - USB division"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - HICK as system clock frequency division"]
    #[inline(always)]
    pub fn hick_to_sclk_div(&self) -> HICK_TO_SCLK_DIV_R {
        HICK_TO_SCLK_DIV_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - HEXT as system clock frequency division"]
    #[inline(always)]
    pub fn hext_to_sclk_div(&self) -> HEXT_TO_SCLK_DIV_R {
        HEXT_TO_SCLK_DIV_R::new(((self.bits >> 19) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC2")
            .field("auto_step_en", &self.auto_step_en())
            .field("usbdiv", &self.usbdiv())
            .field("hick_to_sclk_div", &self.hick_to_sclk_div())
            .field("hext_to_sclk_div", &self.hext_to_sclk_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:5 - AUTO_STEP_EN"]
    #[inline(always)]
    pub fn auto_step_en(&mut self) -> AUTO_STEP_EN_W<'_, MISC2_SPEC> {
        AUTO_STEP_EN_W::new(self, 4)
    }
    #[doc = "Bits 12:15 - USB division"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W<'_, MISC2_SPEC> {
        USBDIV_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - HICK as system clock frequency division"]
    #[inline(always)]
    pub fn hick_to_sclk_div(&mut self) -> HICK_TO_SCLK_DIV_W<'_, MISC2_SPEC> {
        HICK_TO_SCLK_DIV_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - HEXT as system clock frequency division"]
    #[inline(always)]
    pub fn hext_to_sclk_div(&mut self) -> HEXT_TO_SCLK_DIV_W<'_, MISC2_SPEC> {
        HEXT_TO_SCLK_DIV_W::new(self, 19)
    }
}
#[doc = "Miscellaneous register2\n\nYou can [`read`](crate::Reg::read) this register and get [`misc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC2_SPEC;
impl crate::RegisterSpec for MISC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc2::R`](R) reader structure"]
impl crate::Readable for MISC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc2::W`](W) writer structure"]
impl crate::Writable for MISC2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC2 to value 0x0d"]
impl crate::Resettable for MISC2_SPEC {
    const RESET_VALUE: u32 = 0x0d;
}
