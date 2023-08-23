#!/usr/bin/env bash

TEST01=(
    "assignments::assignment01::small_exercises_grade::test"
)

TEST02=(
    "assignments::assignment02::small_exercises_grade::test"
    "assignments::assignment02::vec_and_mat_grade::test"
)

TEST03=(
    "assignments::assignment03::custom_operators_grade::test"
    "assignments::assignment03::parse_shell_grade::test::test_shell"
    "assignments::assignment03::small_exercises_grade::test"
)

TEST04=(
    "assignments::assignment04::grade::test"
)

TEST06=(
    "assignments::assignment06::semiring_grade::test"
    "assignments::assignment06::symbolic_differentiation_grade::test"
)

TEST07=(
    "assignments::assignment07::generator_grade::test"
    "assignments::assignment07::my_itertools_grade::test"
    "assignments::assignment07::small_exercises_grade::test"
    "assignments::assignment07::transform_grade::test"
)

TEST08=(
    "assignments::assignment08::church_grade::test"
    "assignments::assignment08::small_exercises_grade::test"
)

TEST09=(
    "assignments::assignment09::bigint_grade::test"
    "assignments::assignment09::small_exercises_grade::test"
)

TEST10=(
    "assignments::assignment10::labyrinth_grade::test"
    "assignments::assignment10::small_exercises_grade::test"
)

TEST11=(
    "assignments::assignment11::graph_grade::test_graph"
    "assignments::assignment11::linked_list_grade::test_linked_list"
    "assignments::assignment11::mock_storage_grade::test_mock_storage"
    "assignments::assignment11::tv_room_grade::test_tv_room"
)

TEST12=(
    "assignments::assignment12::card_grade"
    "assignments::assignment12::demux_grade::test_demux"
    "assignments::assignment12::funnel_grade::test_funnel"
    "assignments::assignment12::small_exercises_grade::test_pingpong"
)

TEST13=(
    "assignments::assignment13::small_exercises_grade::test"
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
