/// Implements a test on a unary operation using proptest.
///
/// Compares the vector operation to the equivalent scalar operation.
#[macro_export]
macro_rules! impl_unary_op_test {
    { $vector:ty, $scalar:ty, $trait:ident :: $fn:ident, $scalar_fn:expr } => {
        test_helpers::test_lanes! {
            fn $fn<const LANES: usize>() {
                test_helpers::test_unary_elementwise(
                    &<$vector as core::ops::$trait>::$fn,
                    &$scalar_fn,
                    &|_| true,
                );
            }
        }
    };
    { $vector:ty, $scalar:ty, $trait:ident :: $fn:ident } => {
        impl_unary_op_test! { $vector, $scalar, $trait::$fn, <$scalar as core::ops::$trait>::$fn }
    };
}

/// Implements a test on a binary operation using proptest.
///
/// Compares the vector operation to the equivalent scalar operation.
#[macro_export]
macro_rules! impl_binary_op_test {
    { $vector:ty, $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident, $scalar_fn:expr } => {
        mod $fn {
            use super::*;

            test_helpers::test_lanes! {
                fn normal<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &<$vector as core::ops::$trait>::$fn,
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }

                fn scalar_rhs<const LANES: usize>() {
                    test_helpers::test_binary_scalar_rhs_elementwise(
                        &<$vector as core::ops::$trait<$scalar>>::$fn,
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }

                fn scalar_lhs<const LANES: usize>() {
                    test_helpers::test_binary_scalar_lhs_elementwise(
                        &<$scalar as core::ops::$trait<$vector>>::$fn,
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }

                fn assign<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &|mut a, b| { <$vector as core::ops::$trait_assign>::$fn_assign(&mut a, b); a },
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }

                fn assign_scalar_rhs<const LANES: usize>() {
                    test_helpers::test_binary_scalar_rhs_elementwise(
                        &|mut a, b| { <$vector as core::ops::$trait_assign<$scalar>>::$fn_assign(&mut a, b); a },
                        &$scalar_fn,
                        &|_, _| true,
                    );
                }
            }
        }
    };
    { $vector:ty, $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident } => {
        impl_binary_op_test! { $vector, $scalar, $trait::$fn, $trait_assign::$fn_assign, <$scalar as core::ops::$trait>::$fn }
    };
}

/// Implements a test on a binary operation using proptest.
///
/// Like `impl_binary_op_test`, but allows providing a function for rejecting particular inputs
/// (like the `proptest_assume` macro).
///
/// Compares the vector operation to the equivalent scalar operation.
#[macro_export]
macro_rules! impl_binary_checked_op_test {
    { $vector:ty, $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident, $scalar_fn:expr, $check_fn:expr } => {
        mod $fn {
            use super::*;

            test_helpers::test_lanes! {
                fn normal<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &<$vector as core::ops::$trait>::$fn,
                        &$scalar_fn,
                        &|x, y| x.iter().zip(y.iter()).all(|(x, y)| $check_fn(*x, *y)),
                    );
                }

                fn scalar_rhs<const LANES: usize>() {
                    test_helpers::test_binary_scalar_rhs_elementwise(
                        &<$vector as core::ops::$trait<$scalar>>::$fn,
                        &$scalar_fn,
                        &|x, y| x.iter().all(|x| $check_fn(*x, y)),
                    );
                }

                fn scalar_lhs<const LANES: usize>() {
                    test_helpers::test_binary_scalar_lhs_elementwise(
                        &<$scalar as core::ops::$trait<$vector>>::$fn,
                        &$scalar_fn,
                        &|x, y| y.iter().all(|y| $check_fn(x, *y)),
                    );
                }

                fn assign<const LANES: usize>() {
                    test_helpers::test_binary_elementwise(
                        &|mut a, b| { <$vector as core::ops::$trait_assign>::$fn_assign(&mut a, b); a },
                        &$scalar_fn,
                        &|x, y| x.iter().zip(y.iter()).all(|(x, y)| $check_fn(*x, *y)),
                    )
                }

                fn assign_scalar_rhs<const LANES: usize>() {
                    test_helpers::test_binary_scalar_rhs_elementwise(
                        &|mut a, b| { <$vector as core::ops::$trait_assign<$scalar>>::$fn_assign(&mut a, b); a },
                        &$scalar_fn,
                        &|x, y| x.iter().all(|x| $check_fn(*x, y)),
                    )
                }
            }
        }
    };
    { $vector:ty, $scalar:ty, $trait:ident :: $fn:ident, $trait_assign:ident :: $fn_assign:ident, $check_fn:expr } => {
        impl_binary_nonzero_rhs_op_test! { $vector, $scalar, $trait::$fn, $trait_assign::$fn_assign, <$scalar as core::ops::$trait>::$fn, $check_fn }
    };
}

#[macro_export]
macro_rules! impl_common_integer_tests {
    { $vector:ident, $scalar:ident } => {
        test_helpers::test_lanes! {
            fn horizontal_sum<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_sum(),
                        x.iter().copied().fold(0 as $scalar, $scalar::wrapping_add),
                    );
                    Ok(())
                });
            }

            fn horizontal_product<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_product(),
                        x.iter().copied().fold(1 as $scalar, $scalar::wrapping_mul),
                    );
                    Ok(())
                });
            }

            fn horizontal_and<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_and(),
                        x.iter().copied().fold(-1i8 as $scalar, <$scalar as core::ops::BitAnd>::bitand),
                    );
                    Ok(())
                });
            }

            fn horizontal_or<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_or(),
                        x.iter().copied().fold(0 as $scalar, <$scalar as core::ops::BitOr>::bitor),
                    );
                    Ok(())
                });
            }

            fn horizontal_xor<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_xor(),
                        x.iter().copied().fold(0 as $scalar, <$scalar as core::ops::BitXor>::bitxor),
                    );
                    Ok(())
                });
            }

            fn horizontal_max<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_max(),
                        x.iter().copied().max().unwrap(),
                    );
                    Ok(())
                });
            }

            fn horizontal_min<const LANES: usize>() {
                test_helpers::test_1(&|x| {
                    test_helpers::prop_assert_biteq! (
                        $vector::<LANES>::from_array(x).horizontal_min(),
                        x.iter().copied().min().unwrap(),
                    );
                    Ok(())
                });
            }
        }
    }
}

/// Implement tests for signed integers.
#[macro_export]
macro_rules! impl_signed_tests {
    { $vector:ident, $scalar:tt } => {
        mod $scalar {
            type Vector<const LANES: usize> = core_simd::$vector<LANES>;
            type Scalar = $scalar;

            impl_common_integer_tests! { Vector, Scalar }

            test_helpers::test_lanes! {
                fn neg<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &<Vector::<LANES> as core::ops::Neg>::neg,
                        &<Scalar as core::ops::Neg>::neg,
                        &|x| !x.contains(&Scalar::MIN),
                    );
                }

                fn is_positive<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_positive,
                        &Scalar::is_positive,
                        &|_| true,
                    );
                }

                fn is_negative<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_negative,
                        &Scalar::is_negative,
                        &|_| true,
                    );
                }
            }

            test_helpers::test_lanes_panic! {
                fn div_min_overflow_panics<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(Scalar::MIN);
                    let b = Vector::<LANES>::splat(-1);
                    let _ = a / b;
                }

                fn div_by_all_zeros_panics<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(0);
                    let _ = a / b;
                }

                fn div_by_one_zero_panics<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let mut b = Vector::<LANES>::splat(21);
                    b[0] = 0 as _;
                    let _ = a / b;
                }

                fn rem_min_overflow_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(Scalar::MIN);
                    let b = Vector::<LANES>::splat(-1);
                    let _ = a % b;
                }

                fn rem_zero_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(0);
                    let _ = a % b;
                }
            }

            test_helpers::test_lanes! {
                fn div_neg_one_no_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(-1);
                    let _ = a / b;
                }

                fn rem_neg_one_no_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(-1);
                    let _ = a % b;
                }
            }

            impl_binary_op_test!(Vector<LANES>, Scalar, Add::add, AddAssign::add_assign, Scalar::wrapping_add);
            impl_binary_op_test!(Vector<LANES>, Scalar, Sub::sub, SubAssign::sub_assign, Scalar::wrapping_sub);
            impl_binary_op_test!(Vector<LANES>, Scalar, Mul::mul, MulAssign::mul_assign, Scalar::wrapping_mul);

            // Exclude Div and Rem panicking cases
            impl_binary_checked_op_test!(Vector<LANES>, Scalar, Div::div, DivAssign::div_assign, Scalar::wrapping_div, |x, y| y != 0 && !(x == Scalar::MIN && y == -1));
            impl_binary_checked_op_test!(Vector<LANES>, Scalar, Rem::rem, RemAssign::rem_assign, Scalar::wrapping_rem, |x, y| y != 0 && !(x == Scalar::MIN && y == -1));

            impl_unary_op_test!(Vector<LANES>, Scalar, Not::not);
            impl_binary_op_test!(Vector<LANES>, Scalar, BitAnd::bitand, BitAndAssign::bitand_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, BitOr::bitor, BitOrAssign::bitor_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, BitXor::bitxor, BitXorAssign::bitxor_assign);
        }
    }
}

/// Implement tests for unsigned integers.
#[macro_export]
macro_rules! impl_unsigned_tests {
    { $vector:ident, $scalar:tt } => {
        mod $scalar {
            type Vector<const LANES: usize> = core_simd::$vector<LANES>;
            type Scalar = $scalar;

            impl_common_integer_tests! { Vector, Scalar }

            test_helpers::test_lanes_panic! {
                fn rem_zero_panic<const LANES: usize>() {
                    let a = Vector::<LANES>::splat(42);
                    let b = Vector::<LANES>::splat(0);
                    let _ = a % b;
                }
            }

            impl_binary_op_test!(Vector<LANES>, Scalar, Add::add, AddAssign::add_assign, Scalar::wrapping_add);
            impl_binary_op_test!(Vector<LANES>, Scalar, Sub::sub, SubAssign::sub_assign, Scalar::wrapping_sub);
            impl_binary_op_test!(Vector<LANES>, Scalar, Mul::mul, MulAssign::mul_assign, Scalar::wrapping_mul);

            // Exclude Div and Rem panicking cases
            impl_binary_checked_op_test!(Vector<LANES>, Scalar, Div::div, DivAssign::div_assign, Scalar::wrapping_div, |_, y| y != 0);
            impl_binary_checked_op_test!(Vector<LANES>, Scalar, Rem::rem, RemAssign::rem_assign, Scalar::wrapping_rem, |_, y| y != 0);

            impl_unary_op_test!(Vector<LANES>, Scalar, Not::not);
            impl_binary_op_test!(Vector<LANES>, Scalar, BitAnd::bitand, BitAndAssign::bitand_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, BitOr::bitor, BitOrAssign::bitor_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, BitXor::bitxor, BitXorAssign::bitxor_assign);
        }
    }
}

/// Implement tests for floating point numbers.
#[macro_export]
macro_rules! impl_float_tests {
    { $vector:ident, $scalar:tt, $int_scalar:tt } => {
        mod $scalar {
            type Vector<const LANES: usize> = core_simd::$vector<LANES>;
            type Scalar = $scalar;

            impl_unary_op_test!(Vector<LANES>, Scalar, Neg::neg);
            impl_binary_op_test!(Vector<LANES>, Scalar, Add::add, AddAssign::add_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, Sub::sub, SubAssign::sub_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, Mul::mul, MulAssign::mul_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, Div::div, DivAssign::div_assign);
            impl_binary_op_test!(Vector<LANES>, Scalar, Rem::rem, RemAssign::rem_assign);

            test_helpers::test_lanes! {
                fn is_sign_positive<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_sign_positive,
                        &Scalar::is_sign_positive,
                        &|_| true,
                    );
                }

                fn is_sign_negative<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_sign_negative,
                        &Scalar::is_sign_negative,
                        &|_| true,
                    );
                }

                fn is_finite<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_finite,
                        &Scalar::is_finite,
                        &|_| true,
                    );
                }

                fn is_infinite<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_infinite,
                        &Scalar::is_infinite,
                        &|_| true,
                    );
                }

                fn is_nan<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_nan,
                        &Scalar::is_nan,
                        &|_| true,
                    );
                }

                fn is_normal<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_normal,
                        &Scalar::is_normal,
                        &|_| true,
                    );
                }

                fn is_subnormal<const LANES: usize>() {
                    test_helpers::test_unary_mask_elementwise(
                        &Vector::<LANES>::is_subnormal,
                        &Scalar::is_subnormal,
                        &|_| true,
                    );
                }

                fn abs<const LANES: usize>() {
                    test_helpers::test_unary_elementwise(
                        &Vector::<LANES>::abs,
                        &Scalar::abs,
                        &|_| true,
                    )
                }

                fn horizontal_sum<const LANES: usize>() {
                    test_helpers::test_1(&|x| {
                        test_helpers::prop_assert_biteq! (
                            Vector::<LANES>::from_array(x).horizontal_sum(),
                            x.iter().sum(),
                        );
                        Ok(())
                    });
                }

                fn horizontal_product<const LANES: usize>() {
                    test_helpers::test_1(&|x| {
                        test_helpers::prop_assert_biteq! (
                            Vector::<LANES>::from_array(x).horizontal_product(),
                            x.iter().product(),
                        );
                        Ok(())
                    });
                }

                fn horizontal_max<const LANES: usize>() {
                    test_helpers::test_1(&|x| {
                        let vmax = Vector::<LANES>::from_array(x).horizontal_max();
                        let smax = x.iter().copied().fold(Scalar::NAN, Scalar::max);
                        // 0 and -0 are treated the same
                        if !(x.contains(&0.) && x.contains(&-0.) && vmax.abs() == 0. && smax.abs() == 0.) {
                            test_helpers::prop_assert_biteq!(vmax, smax);
                        }
                        Ok(())
                    });
                }

                fn horizontal_min<const LANES: usize>() {
                    test_helpers::test_1(&|x| {
                        let vmax = Vector::<LANES>::from_array(x).horizontal_min();
                        let smax = x.iter().copied().fold(Scalar::NAN, Scalar::min);
                        // 0 and -0 are treated the same
                        if !(x.contains(&0.) && x.contains(&-0.) && vmax.abs() == 0. && smax.abs() == 0.) {
                            test_helpers::prop_assert_biteq!(vmax, smax);
                        }
                        Ok(())
                    });
                }
            }
        }
    }
}
