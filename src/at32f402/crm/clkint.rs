#[doc = "Register `CLKINT` reader"]
pub type R = crate::R<CLKINT_SPEC>;
#[doc = "Register `CLKINT` writer"]
pub type W = crate::W<CLKINT_SPEC>;
#[doc = "LICK ready interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LICKSTBLF_A {
    #[doc = "0: LICK is not ready"]
    NotReady = 0,
    #[doc = "1: LICK is ready"]
    Ready = 1,
}
impl From<LICKSTBLF_A> for bool {
    #[inline(always)]
    fn from(variant: LICKSTBLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LICKSTBLF` reader - LICK ready interrupt flag"]
pub type LICKSTBLF_R = crate::BitReader<LICKSTBLF_A>;
impl LICKSTBLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LICKSTBLF_A {
        match self.bits {
            false => LICKSTBLF_A::NotReady,
            true => LICKSTBLF_A::Ready,
        }
    }
    #[doc = "LICK is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LICKSTBLF_A::NotReady
    }
    #[doc = "LICK is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LICKSTBLF_A::Ready
    }
}
#[doc = "LEXT ready interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LEXTSTBLF_A {
    #[doc = "0: LEXT is not ready"]
    NotReady = 0,
    #[doc = "1: LEXT is ready"]
    Ready = 1,
}
impl From<LEXTSTBLF_A> for bool {
    #[inline(always)]
    fn from(variant: LEXTSTBLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEXTSTBLF` reader - LEXT ready interrupt flag"]
pub type LEXTSTBLF_R = crate::BitReader<LEXTSTBLF_A>;
impl LEXTSTBLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LEXTSTBLF_A {
        match self.bits {
            false => LEXTSTBLF_A::NotReady,
            true => LEXTSTBLF_A::Ready,
        }
    }
    #[doc = "LEXT is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LEXTSTBLF_A::NotReady
    }
    #[doc = "LEXT is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LEXTSTBLF_A::Ready
    }
}
#[doc = "HICK ready interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HICKSTBLF_A {
    #[doc = "0: HICK is not ready"]
    NotReady = 0,
    #[doc = "1: HICK is ready"]
    Ready = 1,
}
impl From<HICKSTBLF_A> for bool {
    #[inline(always)]
    fn from(variant: HICKSTBLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HICKSTBLF` reader - HICK ready interrupt flag"]
pub type HICKSTBLF_R = crate::BitReader<HICKSTBLF_A>;
impl HICKSTBLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HICKSTBLF_A {
        match self.bits {
            false => HICKSTBLF_A::NotReady,
            true => HICKSTBLF_A::Ready,
        }
    }
    #[doc = "HICK is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HICKSTBLF_A::NotReady
    }
    #[doc = "HICK is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HICKSTBLF_A::Ready
    }
}
#[doc = "HEXT ready interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEXTSTBLF_A {
    #[doc = "0: HEXT is not ready"]
    NotReady = 0,
    #[doc = "1: HEXT is ready"]
    Ready = 1,
}
impl From<HEXTSTBLF_A> for bool {
    #[inline(always)]
    fn from(variant: HEXTSTBLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HEXTSTBLF` reader - HEXT ready interrupt flag"]
pub type HEXTSTBLF_R = crate::BitReader<HEXTSTBLF_A>;
impl HEXTSTBLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HEXTSTBLF_A {
        match self.bits {
            false => HEXTSTBLF_A::NotReady,
            true => HEXTSTBLF_A::Ready,
        }
    }
    #[doc = "HEXT is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == HEXTSTBLF_A::NotReady
    }
    #[doc = "HEXT is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == HEXTSTBLF_A::Ready
    }
}
#[doc = "PLL ready interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLSTBLF_A {
    #[doc = "0: PLL is not ready"]
    NotReady = 0,
    #[doc = "1: PLL is ready"]
    Ready = 1,
}
impl From<PLLSTBLF_A> for bool {
    #[inline(always)]
    fn from(variant: PLLSTBLF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLSTBLF` reader - PLL ready interrupt flag"]
pub type PLLSTBLF_R = crate::BitReader<PLLSTBLF_A>;
impl PLLSTBLF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PLLSTBLF_A {
        match self.bits {
            false => PLLSTBLF_A::NotReady,
            true => PLLSTBLF_A::Ready,
        }
    }
    #[doc = "PLL is not ready"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == PLLSTBLF_A::NotReady
    }
    #[doc = "PLL is ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == PLLSTBLF_A::Ready
    }
}
#[doc = "Clock failure detection interrupt flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFDF_A {
    #[doc = "0: No clock failure"]
    NoFailure = 0,
    #[doc = "1: Clock failure detected"]
    Failure = 1,
}
impl From<CFDF_A> for bool {
    #[inline(always)]
    fn from(variant: CFDF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFDF` reader - Clock failure detection interrupt flag"]
pub type CFDF_R = crate::BitReader<CFDF_A>;
impl CFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CFDF_A {
        match self.bits {
            false => CFDF_A::NoFailure,
            true => CFDF_A::Failure,
        }
    }
    #[doc = "No clock failure"]
    #[inline(always)]
    pub fn is_no_failure(&self) -> bool {
        *self == CFDF_A::NoFailure
    }
    #[doc = "Clock failure detected"]
    #[inline(always)]
    pub fn is_failure(&self) -> bool {
        *self == CFDF_A::Failure
    }
}
#[doc = "LICK ready interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LICKSTBLIEN_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<LICKSTBLIEN_A> for bool {
    #[inline(always)]
    fn from(variant: LICKSTBLIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LICKSTBLIEN` reader - LICK ready interrupt enable"]
pub type LICKSTBLIEN_R = crate::BitReader<LICKSTBLIEN_A>;
impl LICKSTBLIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LICKSTBLIEN_A {
        match self.bits {
            false => LICKSTBLIEN_A::Disabled,
            true => LICKSTBLIEN_A::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LICKSTBLIEN_A::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LICKSTBLIEN_A::Enabled
    }
}
#[doc = "Field `LICKSTBLIEN` writer - LICK ready interrupt enable"]
pub type LICKSTBLIEN_W<'a, REG> = crate::BitWriter<'a, REG, LICKSTBLIEN_A>;
impl<'a, REG> LICKSTBLIEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LICKSTBLIEN_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(LICKSTBLIEN_A::Enabled)
    }
}
#[doc = "Field `LEXTSTBLIEN` reader - LEXT ready interrupt enable"]
pub use LICKSTBLIEN_R as LEXTSTBLIEN_R;
#[doc = "Field `HICKSTBLIEN` reader - HICK ready interrupt enable"]
pub use LICKSTBLIEN_R as HICKSTBLIEN_R;
#[doc = "Field `HEXTSTBLIEN` reader - HEXT ready interrupt enable"]
pub use LICKSTBLIEN_R as HEXTSTBLIEN_R;
#[doc = "Field `PLLSTBLIEN` reader - PLL ready interrupt enable"]
pub use LICKSTBLIEN_R as PLLSTBLIEN_R;
#[doc = "Field `LEXTSTBLIEN` writer - LEXT ready interrupt enable"]
pub use LICKSTBLIEN_W as LEXTSTBLIEN_W;
#[doc = "Field `HICKSTBLIEN` writer - HICK ready interrupt enable"]
pub use LICKSTBLIEN_W as HICKSTBLIEN_W;
#[doc = "Field `HEXTSTBLIEN` writer - HEXT ready interrupt enable"]
pub use LICKSTBLIEN_W as HEXTSTBLIEN_W;
#[doc = "Field `PLLSTBLIEN` writer - PLL ready interrupt enable"]
pub use LICKSTBLIEN_W as PLLSTBLIEN_W;
#[doc = "LICK ready interrupt clear\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LICKSTBLFCW_A {
    #[doc = "0: Clear flag"]
    Clear = 0,
}
impl From<LICKSTBLFCW_A> for bool {
    #[inline(always)]
    fn from(variant: LICKSTBLFCW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LICKSTBLFC` writer - LICK ready interrupt clear"]
pub type LICKSTBLFC_W<'a, REG> = crate::BitWriter1C<'a, REG, LICKSTBLFCW_A>;
impl<'a, REG> LICKSTBLFC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(LICKSTBLFCW_A::Clear)
    }
}
#[doc = "Field `LEXTSTBLFC` writer - LEXT ready interrupt clear"]
pub use LICKSTBLFC_W as LEXTSTBLFC_W;
#[doc = "Field `HICKSTBLFC` writer - HICK ready interrupt clear"]
pub use LICKSTBLFC_W as HICKSTBLFC_W;
#[doc = "Field `HEXTSTBLFC` writer - HEXT ready interrupt clear"]
pub use LICKSTBLFC_W as HEXTSTBLFC_W;
#[doc = "Field `PLLSTBLFC` writer - PLL ready interrupt clear"]
pub use LICKSTBLFC_W as PLLSTBLFC_W;
#[doc = "Field `CFDFC` writer - Clock failure detection interrupt clear"]
pub use LICKSTBLFC_W as CFDFC_W;
impl R {
    #[doc = "Bit 0 - LICK ready interrupt flag"]
    #[inline(always)]
    pub fn lickstblf(&self) -> LICKSTBLF_R {
        LICKSTBLF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LEXT ready interrupt flag"]
    #[inline(always)]
    pub fn lextstblf(&self) -> LEXTSTBLF_R {
        LEXTSTBLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HICK ready interrupt flag"]
    #[inline(always)]
    pub fn hickstblf(&self) -> HICKSTBLF_R {
        HICKSTBLF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HEXT ready interrupt flag"]
    #[inline(always)]
    pub fn hextstblf(&self) -> HEXTSTBLF_R {
        HEXTSTBLF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PLL ready interrupt flag"]
    #[inline(always)]
    pub fn pllstblf(&self) -> PLLSTBLF_R {
        PLLSTBLF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock failure detection interrupt flag"]
    #[inline(always)]
    pub fn cfdf(&self) -> CFDF_R {
        CFDF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LICK ready interrupt enable"]
    #[inline(always)]
    pub fn lickstblien(&self) -> LICKSTBLIEN_R {
        LICKSTBLIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LEXT ready interrupt enable"]
    #[inline(always)]
    pub fn lextstblien(&self) -> LEXTSTBLIEN_R {
        LEXTSTBLIEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HICK ready interrupt enable"]
    #[inline(always)]
    pub fn hickstblien(&self) -> HICKSTBLIEN_R {
        HICKSTBLIEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HEXT ready interrupt enable"]
    #[inline(always)]
    pub fn hextstblien(&self) -> HEXTSTBLIEN_R {
        HEXTSTBLIEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllstblien(&self) -> PLLSTBLIEN_R {
        PLLSTBLIEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKINT")
            .field("lickstblf", &self.lickstblf())
            .field("lextstblf", &self.lextstblf())
            .field("hickstblf", &self.hickstblf())
            .field("hextstblf", &self.hextstblf())
            .field("pllstblf", &self.pllstblf())
            .field("cfdf", &self.cfdf())
            .field("lickstblien", &self.lickstblien())
            .field("lextstblien", &self.lextstblien())
            .field("hickstblien", &self.hickstblien())
            .field("hextstblien", &self.hextstblien())
            .field("pllstblien", &self.pllstblien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - LICK ready interrupt enable"]
    #[inline(always)]
    pub fn lickstblien(&mut self) -> LICKSTBLIEN_W<'_, CLKINT_SPEC> {
        LICKSTBLIEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - LEXT ready interrupt enable"]
    #[inline(always)]
    pub fn lextstblien(&mut self) -> LEXTSTBLIEN_W<'_, CLKINT_SPEC> {
        LEXTSTBLIEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - HICK ready interrupt enable"]
    #[inline(always)]
    pub fn hickstblien(&mut self) -> HICKSTBLIEN_W<'_, CLKINT_SPEC> {
        HICKSTBLIEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - HEXT ready interrupt enable"]
    #[inline(always)]
    pub fn hextstblien(&mut self) -> HEXTSTBLIEN_W<'_, CLKINT_SPEC> {
        HEXTSTBLIEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - PLL ready interrupt enable"]
    #[inline(always)]
    pub fn pllstblien(&mut self) -> PLLSTBLIEN_W<'_, CLKINT_SPEC> {
        PLLSTBLIEN_W::new(self, 12)
    }
    #[doc = "Bit 16 - LICK ready interrupt clear"]
    #[inline(always)]
    pub fn lickstblfc(&mut self) -> LICKSTBLFC_W<'_, CLKINT_SPEC> {
        LICKSTBLFC_W::new(self, 16)
    }
    #[doc = "Bit 17 - LEXT ready interrupt clear"]
    #[inline(always)]
    pub fn lextstblfc(&mut self) -> LEXTSTBLFC_W<'_, CLKINT_SPEC> {
        LEXTSTBLFC_W::new(self, 17)
    }
    #[doc = "Bit 18 - HICK ready interrupt clear"]
    #[inline(always)]
    pub fn hickstblfc(&mut self) -> HICKSTBLFC_W<'_, CLKINT_SPEC> {
        HICKSTBLFC_W::new(self, 18)
    }
    #[doc = "Bit 19 - HEXT ready interrupt clear"]
    #[inline(always)]
    pub fn hextstblfc(&mut self) -> HEXTSTBLFC_W<'_, CLKINT_SPEC> {
        HEXTSTBLFC_W::new(self, 19)
    }
    #[doc = "Bit 20 - PLL ready interrupt clear"]
    #[inline(always)]
    pub fn pllstblfc(&mut self) -> PLLSTBLFC_W<'_, CLKINT_SPEC> {
        PLLSTBLFC_W::new(self, 20)
    }
    #[doc = "Bit 23 - Clock failure detection interrupt clear"]
    #[inline(always)]
    pub fn cfdfc(&mut self) -> CFDFC_W<'_, CLKINT_SPEC> {
        CFDFC_W::new(self, 23)
    }
}
#[doc = "Clock interrupt register (CRM_CLKINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`clkint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLKINT_SPEC;
impl crate::RegisterSpec for CLKINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkint::R`](R) reader structure"]
impl crate::Readable for CLKINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clkint::W`](W) writer structure"]
impl crate::Writable for CLKINT_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x009f_0000;
}
#[doc = "`reset()` method sets CLKINT to value 0"]
impl crate::Resettable for CLKINT_SPEC {}
