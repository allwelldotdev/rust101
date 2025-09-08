//! Creating a mbe macro that can perform the fibonacci sequence and other sequences
//! depending on the sequence syntax passed in as the macro invocation argument.

/// Creating a macro to count the number repitions in `$($inits:expr),+` for
/// `recurrence` macro.
macro_rules! count_exprs {
    () => (0);
    ( $head:expr ) => (1);
    ( $head:expr, $( $tail:expr ),+ ) => (1 + count_exprs!($( $tail ),+));
}

/// Returns a type that implements `Iterator`.
macro_rules! recurrence {
    ( $seq:ident [ $index:ident ]: $sty:ty = $($inits:expr),+ ; ... ; $recur:expr ) => {
        {
            use std::ops::Index;

            const MEM_SIZE: usize = count_exprs!( $($inits),* );

            struct Recurrence {
                mem: [$sty; MEM_SIZE],
                pos: usize,
            }

            struct IndexOffset<'a> {
                slice: &'a [$sty; MEM_SIZE],
                offset: usize,
            }

            impl<'a> Index<usize> for IndexOffset<'a> {
                type Output = $sty;

                #[inline(always)]
                fn index(&self, index: usize) -> &Self::Output {
                    use std::num::Wrapping;

                    let index = Wrapping(index);
                    let offset = Wrapping(self.offset);
                    let window = Wrapping(MEM_SIZE);

                    let real_index = index - offset + window;
                    &self.slice[real_index.0]
                }
            }

            impl Iterator for Recurrence {
                type Item = $sty;

                #[inline]
                fn next(&mut self) -> Option<Self::Item> {
                    if self.pos < MEM_SIZE {
                        let next_val = self.mem[self.pos];
                        self.pos += 1;
                        Some(next_val)
                    } else {
                        let next_val = {
                            let $index = self.pos;
                            let $seq = IndexOffset {
                                slice: &self.mem,
                                offset: $index,
                            };
                            $recur
                        };

                        {
                            use std::mem::swap;

                            let mut swap_tmp = next_val;
                            for i in (0..MEM_SIZE).rev() {
                                swap(&mut swap_tmp, &mut self.mem[i]);
                            }
                        }

                        self.pos += 1;
                        Some(next_val)
                    }
                }
            }

            Recurrence {
                mem: [$($inits),+],
                pos: 0,
            }
        }
    }
}

fn main() {
    // let fib = recurrence![a[n]: u64 = 0, 1; ...; a[n-1] + a[n-2]];

    // let fib = {
    //     use std::ops::Index;

    //     struct Recurrence {
    //         mem: [u64; 2],
    //         pos: usize,
    //     }

    //     struct IndexOffset<'a> {
    //         slice: &'a [u64; 2],
    //         offset: usize,
    //     }

    //     impl<'a> Index<usize> for IndexOffset<'a> {
    //         type Output = u64;

    //         #[inline(always)]
    //         fn index(&self, index: usize) -> &Self::Output {
    //             use std::num::Wrapping;

    //             let index = Wrapping(index);
    //             let offset = Wrapping(self.offset);
    //             let window = Wrapping(2);

    //             let real_index = index - offset + window;
    //             &self.slice[real_index.0]
    //         }
    //     }

    //     impl Iterator for Recurrence {
    //         type Item = u64;

    //         #[inline]
    //         fn next(&mut self) -> Option<Self::Item> {
    //             if self.pos < 2 {
    //                 let next_val = self.mem[self.pos];
    //                 self.pos += 1;
    //                 Some(next_val)
    //             } else {
    //                 let next_val = {
    //                     let n = self.pos;
    //                     let a = IndexOffset {
    //                         slice: &self.mem,
    //                         offset: n,
    //                     };
    //                     a[n - 1] + a[n - 2]
    //                 };

    //                 {
    //                     use std::mem::swap;

    //                     let mut swap_tmp = next_val;
    //                     for i in (0..2).rev() {
    //                         swap(&mut swap_tmp, &mut self.mem[i]);
    //                     }
    //                 }

    //                 self.pos += 1;
    //                 Some(next_val)
    //             }
    //         }
    //     }

    //     Recurrence {
    //         mem: [0, 1],
    //         pos: 0,
    //     }
    // };

    // for e in fib.take(10) {
    //     println!("{e}")
    // }

    for e in recurrence!(a[n]: u64 = 0, 1; ... ; a[n-1] + a[n-2]).take(10) {
        println!("{e}");
    }
}
