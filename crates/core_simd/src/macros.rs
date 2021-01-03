/// Provides implementations of `From<$a> for $b` and `From<$b> for $a` that transmutes the value.
macro_rules! from_transmute {
    { unsafe $a:ty => $b:ty } => {
        from_transmute!{ @impl $a => $b }
        from_transmute!{ @impl $b => $a }
    };
    { @impl $from:ty => $to:ty } => {
        impl core::convert::From<$from> for $to {
            #[inline]
            fn from(value: $from) -> $to {
                unsafe { core::mem::transmute(value) }
            }
        }
    };
}

/// Provides implementations of `From<$generic> for core::arch::{x86, x86_64}::$intel` and
/// vice-versa that transmutes the value.
macro_rules! from_transmute_x86 {
    { unsafe $generic:ty => $intel:ident } => {
        #[cfg(target_arch = "x86")]
        from_transmute! { unsafe $generic => core::arch::x86::$intel }

        #[cfg(target_arch = "x86_64")]
        from_transmute! { unsafe $generic => core::arch::x86_64::$intel }
    }
}

/// Implements common traits on the specified vector `$name`, holding multiple `$lanes` of `$type`.
macro_rules! impl_vector {
    { $name:ident, $type:ty } => {
        impl<const LANES: usize> $name<LANES> {
            /// Construct a SIMD vector by setting all lanes to the given value.
            pub const fn splat(value: $type) -> Self {
                Self([value; LANES])
            }

            /// Returns a slice containing the entire SIMD vector.
            pub const fn as_slice(&self) -> &[$type] {
                &self.0
            }

            /// Returns a mutable slice containing the entire SIMD vector.
            pub fn as_mut_slice(&mut self) -> &mut [$type] {
                &mut self.0
            }

            /// Converts an array to a SIMD vector.
            pub const fn from_array(array: [$type; LANES]) -> Self {
                Self(array)
            }

            /// Converts a SIMD vector to an array.
            pub const fn to_array(self) -> [$type; LANES] {
                // workaround for rust-lang/rust#80108
                // TODO fix this
                #[cfg(target_arch = "wasm32")]
                {
                    let mut arr = [self.0[0]; LANES];
                    let mut i = 0;
                    while i < LANES {
                        arr[i] = self.0[i];
                        i += 1;
                    }
                    arr
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.0
                }
            }
        }

        impl<const LANES: usize> Copy for $name<LANES> {}

        impl<const LANES: usize> Clone for $name<LANES> {
            #[inline]
            fn clone(&self) -> Self {
                *self
            }
        }

        impl<const LANES: usize> Default for $name<LANES> {
            #[inline]
            fn default() -> Self {
                Self::splat(<$type>::default())
            }
        }

        impl<const LANES: usize> PartialEq for $name<LANES> {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                // TODO use SIMD equality
                self.to_array() == other.to_array()
            }
        }

        impl<const LANES: usize> PartialOrd for $name<LANES> {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                // TODO use SIMD equalitya
                self.to_array().partial_cmp(other.as_ref())
            }
        }

        // array references
        impl<const LANES: usize> AsRef<[$type; LANES]> for $name<LANES> {
            #[inline]
            fn as_ref(&self) -> &[$type; LANES] {
                &self.0
            }
        }

        impl<const LANES: usize> AsMut<[$type; LANES]> for $name<LANES> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$type; LANES] {
                &mut self.0
            }
        }

        // slice references
        impl<const LANES: usize> AsRef<[$type]> for $name<LANES> {
            #[inline]
            fn as_ref(&self) -> &[$type] {
                &self.0
            }
        }

        impl<const LANES: usize> AsMut<[$type]> for $name<LANES> {
            #[inline]
            fn as_mut(&mut self) -> &mut [$type] {
                &mut self.0
            }
        }

        // vector/array conversion
        impl<const LANES: usize> From<[$type; LANES]> for $name<LANES> {
            fn from(array: [$type; LANES]) -> Self {
                Self(array)
            }
        }

        impl <const LANES: usize> From<$name<LANES>> for [$type; LANES] {
            fn from(vector: $name<LANES>) -> Self {
                vector.0
            }
        }

        // splat
        impl<const LANES: usize> From<$type> for $name<LANES> {
            #[inline]
            fn from(value: $type) -> Self {
                Self::splat(value)
            }
        }
    }
}

/// Implements additional integer traits (Eq, Ord, Hash) on the specified vector `$name`, holding multiple `$lanes` of `$type`.
macro_rules! impl_integer_vector {
    { $name:ident, $type:ty } => {
        impl_vector! { $name, $type }

        impl<const LANES: usize> Eq for $name<LANES> {}

        impl<const LANES: usize> Ord for $name<LANES> {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                // TODO use SIMD cmp
                self.to_array().cmp(other.as_ref())
            }
        }

        impl<const LANES: usize> core::hash::Hash for $name<LANES> {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: core::hash::Hasher
            {
                self.as_slice().hash(state)
            }
        }
    }
}

/// Implements inherent methods for a float vector `$name` containing multiple
/// `$lanes` of float `$type`, which uses `$bits_ty` as its binary
/// representation. Called from `define_float_vector!`.
macro_rules! impl_float_vector {
    { $name:ident, $type:ty, $bits_ty:ident } => {
        impl_vector! { $name, $type }

        impl<const LANES: usize> $name<LANES> {
            /// Raw transmutation to an unsigned integer vector type with the
            /// same size and number of lanes.
            #[inline]
            pub fn to_bits(self) -> crate::$bits_ty<LANES> {
                assert_eq!(core::mem::size_of::<Self>(), core::mem::size_of::<crate::$bits_ty<LANES>>());
                unsafe { core::mem::transmute_copy(&self) }
            }

            /// Raw transmutation from an unsigned integer vector type with the
            /// same size and number of lanes.
            #[inline]
            pub fn from_bits(bits: crate::$bits_ty<LANES>) -> Self {
                assert_eq!(core::mem::size_of::<Self>(), core::mem::size_of::<crate::$bits_ty<LANES>>());
                unsafe { core::mem::transmute_copy(&bits) }
            }

            /// Produces a vector where every lane has the absolute value of the
            /// equivalently-indexed lane in `self`.
            #[inline]
            pub fn abs(self) -> Self {
                let no_sign = crate::$bits_ty::splat(!0 >> 1);
                Self::from_bits(self.to_bits() & no_sign)
            }
        }
    };
}
