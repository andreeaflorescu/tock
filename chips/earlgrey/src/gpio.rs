// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

//! GPIO instantiation.

use core::ops::{Index, IndexMut};

use crate::registers::top_earlgrey::{
    TOP_EARLGREY_GPIO_BASE_ADDR, TOP_EARLGREY_PINMUX_AON_BASE_ADDR,
};
use kernel::utilities::StaticRef;
use lowrisc::gpio::GpioRegisters;
pub use lowrisc::gpio::{pins, GpioPin};
use lowrisc::padctrl::PadCtrlRegisters;

pub const PADCTRL_BASE: StaticRef<PadCtrlRegisters> =
    unsafe { StaticRef::new(TOP_EARLGREY_PINMUX_AON_BASE_ADDR as *const PadCtrlRegisters) };

pub const GPIO0_BASE: StaticRef<GpioRegisters> =
    unsafe { StaticRef::new(TOP_EARLGREY_GPIO_BASE_ADDR as *const GpioRegisters) };

pub struct Port<'a> {
    pins: [GpioPin<'a>; 32],
}

impl<'a> Port<'a> {
    pub const fn new() -> Self {
        Self {
            pins: [
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin0),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin1),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin2),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin3),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin4),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin5),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin6),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin7),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin8),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin9),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin10),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin11),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin12),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin13),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin14),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin15),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin16),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin17),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin18),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin19),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin20),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin21),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin22),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin23),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin24),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin25),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin26),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin27),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin28),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin29),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin30),
                GpioPin::new(GPIO0_BASE, PADCTRL_BASE, pins::pin31),
            ],
        }
    }
}

impl<'a> Index<usize> for Port<'a> {
    type Output = GpioPin<'a>;

    fn index(&self, index: usize) -> &GpioPin<'a> {
        &self.pins[index]
    }
}

impl<'a> IndexMut<usize> for Port<'a> {
    fn index_mut(&mut self, index: usize) -> &mut GpioPin<'a> {
        &mut self.pins[index]
    }
}
