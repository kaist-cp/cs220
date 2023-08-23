#!/usr/bin/env bash

TEST01=(
    "assignments::assignment01::small_exercises_grade::test::test_add_7_3"
    "assignments::assignment01::small_exercises_grade::test::test_add_overflow"
    "assignments::assignment01::small_exercises_grade::test::test_sub_7_3"
    "assignments::assignment01::small_exercises_grade::test::test_sub_overflow"
)

TEST02=(
    "assignments::assignment02::small_exercises_grade::test::test_capitalize"
    "assignments::assignment02::small_exercises_grade::test::test_chooses"
    "assignments::assignment02::small_exercises_grade::test::test_fahrenheit"
    "assignments::assignment02::small_exercises_grade::test::test_gcd"
    "assignments::assignment02::small_exercises_grade::test::test_sum_array"
    "assignments::assignment02::small_exercises_grade::test::test_up3"
    "assignments::assignment02::small_exercises_grade::test::test_zip"
    "assignments::assignment02::vec_and_mat_grade::test::test_fibonacci"
    "assignments::assignment02::vec_and_mat_grade::test::test_inverse"
)

TEST03=(
    "assignments::assignment03::custom_operators_grade::test::test_my_and_then"
    "assignments::assignment03::custom_operators_grade::test::test_my_map"
    "assignments::assignment03::custom_operators_grade::test::test_my_option_op_or"
    "assignments::assignment03::parse_shell_grade::test::test_shell"
    "assignments::assignment03::small_exercises_grade::test::test_editor"
    "assignments::assignment03::small_exercises_grade::test::test_median"
    "assignments::assignment03::small_exercises_grade::test::test_mode"
    "assignments::assignment03::small_exercises_grade::test::test_next_weekday"
    "assignments::assignment03::small_exercises_grade::test::test_organize"
    "assignments::assignment03::small_exercises_grade::test::test_piglatin"
)

TEST04=(
    "assignments::assignment04::grade::test::test_context_calc_command"
    "assignments::assignment04::grade::test::test_context_calc_expression"
    "assignments::assignment04::grade::test::test_parse"
)

TEST06=(
    "assignments::assignment06::semiring_grade::test::test_123"
    "assignments::assignment06::semiring_grade::test::test_24x"
    "assignments::assignment06::semiring_grade::test::test_2x3_3x2_5x_12"
    "assignments::assignment06::semiring_grade::test::test_2x_3"
    "assignments::assignment06::semiring_grade::test::test_polynomial_f64"
    "assignments::assignment06::semiring_grade::test::test_polynomial_p_u64"
    "assignments::assignment06::semiring_grade::test::test_polynomial_u64"
    "assignments::assignment06::semiring_grade::test::test_polynomial_xy"
    "assignments::assignment06::semiring_grade::test::test_x"
    "assignments::assignment06::semiring_grade::test::test_x3"
    "assignments::assignment06::semiring_grade::test::test_x5_1"
    "assignments::assignment06::semiring_grade::test::test_zero_remove"
    "assignments::assignment06::symbolic_differentiation_grade::test::test_differentiate_complex"
    "assignments::assignment06::symbolic_differentiation_grade::test::test_differentiate_simple"
    "assignments::assignment06::symbolic_differentiation_grade::test::test_rational_arithmetic"
    "assignments::assignment06::symbolic_differentiation_grade::test::test_rational_arithmetic_long"
)

TEST07=(
    "assignments::assignment07::generator_grade::test::test_generator"
    "assignments::assignment07::my_itertools_grade::test::test_itertools"
    "assignments::assignment07::small_exercises_grade::test::test_fib_iter"
    "assignments::assignment07::small_exercises_grade::test::test_find"
    "assignments::assignment07::small_exercises_grade::test::test_find_usize"
    "assignments::assignment07::small_exercises_grade::test::test_large"
    "assignments::assignment07::small_exercises_grade::test::test_range_iter"
    "assignments::assignment07::small_exercises_grade::test::test_small"
    "assignments::assignment07::transform_grade::test::test_transform_identity"
    "assignments::assignment07::transform_grade::test::test_transform_repeat"
    "assignments::assignment07::transform_grade::test::test_transform_repeat_until_converge"
    "assignments::assignment07::transform_grade::test::test_transform_tuple"
)

TEST08=(
    "assignments::assignment08::church_grade::test::be_honest"
    "assignments::assignment08::church_grade::test::engineering_isnt_just_mathematics"
    "assignments::assignment08::church_grade::test::you_must_pass_these_examples"
    "assignments::assignment08::small_exercises_grade::test::test_count_repeat"
    "assignments::assignment08::small_exercises_grade::test::test_either2_map"
    "assignments::assignment08::small_exercises_grade::test::test_funny_map"
    "assignments::assignment08::small_exercises_grade::test::test_repeat"
)

TEST09=(
    "assignments::assignment09::bigint_grade::test::test_inf_prec_complex"
    "assignments::assignment09::bigint_grade::test::test_inf_prec_simple"
    "assignments::assignment09::matmul_grade::test::dot_product_test"
    "assignments::assignment09::matmul_grade::test::matmul_test"
    "assignments::assignment09::matmul_grade::test::vec_add_test"
    "assignments::assignment09::small_exercises_grade::test::test_calculate_mean"
    "assignments::assignment09::small_exercises_grade::test::test_find_count_n"
    "assignments::assignment09::small_exercises_grade::test::test_interleave3"
    "assignments::assignment09::small_exercises_grade::test::test_interleave_n"
    "assignments::assignment09::small_exercises_grade::test::test_is_fibonacci"
    "assignments::assignment09::small_exercises_grade::test::test_is_palindrome"
    "assignments::assignment09::small_exercises_grade::test::test_k_smallest_man"
    "assignments::assignment09::small_exercises_grade::test::test_position_median"
    "assignments::assignment09::small_exercises_grade::test::test_sigma"
    "assignments::assignment09::small_exercises_grade::test::test_sum_is_n"
    "assignments::assignment09::small_exercises_grade::test::test_two_dimensional_sum"
)

TEST10=(
    "assignments::assignment10::labyrinth_grade::test::can_every_husband_rescue_his_wife"
    "assignments::assignment10::small_exercises_grade::test::test_du_sort"
    "assignments::assignment10::small_exercises_grade::test::test_inversion"
    "assignments::assignment10::small_exercises_grade::test::test_natural_join"
    "assignments::assignment10::small_exercises_grade::test::test_pythagorean"
    "assignments::assignment10::small_exercises_grade::test::test_remove_duplicate"
    "assignments::assignment10::small_exercises_grade::test::test_remove_even"
    "assignments::assignment10::small_exercises_grade::test::test_traverse_preorder"
)

TEST11=(
    "assignments::assignment11::graph_grade::test_graph::test_graph"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_chunk_reverse"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_flatten"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_from_as_vec"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_insert"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_length"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_map"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_pair_map"
    "assignments::assignment11::linked_list_grade::test_linked_list::test_push_pop"
    "assignments::assignment11::mock_storage_grade::test_mock_storage::test_mock_storage"
    "assignments::assignment11::tv_room_grade::test_tv_room::test_tv_room"
)

TEST12=(
    "assignments::assignment12::card_grade::test_card::play"
    "assignments::assignment12::demux_grade::test_demux::test_demux"
    "assignments::assignment12::funnel_grade::test_funnel::test_funnel_concurrent"
    "assignments::assignment12::small_exercises_grade::test_pingpong::test_ping_pong"
    "assignments::assignment12::small_exercises_grade::test_pingpong::test_scoped_thread"
    "assignments::assignment12::small_exercises_grade::test_pingpong::test_scoped_thread_concurrent"
)

TEST13=(
    "assignments::assignment13::small_exercises_grade::test::dot_product_test"
    "assignments::assignment13::small_exercises_grade::test::matmul_test"
    "assignments::assignment13::small_exercises_grade::test::test_interleave3_par"
    "assignments::assignment13::small_exercises_grade::test::test_sigma_par"
    "assignments::assignment13::small_exercises_grade::test::vec_add_test"
)

get_test_cases() {
    local TEST_NAME=$1
    local TEST_CASES=()
    case $TEST_NAME in
        TEST01)
            TEST_CASES=("${TEST01[@]}")
            ;;
        TEST02)
            TEST_CASES=("${TEST02[@]}")
            ;;
        TEST03)
            TEST_CASES=("${TEST03[@]}")
            ;;
        TEST04)
            TEST_CASES=("${TEST04[@]}")
            ;;
        TEST06)
            TEST_CASES=("${TEST06[@]}")
            ;;
        TEST07)
            TEST_CASES=("${TEST07[@]}")
            ;;
        TEST08)
            TEST_CASES=("${TEST08[@]}")
            ;;
        TEST09)
            TEST_CASES=("${TEST09[@]}")
            ;;
        TEST10)
            TEST_CASES=("${TEST10[@]}")
            ;;
        TEST11)
            TEST_CASES=("${TEST11[@]}")
            ;;
        TEST12)
            TEST_CASES=("${TEST12[@]}")
            ;;
        TEST13)
            TEST_CASES=("${TEST13[@]}")
            ;;
        *)
            echo_err "Invalid test name: $TEST_NAME"
            exit 1
            ;;
    esac
    echo "${TEST_CASES[@]}"    
}
export -f get_test_cases
