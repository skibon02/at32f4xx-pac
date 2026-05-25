#[doc = "Register `EXINTC2` reader"]
pub type R = crate::R<EXINTC2_SPEC>;
#[doc = "Register `EXINTC2` writer"]
pub type W = crate::W<EXINTC2_SPEC>;
#[doc = "External interrupt line %s\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXINT_SOURCE_PORT_NO_H_A {
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
}
impl From<EXINT_SOURCE_PORT_NO_H_A> for u8 {
    #[inline(always)]
    fn from(variant: EXINT_SOURCE_PORT_NO_H_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXINT_SOURCE_PORT_NO_H_A {
    type Ux = u8;
}
impl crate::IsEnum for EXINT_SOURCE_PORT_NO_H_A {}
#[doc = "Field `EXINT(4-7)` reader - External interrupt line %s"]
pub type EXINT_R = crate::FieldReader<EXINT_SOURCE_PORT_NO_H_A>;
impl EXINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXINT_SOURCE_PORT_NO_H_A> {
        match self.bits {
            0 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioA),
            1 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioB),
            2 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioC),
            3 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioD),
            4 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioE),
            5 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioF),
            6 => Some(EXINT_SOURCE_PORT_NO_H_A::GpioG),
            _ => None,
        }
    }
    #[doc = "GPIOA pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_a(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioA
    }
    #[doc = "GPIOB pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_b(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioB
    }
    #[doc = "GPIOC pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_c(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioC
    }
    #[doc = "GPIOD pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_d(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioD
    }
    #[doc = "GPIOE pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_e(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioE
    }
    #[doc = "GPIOF pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_f(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioF
    }
    #[doc = "GPIOG pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn is_gpio_g(&self) -> bool {
        *self == EXINT_SOURCE_PORT_NO_H_A::GpioG
    }
}
#[doc = "Field `EXINT(4-7)` writer - External interrupt line %s"]
pub type EXINT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EXINT_SOURCE_PORT_NO_H_A>;
impl<'a, REG> EXINT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIOA pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_a(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioA)
    }
    #[doc = "GPIOB pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_b(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioB)
    }
    #[doc = "GPIOC pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_c(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioC)
    }
    #[doc = "GPIOD pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_d(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioD)
    }
    #[doc = "GPIOE pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_e(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioE)
    }
    #[doc = "GPIOF pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_f(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioF)
    }
    #[doc = "GPIOG pin %s is selected as the source input for EXINT%s"]
    #[inline(always)]
    pub fn gpio_g(self) -> &'a mut crate::W<REG> {
        self.variant(EXINT_SOURCE_PORT_NO_H_A::GpioG)
    }
}
impl R {
    #[doc = "External interrupt line (4-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT4` field.</div>"]
    #[inline(always)]
    pub fn exint(&self, n: u8) -> EXINT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "External interrupt line (4-7)"]
    #[inline(always)]
    pub fn exint_iter(&self) -> impl Iterator<Item = EXINT_R> + '_ {
        (0..4).map(move |n| EXINT_R::new(((self.bits >> (n * 4)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - External interrupt line 4"]
    #[inline(always)]
    pub fn exint4(&self) -> EXINT_R {
        EXINT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - External interrupt line 5"]
    #[inline(always)]
    pub fn exint5(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - External interrupt line 6"]
    #[inline(always)]
    pub fn exint6(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - External interrupt line 7"]
    #[inline(always)]
    pub fn exint7(&self) -> EXINT_R {
        EXINT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXINTC2")
            .field("exint4", &self.exint4())
            .field("exint5", &self.exint5())
            .field("exint6", &self.exint6())
            .field("exint7", &self.exint7())
            .finish()
    }
}
impl W {
    #[doc = "External interrupt line (4-7)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `EXINT4` field.</div>"]
    #[inline(always)]
    pub fn exint(&mut self, n: u8) -> EXINT_W<'_, EXINTC2_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        EXINT_W::new(self, n * 4)
    }
    #[doc = "Bits 0:3 - External interrupt line 4"]
    #[inline(always)]
    pub fn exint4(&mut self) -> EXINT_W<'_, EXINTC2_SPEC> {
        EXINT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - External interrupt line 5"]
    #[inline(always)]
    pub fn exint5(&mut self) -> EXINT_W<'_, EXINTC2_SPEC> {
        EXINT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - External interrupt line 6"]
    #[inline(always)]
    pub fn exint6(&mut self) -> EXINT_W<'_, EXINTC2_SPEC> {
        EXINT_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - External interrupt line 7"]
    #[inline(always)]
    pub fn exint7(&mut self) -> EXINT_W<'_, EXINTC2_SPEC> {
        EXINT_W::new(self, 12)
    }
}
#[doc = "external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`exintc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exintc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXINTC2_SPEC;
impl crate::RegisterSpec for EXINTC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exintc2::R`](R) reader structure"]
impl crate::Readable for EXINTC2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exintc2::W`](W) writer structure"]
impl crate::Writable for EXINTC2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXINTC2 to value 0"]
impl crate::Resettable for EXINTC2_SPEC {}
