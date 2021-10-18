// use msp430_atomic::AtomicOperations;

impl<REG: Readable + Writable> Reg<REG>
where
    REG::Ux: Default + core::ops::Not<Output = REG::Ux>,
{
    /// Set high every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn set_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut REG::Writer,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: Default::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        self.register
            .as_ptr()
            .add(0x2000 / core::mem::size_of::<REG::Ux>())
            .write_volatile(bits);
    }

    /// Clear every bit in the register that was cleared in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn clear_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut REG::Writer,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: !REG::Ux::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        self.register
            .as_ptr()
            .add(0x3000 / core::mem::size_of::<REG::Ux>())
            .write_volatile(bits);
    }

    /// Toggle every bit in the register that was set in the write proxy. Leave other bits
    /// untouched. The write is done in a single atomic instruction.
    #[inline(always)]
    pub unsafe fn toggle_bits<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut REG::Writer,
    {
        let bits = f(&mut REG::Writer::from(W {
            bits: Default::default(),
            _reg: marker::PhantomData,
        }))
        .bits;
        self.register
            .as_ptr()
            .add(0x1000 / core::mem::size_of::<REG::Ux>())
            .write_volatile(bits);
    }
}
