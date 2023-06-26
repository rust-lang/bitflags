// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A typesafe bitmask flag generator useful for sets of C-style flags.
//! It can be used for creating ergonomic wrappers around C APIs.
//!
//! The `bitflags!` macro generates `struct`s that manage a set of flags. The
//! type of those flags must be some primitive integer.
//!
//! # Examples
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!         const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
//!     }
//! }
//!
//! fn main() {
//!     let e1 = Flags::A | Flags::C;
//!     let e2 = Flags::B | Flags::C;
//!     assert_eq!((e1 | e2), Flags::ABC);   // union
//!     assert_eq!((e1 & e2), Flags::C);     // intersection
//!     assert_eq!((e1 - e2), Flags::A);     // set difference
//!     assert_eq!(!e2, Flags::A);           // set complement
//! }
//! ```
//!
//! See [`example_generated::Flags`](./example_generated/struct.Flags.html) for documentation of code
//! generated by the above `bitflags!` expansion.
//!
//! # Visibility
//!
//! The `bitflags!` macro supports visibility, just like you'd expect when writing a normal
//! Rust `struct`:
//!
//! ```
//! mod example {
//!     use bitflags::bitflags;
//!
//!     bitflags! {
//!         #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!         pub struct Flags1: u32 {
//!             const A = 0b00000001;
//!         }
//!
//!         #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//! #       pub
//!         struct Flags2: u32 {
//!             const B = 0b00000010;
//!         }
//!     }
//! }
//!
//! fn main() {
//!     let flag1 = example::Flags1::A;
//!     let flag2 = example::Flags2::B; // error: const `B` is private
//! }
//! ```
//!
//! # Attributes
//!
//! Attributes can be attached to the generated flags types and their constants as normal.
//!
//! # Representation
//!
//! It's valid to add a `#[repr(C)]` or `#[repr(transparent)]` attribute to a generated flags type.
//! The generated flags type is always guaranteed to be a newtype where its only field has the same
//! ABI as the underlying integer type.
//!
//! In this example, `Flags` has the same ABI as `u32`:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[repr(transparent)]
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//! ```
//!
//! # Extending
//!
//! Generated flags types belong to you, so you can add trait implementations to them outside
//! of what the `bitflags!` macro gives:
//!
//! ```
//! use std::fmt;
//!
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!     }
//! }
//!
//! impl Flags {
//!     pub fn clear(&mut self) {
//!         *self.0.bits_mut() = 0;
//!     }
//! }
//!
//! fn main() {
//!     let mut flags = Flags::A | Flags::B;
//!
//!     flags.clear();
//!     assert!(flags.is_empty());
//!
//!     assert_eq!(format!("{:?}", Flags::A | Flags::B), "Flags(A | B)");
//!     assert_eq!(format!("{:?}", Flags::B), "Flags(B)");
//! }
//! ```
//!
//! # What's implemented by `bitflags!`
//!
//! The `bitflags!` macro adds some trait implementations and inherent methods
//! to generated flags types, but leaves room for you to choose the semantics
//! of others.
//!
//! ## Iterators
//!
//! The following iterator traits are implemented for generated flags types:
//!
//! - `Extend`: adds the union of the instances iterated over.
//! - `FromIterator`: calculates the union.
//! - `IntoIterator`: iterates over set flag values.
//!
//! ## Formatting
//!
//! The following formatting traits are implemented for generated flags types:
//!
//! - `Binary`.
//! - `LowerHex` and `UpperHex`.
//! - `Octal`.
//!
//! Also see the _Debug and Display_ section for details about standard text
//! representations for flags types.
//!
//! ## Operators
//!
//! The following operator traits are implemented for the generated `struct`s:
//!
//! - `BitOr` and `BitOrAssign`: union
//! - `BitAnd` and `BitAndAssign`: intersection
//! - `BitXor` and `BitXorAssign`: toggle
//! - `Sub` and `SubAssign`: set difference
//! - `Not`: set complement
//!
//! ## Methods
//!
//! The following methods are defined for the generated `struct`s:
//!
//! - `empty`: an empty set of flags
//! - `all`: the set of all defined flags
//! - `bits`: the raw value of the flags currently stored
//! - `from_bits`: convert from underlying bit representation, unless that
//!                representation contains bits that do not correspond to a
//!                defined flag
//! - `from_bits_truncate`: convert from underlying bit representation, dropping
//!                         any bits that do not correspond to defined flags
//! - `from_bits_retain`: convert from underlying bit representation, keeping
//!                          all bits (even those not corresponding to defined
//!                          flags)
//! - `is_empty`: `true` if no flags are currently stored
//! - `is_all`: `true` if currently set flags exactly equal all defined flags
//! - `intersects`: `true` if there are flags common to both `self` and `other`
//! - `contains`: `true` if all of the flags in `other` are contained within `self`
//! - `insert`: inserts the specified flags in-place
//! - `remove`: removes the specified flags in-place
//! - `toggle`: the specified flags will be inserted if not present, and removed
//!             if they are.
//! - `set`: inserts or removes the specified flags depending on the passed value
//! - `intersection`: returns a new set of flags, containing only the flags present
//!                   in both `self` and `other` (the argument to the function).
//! - `union`: returns a new set of flags, containing any flags present in
//!            either `self` or `other` (the argument to the function).
//! - `difference`: returns a new set of flags, containing all flags present in
//!                 `self` without any of the flags present in `other` (the
//!                 argument to the function).
//! - `symmetric_difference`: returns a new set of flags, containing all flags
//!                           present in either `self` or `other` (the argument
//!                           to the function), but not both.
//! - `complement`: returns a new set of flags, containing all flags which are
//!                 not set in `self`, but which are allowed for this type.
//!
//! # What's not implemented by `bitflags!`
//!
//! Some functionality is not automatically implemented for generated flags types
//! by the `bitflags!` macro, even when it reasonably could be. This is so callers
//! have more freedom to decide on the semantics of their flags types.
//!
//! ## `Clone` and `Copy`
//!
//! Generated flags types are not automatically copyable, even though they can always
//! derive both `Clone` and `Copy`.
//!
//! ## `Default`
//!
//! The `Default` trait is not automatically implemented for the generated structs.
//!
//! If your default value is equal to `0` (which is the same value as calling `empty()`
//! on the generated struct), you can simply derive `Default`:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     // Results in default value with bits: 0
//!     #[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! fn main() {
//!     let derived_default: Flags = Default::default();
//!     assert_eq!(derived_default.bits(), 0);
//! }
//! ```
//!
//! If your default value is not equal to `0` you need to implement `Default` yourself:
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! // explicit `Default` implementation
//! impl Default for Flags {
//!     fn default() -> Flags {
//!         Flags::A | Flags::C
//!     }
//! }
//!
//! fn main() {
//!     let implemented_default: Flags = Default::default();
//!     assert_eq!(implemented_default, (Flags::A | Flags::C));
//! }
//! ```
//!
//! ## `Debug` and `Display`
//!
//! The `Debug` trait can be derived for a reasonable implementation. This library defines a standard
//! text-based representation for flags that generated flags types can use. For details on the exact
//! grammar, see the [`parser`] module.
//!
//! To support formatting and parsing your generated flags types using that representation, you can implement
//! the standard `Display` and `FromStr` traits in this fashion:
//!
//! ```
//! use bitflags::bitflags;
//! use std::{fmt, str};
//!
//! bitflags! {
//!     pub struct Flags: u32 {
//!         const A = 1;
//!         const B = 2;
//!         const C = 4;
//!         const D = 8;
//!     }
//! }
//!
//! impl fmt::Debug for Flags {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         fmt::Debug::fmt(&self.0, f)
//!     }
//! }
//!
//! impl fmt::Display for Flags {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         fmt::Display::fmt(&self.0, f)
//!     }
//! }
//!
//! impl str::FromStr for Flags {
//!     type Err = bitflags::parser::ParseError;
//!
//!     fn from_str(flags: &str) -> Result<Self, Self::Err> {
//!         Ok(Self(flags.parse()?))
//!     }
//! }
//! ```
//!
//! ## `PartialEq` and `PartialOrd`
//!
//! Equality and ordering can be derived for a reasonable implementation, or implemented manually
//! for different semantics.
//!
//! # Edge cases
//!
//! ## Zero Flags
//!
//! Flags with a value equal to zero will have some strange behavior that one should be aware of.
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const NONE = 0b00000000;
//!         const SOME = 0b00000001;
//!     }
//! }
//!
//! fn main() {
//!     let empty = Flags::empty();
//!     let none = Flags::NONE;
//!     let some = Flags::SOME;
//!
//!     // Zero flags are treated as always present
//!     assert!(empty.contains(Flags::NONE));
//!     assert!(none.contains(Flags::NONE));
//!     assert!(some.contains(Flags::NONE));
//!
//!     // Zero flags will be ignored when testing for emptiness
//!     assert!(none.is_empty());
//! }
//! ```
//!
//! Users should generally avoid defining a flag with a value of zero.
//!
//! ## Multi-bit Flags
//!
//! It is allowed to define a flag with multiple bits set, however such
//! flags are _not_ treated as a set where any of those bits is a valid
//! flag. Instead, each flag is treated as a unit when converting from
//! bits with [`from_bits`] or [`from_bits_truncate`].
//!
//! ```
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u8 {
//!         const F3 = 0b00000011;
//!     }
//! }
//!
//! fn main() {
//!     // This bit pattern does not set all the bits in `F3`, so it is rejected.
//!     assert!(Flags::from_bits(0b00000001).is_none());
//!     assert!(Flags::from_bits_truncate(0b00000001).is_empty());
//! }
//! ```
//!
//! [`from_bits`]: Flags::from_bits
//! [`from_bits_truncate`]: Flags::from_bits_truncate
//!
//! # The `Flags` trait
//!
//! This library defines a `Flags` trait that's implemented by all generated flags types.
//! The trait makes it possible to work with flags types generically:
//!
//! ```
//! fn count_unset_flags<F: bitflags::Flags>(flags: &F) -> usize {
//!     // Find out how many flags there are in total
//!     let total = F::all().iter().count();
//!
//!     // Find out how many flags are set
//!     let set = flags.iter().count();
//!
//!     total - set
//! }
//!
//! use bitflags::bitflags;
//!
//! bitflags! {
//!     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
//!     struct Flags: u32 {
//!         const A = 0b00000001;
//!         const B = 0b00000010;
//!         const C = 0b00000100;
//!     }
//! }
//!
//! assert_eq!(2, count_unset_flags(&Flags::B));
//! ```
//!
//! # The internal field
//!
//! This library generates newtypes like:
//!
//! ```
//! # pub struct Field0;
//! pub struct Flags(Field0);
//! ```
//!
//! You can freely use methods and trait implementations on this internal field as `.0`.
//! For details on exactly what's generated for it, see the [`Field0`](example_generated/struct.Field0.html)
//! example docs.

#![cfg_attr(not(any(feature = "std", test)), no_std)]
#![cfg_attr(not(test), forbid(unsafe_code))]
#![cfg_attr(test, allow(mixed_script_confusables))]
#![doc(html_root_url = "https://docs.rs/bitflags/2.3.2")]

#[doc(inline)]
pub use traits::{Bits, Flag, Flags};

pub mod iter;
pub mod parser;

mod traits;

#[doc(hidden)]
pub mod __private {
    pub use crate::{external::__private::*, traits::__private::*};

    pub use core;
}

#[allow(unused_imports)]
pub use external::*;

#[allow(deprecated)]
pub use traits::BitFlags;

/*
How does the bitflags crate work?

This library generates a `struct` in the end-user's crate with a bunch of constants on it that represent flags.
The difference between `bitflags` and a lot of other libraries is that we don't actually control the generated `struct` in the end.
It's part of the end-user's crate, so it belongs to them. That makes it difficult to extend `bitflags` with new functionality
because we could end up breaking valid code that was already written.

Our solution is to split the type we generate into two: the public struct owned by the end-user, and an internal struct owned by `bitflags` (us).
To give you an example, let's say we had a crate that called `bitflags!`:

```rust
bitflags! {
    pub struct MyFlags: u32 {
        const A = 1;
        const B = 2;
    }
}
```

What they'd end up with looks something like this:

```rust
pub struct MyFlags(<MyFlags as PublicFlags>::InternalBitFlags);

const _: () = {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MyInternalBitFlags {
        bits: u32,
    }

    impl PublicFlags for MyFlags {
        type Internal = InternalBitFlags;
    }
};
```

If we want to expose something like a new trait impl for generated flags types, we add it to our generated `MyInternalBitFlags`,
and let `#[derive]` on `MyFlags` pick up that implementation, if an end-user chooses to add one.

The public API is generated in the `__impl_public_flags!` macro, and the internal API is generated in
the `__impl_internal_flags!` macro.

The macros are split into 3 modules:

- `public`: where the user-facing flags types are generated.
- `internal`: where the `bitflags`-facing flags types are generated.
- `external`: where external library traits are implemented conditionally.
*/

/// The macro used to generate the flag structure.
///
/// See the [crate level docs](../bitflags/index.html) for complete documentation.
///
/// # Example
///
/// ```
/// use bitflags::bitflags;
///
/// bitflags! {
///     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
///     struct Flags: u32 {
///         const A = 0b00000001;
///         const B = 0b00000010;
///         const C = 0b00000100;
///         const ABC = Self::A.bits() | Self::B.bits() | Self::C.bits();
///     }
/// }
///
/// let e1 = Flags::A | Flags::C;
/// let e2 = Flags::B | Flags::C;
/// assert_eq!((e1 | e2), Flags::ABC);   // union
/// assert_eq!((e1 & e2), Flags::C);     // intersection
/// assert_eq!((e1 - e2), Flags::A);     // set difference
/// assert_eq!(!e2, Flags::A);           // set complement
/// ```
///
/// The generated `struct`s can also be extended with type and trait
/// implementations:
///
/// ```
/// use std::fmt;
///
/// use bitflags::bitflags;
///
/// bitflags! {
///     #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
///     struct Flags: u32 {
///         const A = 0b00000001;
///         const B = 0b00000010;
///     }
/// }
///
/// impl Flags {
///     pub fn clear(&mut self) {
///         *self.0.bits_mut() = 0;
///     }
/// }
///
/// let mut flags = Flags::A | Flags::B;
///
/// flags.clear();
/// assert!(flags.is_empty());
///
/// assert_eq!(format!("{:?}", Flags::A | Flags::B), "Flags(A | B)");
/// assert_eq!(format!("{:?}", Flags::B), "Flags(B)");
/// ```
#[macro_export(local_inner_macros)]
macro_rules! bitflags {
    (
        $(#[$outer:meta])*
        $vis:vis struct $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        // Declared in the scope of the `bitflags!` call
        // This type appears in the end-user's API
        __declare_public_bitflags! {
            $(#[$outer])*
            $vis struct $BitFlags
        }

        // Workaround for: https://github.com/bitflags/bitflags/issues/320
        __impl_public_bitflags_consts! {
            $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )*
            }
        }

        #[allow(
            dead_code,
            deprecated,
            unused_doc_comments,
            unused_attributes,
            unused_mut,
            unused_imports,
            non_upper_case_globals,
            clippy::assign_op_pattern
        )]
        const _: () = {
            // Declared in a "hidden" scope that can't be reached directly
            // These types don't appear in the end-user's API
            __declare_internal_bitflags! {
                $vis struct InternalBitFlags: $T
            }

            __impl_internal_bitflags! {
                InternalBitFlags: $T, $BitFlags {
                    $(
                        $(#[$inner $($args)*])*
                        $Flag = $value;
                    )*
                }
            }

            // This is where new library trait implementations can be added
            __impl_external_bitflags! {
                InternalBitFlags: $T, $BitFlags {
                    $(
                        $(#[$inner $($args)*])*
                        $Flag;
                    )*
                }
            }

            __impl_public_bitflags_forward! {
                $BitFlags: $T, InternalBitFlags
            }

            __impl_public_bitflags_ops! {
                $BitFlags
            }

            __impl_public_bitflags_iter! {
                $BitFlags: $T, $BitFlags
            }
        };

        bitflags! {
            $($t)*
        }
    };
    (
        impl $BitFlags:ident: $T:ty {
            $(
                $(#[$inner:ident $($args:tt)*])*
                const $Flag:ident = $value:expr;
            )*
        }

        $($t:tt)*
    ) => {
        __impl_public_bitflags_consts! {
            $BitFlags: $T {
                $(
                    $(#[$inner $($args)*])*
                    $Flag = $value;
                )*
            }
        }

        #[allow(
            dead_code,
            deprecated,
            unused_doc_comments,
            unused_attributes,
            unused_mut,
            unused_imports,
            non_upper_case_globals,
            clippy::assign_op_pattern
        )]
        const _: () = {
            __impl_public_bitflags! {
                $BitFlags: $T, $BitFlags {
                    $(
                        $(#[$inner $($args)*])*
                        $Flag;
                    )*
                }
            }

            __impl_public_bitflags_ops! {
                $BitFlags
            }

            __impl_public_bitflags_iter! {
                $BitFlags: $T, $BitFlags
            }
        };

        bitflags! {
            $($t)*
        }
    };
    () => {};
}

/// Implement functions on bitflags types.
///
/// We need to be careful about adding new methods and trait implementations here because they
/// could conflict with items added by the end-user.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __impl_bitflags {
    (
        $PublicBitFlags:ident: $T:ty {
            fn empty() $empty:block
            fn all() $all:block
            fn bits($bits0:ident) $bits:block
            fn from_bits($from_bits0:ident) $from_bits:block
            fn from_bits_truncate($from_bits_truncate0:ident) $from_bits_truncate:block
            fn from_bits_retain($from_bits_retain0:ident) $from_bits_retain:block
            fn from_name($from_name0:ident) $from_name:block
            fn is_empty($is_empty0:ident) $is_empty:block
            fn is_all($is_all0:ident) $is_all:block
            fn intersects($intersects0:ident, $intersects1:ident) $intersects:block
            fn contains($contains0:ident, $contains1:ident) $contains:block
            fn insert($insert0:ident, $insert1:ident) $insert:block
            fn remove($remove0:ident, $remove1:ident) $remove:block
            fn toggle($toggle0:ident, $toggle1:ident) $toggle:block
            fn set($set0:ident, $set1:ident, $set2:ident) $set:block
            fn intersection($intersection0:ident, $intersection1:ident) $intersection:block
            fn union($union0:ident, $union1:ident) $union:block
            fn difference($difference0:ident, $difference1:ident) $difference:block
            fn symmetric_difference($symmetric_difference0:ident, $symmetric_difference1:ident) $symmetric_difference:block
            fn complement($complement0:ident) $complement:block
        }
    ) => {
        #[allow(dead_code, deprecated, unused_attributes)]
        impl $PublicBitFlags {
            /// Returns an empty set of flags.
            #[inline]
            pub const fn empty() -> Self {
                $empty
            }

            /// Returns the set containing all flags.
            #[inline]
            pub const fn all() -> Self {
                $all
            }

            /// Returns the raw value of the flags currently stored.
            #[inline]
            pub const fn bits(&self) -> $T {
                let $bits0 = self;
                $bits
            }

            /// Convert from underlying bit representation, unless that
            /// representation contains bits that do not correspond to a flag.
            #[inline]
            pub const fn from_bits(bits: $T) -> $crate::__private::core::option::Option<Self> {
                let $from_bits0 = bits;
                $from_bits
            }

            /// Convert from underlying bit representation, dropping any bits
            /// that do not correspond to flags.
            #[inline]
            pub const fn from_bits_truncate(bits: $T) -> Self {
                let $from_bits_truncate0 = bits;
                $from_bits_truncate
            }

            /// Convert from underlying bit representation, preserving all
            /// bits (even those not corresponding to a defined flag).
            #[inline]
            pub const fn from_bits_retain(bits: $T) -> Self {
                let $from_bits_retain0 = bits;
                $from_bits_retain
            }

            /// Get the value for a flag from its stringified name.
            ///
            /// Names are _case-sensitive_, so must correspond exactly to
            /// the identifier given to the flag.
            #[inline]
            pub fn from_name(name: &str) -> $crate::__private::core::option::Option<Self> {
                let $from_name0 = name;
                $from_name
            }

            /// Returns `true` if no flags are currently stored.
            #[inline]
            pub const fn is_empty(&self) -> bool {
                let $is_empty0 = self;
                $is_empty
            }

            /// Returns `true` if all flags are currently set.
            #[inline]
            pub const fn is_all(&self) -> bool {
                let $is_all0 = self;
                $is_all
            }

            /// Returns `true` if there are flags common to both `self` and `other`.
            #[inline]
            pub const fn intersects(&self, other: Self) -> bool {
                let $intersects0 = self;
                let $intersects1 = other;
                $intersects
            }

            /// Returns `true` if all of the flags in `other` are contained within `self`.
            #[inline]
            pub const fn contains(&self, other: Self) -> bool {
                let $contains0 = self;
                let $contains1 = other;
                $contains
            }

            /// Inserts the specified flags in-place.
            ///
            /// This method is equivalent to `union`.
            #[inline]
            pub fn insert(&mut self, other: Self) {
                let $insert0 = self;
                let $insert1 = other;
                $insert
            }

            /// Removes the specified flags in-place.
            ///
            /// This method is equivalent to `difference`.
            #[inline]
            pub fn remove(&mut self, other: Self) {
                let $remove0 = self;
                let $remove1 = other;
                $remove
            }

            /// Toggles the specified flags in-place.
            ///
            /// This method is equivalent to `symmetric_difference`.
            #[inline]
            pub fn toggle(&mut self, other: Self) {
                let $toggle0 = self;
                let $toggle1 = other;
                $toggle
            }

            /// Inserts or removes the specified flags depending on the passed value.
            #[inline]
            pub fn set(&mut self, other: Self, value: bool) {
                let $set0 = self;
                let $set1 = other;
                let $set2 = value;
                $set
            }

            /// Returns the intersection between the flags in `self` and
            /// `other`.
            ///
            /// Calculating `self` bitwise and (`&`) other, including
            /// any bits that don't correspond to a defined flag.
            #[inline]
            #[must_use]
            pub const fn intersection(self, other: Self) -> Self {
                let $intersection0 = self;
                let $intersection1 = other;
                $intersection
            }

            /// Returns the union of between the flags in `self` and `other`.
            ///
            /// Calculates `self` bitwise or (`|`) `other`, including
            /// any bits that don't correspond to a defined flag.
            #[inline]
            #[must_use]
            pub const fn union(self, other: Self) -> Self {
                let $union0 = self;
                let $union1 = other;
                $union
            }

            /// Returns the difference between the flags in `self` and `other`.
            ///
            /// Calculates `self` bitwise and (`&!`) the bitwise negation of `other`,
            /// including any bits that don't correspond to a defined flag.
            #[inline]
            #[must_use]
            pub const fn difference(self, other: Self) -> Self {
                let $difference0 = self;
                let $difference1 = other;
                $difference
            }

            /// Returns the symmetric difference between the flags
            /// in `self` and `other`.
            ///
            /// Calculates `self` bitwise exclusive or (`^`) `other`,
            /// including any bits that don't correspond to a defined flag.
            #[inline]
            #[must_use]
            pub const fn symmetric_difference(self, other: Self) -> Self {
                let $symmetric_difference0 = self;
                let $symmetric_difference1 = other;
                $symmetric_difference
            }

            /// Returns the complement of this set of flags.
            ///
            /// Calculates the bitwise negation (`!`) of `self`,
            /// **unsetting** any bits that don't correspond to a defined flag.
            #[inline]
            #[must_use]
            pub const fn complement(self) -> Self {
                let $complement0 = self;
                $complement
            }
        }
    };
}

/// A macro that processed the input to `bitflags!` and shuffles attributes around
/// based on whether or not they're "expression-safe".
///
/// This macro is a token-tree muncher that works on 2 levels:
///
/// For each attribute, we explicitly match on its identifier, like `cfg` to determine
/// whether or not it should be considered expression-safe.
///
/// If you find yourself with an attribute that should be considered expression-safe
/// and isn't, it can be added here.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! __bitflags_expr_safe_attrs {
    // Entrypoint: Move all flags and all attributes into `unprocessed` lists
    // where they'll be munched one-at-a-time
    (
        $(#[$inner:ident $($args:tt)*])*
        { $e:expr }
    ) => {
        __bitflags_expr_safe_attrs! {
            expr: { $e },
            attrs: {
                // All attributes start here
                unprocessed: [$(#[$inner $($args)*])*],
                // Attributes that are safe on expressions go here
                processed: [],
            },
        }
    };
    // Process the next attribute on the current flag
    // `cfg`: The next flag should be propagated to expressions
    // NOTE: You can copy this rules block and replace `cfg` with
    // your attribute name that should be considered expression-safe
    (
        expr: { $e:expr },
            attrs: {
            unprocessed: [
                // cfg matched here
                #[cfg $($args:tt)*]
                $($attrs_rest:tt)*
            ],
            processed: [$($expr:tt)*],
        },
    ) => {
        __bitflags_expr_safe_attrs! {
            expr: { $e },
            attrs: {
                unprocessed: [
                    $($attrs_rest)*
                ],
                processed: [
                    $($expr)*
                    // cfg added here
                    #[cfg $($args)*]
                ],
            },
        }
    };
    // Process the next attribute on the current flag
    // `$other`: The next flag should not be propagated to expressions
    (
        expr: { $e:expr },
            attrs: {
            unprocessed: [
                // $other matched here
                #[$other:ident $($args:tt)*]
                $($attrs_rest:tt)*
            ],
            processed: [$($expr:tt)*],
        },
    ) => {
        __bitflags_expr_safe_attrs! {
            expr: { $e },
                attrs: {
                unprocessed: [
                    $($attrs_rest)*
                ],
                processed: [
                    // $other not added here
                    $($expr)*
                ],
            },
        }
    };
    // Once all attributes on all flags are processed, generate the actual code
    (
        expr: { $e:expr },
        attrs: {
            unprocessed: [],
            processed: [$(#[$expr:ident $($exprargs:tt)*])*],
        },
    ) => {
        $(#[$expr $($exprargs)*])*
        { $e }
    }
}

#[macro_use]
mod public;
#[macro_use]
mod internal;
#[macro_use]
mod external;

#[cfg(feature = "example_generated")]
pub mod example_generated;

#[cfg(test)]
mod tests;
