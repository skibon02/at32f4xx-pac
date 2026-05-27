#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `VRSEL` reader - Voltage regulator state select when deepsleep mode"]
pub type VRSEL_R = crate::BitReader;
#[doc = "Field `VRSEL` writer - Voltage regulator state select when deepsleep mode"]
pub type VRSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low power mode select when Cortex-M4F sleepdeep\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSEL_A {
    #[doc = "0: Enter deepsleep mode when the CPU enters deepsleep mode"]
    EnterDeepsleep = 0,
    #[doc = "1: Enter standby mode when the CPU enters deepsleep mode"]
    EnterStandby = 1,
}
impl From<LPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: LPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPSEL` reader - Low power mode select when Cortex-M4F sleepdeep"]
pub type LPSEL_R = crate::BitReader<LPSEL_A>;
impl LPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPSEL_A {
        match self.bits {
            false => LPSEL_A::EnterDeepsleep,
            true => LPSEL_A::EnterStandby,
        }
    }
    #[doc = "Enter deepsleep mode when the CPU enters deepsleep mode"]
    #[inline(always)]
    pub fn is_enter_deepsleep(&self) -> bool {
        *self == LPSEL_A::EnterDeepsleep
    }
    #[doc = "Enter standby mode when the CPU enters deepsleep mode"]
    #[inline(always)]
    pub fn is_enter_standby(&self) -> bool {
        *self == LPSEL_A::EnterStandby
    }
}
#[doc = "Field `LPSEL` writer - Low power mode select when Cortex-M4F sleepdeep"]
pub type LPSEL_W<'a, REG> = crate::BitWriter<'a, REG, LPSEL_A>;
impl<'a, REG> LPSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enter deepsleep mode when the CPU enters deepsleep mode"]
    #[inline(always)]
    pub fn enter_deepsleep(self) -> &'a mut crate::W<REG> {
        self.variant(LPSEL_A::EnterDeepsleep)
    }
    #[doc = "Enter standby mode when the CPU enters deepsleep mode"]
    #[inline(always)]
    pub fn enter_standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPSEL_A::EnterStandby)
    }
}
#[doc = "Clear SWEF flag\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLSWEFW_A {
    #[doc = "1: Clear the flag"]
    Clear = 1,
}
impl From<CLSWEFW_A> for bool {
    #[inline(always)]
    fn from(variant: CLSWEFW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLSWEF` writer - Clear SWEF flag"]
pub type CLSWEF_W<'a, REG> = crate::BitWriter1C<'a, REG, CLSWEFW_A>;
impl<'a, REG> CLSWEF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear the flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CLSWEFW_A::Clear)
    }
}
#[doc = "Field `CLSEF` writer - Clear SEF flag"]
pub use CLSWEF_W as CLSEF_W;
#[doc = "Power voltage monitoring enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PVMEN_A {
    #[doc = "0: Power voltage monitoring disabled"]
    Disabled = 0,
    #[doc = "1: Power voltage monitoring enabled"]
    Enabled = 1,
}
impl From<PVMEN_A> for bool {
    #[inline(always)]
    fn from(variant: PVMEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PVMEN` reader - Power voltage monitoring enable"]
pub type PVMEN_R = crate::BitReader<PVMEN_A>;
impl PVMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PVMEN_A {
        match self.bits {
            false => PVMEN_A::Disabled,
            true => PVMEN_A::Enabled,
        }
    }
    #[doc = "Power voltage monitoring disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PVMEN_A::Disabled
    }
    #[doc = "Power voltage monitoring enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PVMEN_A::Enabled
    }
}
#[doc = "Field `PVMEN` writer - Power voltage monitoring enable"]
pub type PVMEN_W<'a, REG> = crate::BitWriter<'a, REG, PVMEN_A>;
impl<'a, REG> PVMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power voltage monitoring disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVMEN_A::Disabled)
    }
    #[doc = "Power voltage monitoring enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PVMEN_A::Enabled)
    }
}
#[doc = "Power voltage monitoring boundary select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PVMSEL_A {
    #[doc = "1: 2.3V"]
    V2_3 = 1,
    #[doc = "2: 2.4V"]
    V2_4 = 2,
    #[doc = "3: 2.5V"]
    V2_5 = 3,
    #[doc = "4: 2.6V"]
    V2_6 = 4,
    #[doc = "5: 2.7V"]
    V2_7 = 5,
    #[doc = "6: 2.8V"]
    V2_8 = 6,
    #[doc = "7: 2.9V"]
    V2_9 = 7,
}
impl From<PVMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PVMSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PVMSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for PVMSEL_A {}
#[doc = "Field `PVMSEL` reader - Power voltage monitoring boundary select"]
pub type PVMSEL_R = crate::FieldReader<PVMSEL_A>;
impl PVMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PVMSEL_A> {
        match self.bits {
            1 => Some(PVMSEL_A::V2_3),
            2 => Some(PVMSEL_A::V2_4),
            3 => Some(PVMSEL_A::V2_5),
            4 => Some(PVMSEL_A::V2_6),
            5 => Some(PVMSEL_A::V2_7),
            6 => Some(PVMSEL_A::V2_8),
            7 => Some(PVMSEL_A::V2_9),
            _ => None,
        }
    }
    #[doc = "2.3V"]
    #[inline(always)]
    pub fn is_v2_3(&self) -> bool {
        *self == PVMSEL_A::V2_3
    }
    #[doc = "2.4V"]
    #[inline(always)]
    pub fn is_v2_4(&self) -> bool {
        *self == PVMSEL_A::V2_4
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn is_v2_5(&self) -> bool {
        *self == PVMSEL_A::V2_5
    }
    #[doc = "2.6V"]
    #[inline(always)]
    pub fn is_v2_6(&self) -> bool {
        *self == PVMSEL_A::V2_6
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn is_v2_7(&self) -> bool {
        *self == PVMSEL_A::V2_7
    }
    #[doc = "2.8V"]
    #[inline(always)]
    pub fn is_v2_8(&self) -> bool {
        *self == PVMSEL_A::V2_8
    }
    #[doc = "2.9V"]
    #[inline(always)]
    pub fn is_v2_9(&self) -> bool {
        *self == PVMSEL_A::V2_9
    }
}
#[doc = "Field `PVMSEL` writer - Power voltage monitoring boundary select"]
pub type PVMSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, PVMSEL_A>;
impl<'a, REG> PVMSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.3V"]
    #[inline(always)]
    pub fn v2_3(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_3)
    }
    #[doc = "2.4V"]
    #[inline(always)]
    pub fn v2_4(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_4)
    }
    #[doc = "2.5V"]
    #[inline(always)]
    pub fn v2_5(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_5)
    }
    #[doc = "2.6V"]
    #[inline(always)]
    pub fn v2_6(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_6)
    }
    #[doc = "2.7V"]
    #[inline(always)]
    pub fn v2_7(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_7)
    }
    #[doc = "2.8V"]
    #[inline(always)]
    pub fn v2_8(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_8)
    }
    #[doc = "2.9V"]
    #[inline(always)]
    pub fn v2_9(self) -> &'a mut crate::W<REG> {
        self.variant(PVMSEL_A::V2_9)
    }
}
#[doc = "Battery powered domain write enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BPWEN_A {
    #[doc = "0: Battery powered domain write disabled"]
    Disabled = 0,
    #[doc = "1: Battery powered domain write enabled"]
    Enabled = 1,
}
impl From<BPWEN_A> for bool {
    #[inline(always)]
    fn from(variant: BPWEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPWEN` reader - Battery powered domain write enable"]
pub type BPWEN_R = crate::BitReader<BPWEN_A>;
impl BPWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BPWEN_A {
        match self.bits {
            false => BPWEN_A::Disabled,
            true => BPWEN_A::Enabled,
        }
    }
    #[doc = "Battery powered domain write disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BPWEN_A::Disabled
    }
    #[doc = "Battery powered domain write enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BPWEN_A::Enabled
    }
}
#[doc = "Field `BPWEN` writer - Battery powered domain write enable"]
pub type BPWEN_W<'a, REG> = crate::BitWriter<'a, REG, BPWEN_A>;
impl<'a, REG> BPWEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Battery powered domain write disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BPWEN_A::Disabled)
    }
    #[doc = "Battery powered domain write enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BPWEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Voltage regulator state select when deepsleep mode"]
    #[inline(always)]
    pub fn vrsel(&self) -> VRSEL_R {
        VRSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low power mode select when Cortex-M4F sleepdeep"]
    #[inline(always)]
    pub fn lpsel(&self) -> LPSEL_R {
        LPSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage monitoring enable"]
    #[inline(always)]
    pub fn pvmen(&self) -> PVMEN_R {
        PVMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Power voltage monitoring boundary select"]
    #[inline(always)]
    pub fn pvmsel(&self) -> PVMSEL_R {
        PVMSEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Battery powered domain write enable"]
    #[inline(always)]
    pub fn bpwen(&self) -> BPWEN_R {
        BPWEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("vrsel", &self.vrsel())
            .field("lpsel", &self.lpsel())
            .field("pvmen", &self.pvmen())
            .field("pvmsel", &self.pvmsel())
            .field("bpwen", &self.bpwen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Voltage regulator state select when deepsleep mode"]
    #[inline(always)]
    pub fn vrsel(&mut self) -> VRSEL_W<'_, CTRL_SPEC> {
        VRSEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Low power mode select when Cortex-M4F sleepdeep"]
    #[inline(always)]
    pub fn lpsel(&mut self) -> LPSEL_W<'_, CTRL_SPEC> {
        LPSEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear SWEF flag"]
    #[inline(always)]
    pub fn clswef(&mut self) -> CLSWEF_W<'_, CTRL_SPEC> {
        CLSWEF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear SEF flag"]
    #[inline(always)]
    pub fn clsef(&mut self) -> CLSEF_W<'_, CTRL_SPEC> {
        CLSEF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Power voltage monitoring enable"]
    #[inline(always)]
    pub fn pvmen(&mut self) -> PVMEN_W<'_, CTRL_SPEC> {
        PVMEN_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - Power voltage monitoring boundary select"]
    #[inline(always)]
    pub fn pvmsel(&mut self) -> PVMSEL_W<'_, CTRL_SPEC> {
        PVMSEL_W::new(self, 5)
    }
    #[doc = "Bit 8 - Battery powered domain write enable"]
    #[inline(always)]
    pub fn bpwen(&mut self) -> BPWEN_W<'_, CTRL_SPEC> {
        BPWEN_W::new(self, 8)
    }
}
#[doc = "Power control register (PWC_CTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {}
