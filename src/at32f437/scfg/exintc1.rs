#[doc = "Register `EXINTC1` reader"]
pub type R = crate::R<EXINTC1_SPEC>;
#[doc = "Register `EXINTC1` writer"]
pub type W = crate::W<EXINTC1_SPEC>;
#[doc = "External interrupt line %s\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXINT_SOURCE_PORT_A {
    #[doc = "0: GPIOA pin %s is selected as the source input for EXINT%s"]
    GpioA = 0,
    #[doc = "1: GPIOB pin %s is selected as the source input for EXINT%s"]
    GpioB = 1,
    #[doc = "2: GPIOC pin %s is selected as the source input for EXINT%s"]
    GpioC = 2,
    #[doc = "3: GPIOD pin %s is selected as the source input for EXINT%s"]
    GpioD = 3,
    #[doc = "4: GPIOE pin %s is selected as the source input for EXINT%s"]
    GpioE = 4,
    #[doc = "5: GPIOF pin %s is selected as the source input for EXINT%s"]
    GpioF = 5,
    #[doc = "6: GPIOG pin %s is selected as the source input for EXINT%s"]
    GpioG = 6,
    #[doc = "7: GPIOH pin %s is selected as the source input for EXINT%s"]
    GpioH = 7,
}
impl From<EXINT_SOURCE_PORT_A> for u8 {
    #[inline(always)]
    fn from(variant: EXINT_SOURCE_PORT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXINT_SOURCE_PORT_A {
    type Ux = u8;
}
impl crate::IsEnum for EXINT_SOURCE_PORT_A {}
#[doc = "Field `EXINT(0-3)` reader - External interrupt line %s"]
pub type EXINT_R = crate::FieldReader<EXINT_SOURCE_PORT_A>;
impl EXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXINT_SOURCE_PORT_A> {
        match self.bits {
            0 => Some(EXINT_SOURCE_PORT_A::GpioA),
            1 => Some(EXINT_SOURCE_PORT_A::GpioB),
            2 => Some(EXINT_SOURCE_PORT_A::GpioC),
            3 => Some(EXINT_SOURCE_PORT_A::GpioD),
            4 => Some(EXINT_SOURCE_PORT_A::GpioE),
            5 => Some(EXINT_SOURCE_PORT_A::GpioF),
            6 => Some(EXINT_SOURCE_PORT_A::GpioG),
            7 => Some(EXINT_SOURCE_PORT_A::GpioH),
            _ => None,
        }
    }
    #[doc = "GPIOA pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_a(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioA
    }
    #[doc = "GPIOB pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_b(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioB
    }
    #[doc = "GPIOC pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_c(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioC
    }
    #[doc = "GPIOD pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_d(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioD
    }
    #[doc = "GPIOE pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_e(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioE
    }
    #[doc = "GPIOF pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_f(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioF
    }
    #[doc = "GPIOG pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_g(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioG
    }
    #[doc = "GPIOH pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_h(&self) -> bool {
        *self == EXINT_SOURCE_PORT_A::GpioH
    }
}
#[doc = "Field `EXINT(0-3)` writer - External interrupt line %s"]
pub type EXINT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXINT_SOURCE_PORT_A>;
impl<'a, REG> EXINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_a(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioA)
    }
    #[doc = "GPIOB pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_b(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioB)
    }
    #[doc = "GPIOC pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_c(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioC)
    }
    #[doc = "GPIOD pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_d(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioD)
    }
    #[doc = "GPIOE pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_e(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioE)
    }
    #[doc = "GPIOF pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_f(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioF)
    }
    #[doc = "GPIOG pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_g(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioG)
    }
    #[doc = "GPIOH pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_h(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_A::GpioH)
    }
}
impl R {
    #[doc = "External interrupt line (0-3)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT0` field.</div>"]
    #[inline(always)]
    pub fn exint(&self, n: u8) -> EXINT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "External interrupt line (0-3)"]
    #[inline(always)]
    pub fn exint_iter(&self) -> impl Iterator<Item = EXINT_R> + '_ {
        (0..4).map(move |n| EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - External interrupt line 0"]
    #[inline(always)]
    pub fn exint0(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External interrupt line 1"]
    #[inline(always)]
    pub fn exint1(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External interrupt line 2"]
    #[inline(always)]
    pub fn exint2(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External interrupt line 3"]
    #[inline(always)]
    pub fn exint3(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC1")
            .field("exint0", &self.exint0())
            .field("exint1", &self.exint1())
            .field("exint2", &self.exint2())
            .field("exint3", &self.exint3())
            .finish()
    }
}
impl W {
    #[doc = "External interrupt line (0-3)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT0` field.</div>"]
    #[inline(always)]
    pub fn exint(&mut self, n: u8) -> EXINT_W<'_, EXINTC1_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - External interrupt line 0"]
    #[inline(always)]
    pub fn exint0(&mut self) -> EXINT_W<'_, EXINTC1_SPEC> {
        EXINT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External interrupt line 1"]
    #[inline(always)]
    pub fn exint1(&mut self) -> EXINT_W<'_, EXINTC1_SPEC> {
        EXINT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External interrupt line 2"]
    #[inline(always)]
    pub fn exint2(&mut self) -> EXINT_W<'_, EXINTC1_SPEC> {
        EXINT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External interrupt line 3"]
    #[inline(always)]
    pub fn exint3(&mut self) -> EXINT_W<'_, EXINTC1_SPEC> {
        EXINT_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC1_SPEC;
impl crate::RegisterSpec for EXINTC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc1::R`](R) reader structure"]
impl crate::Readable for EXINTC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc1::W`](W) writer structure"]
impl crate::Writable for EXINTC1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC1 to value 0"]
impl crate::Resettable for EXINTC1_SPEC {}
