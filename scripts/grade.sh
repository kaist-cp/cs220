#!/usr/bin/env bash

set -e
set -uo pipefail
IFS=$'\n\t'

# Imports library.
BASEDIR=$(dirname "$0")
source $BASEDIR/grade-utils.sh

RUNNER="cargo"

# Lints.
run_linters || exit 1

# Executes test for each runner.
echo "Running with $RUNNER..."

if [ $# != 1 ]
  then
    echo "==============================="
    echo "Invalid argument."
    echo "Usage: './grade.sh <assignment_number>'"
    echo "Example: './grade.sh 1' to grade assignment01"
    exit 1
fi
TEST_NAME=$(printf "TEST%02d" $1)
case $TEST_NAME in
    TEST01)
        TESTS=(
            "assignments::assignment01::small_exercises_grade::test"
        )
        ;;
    TEST02)
        TESTS=(
            "assignments::assignment02::small_exercises_grade::test"
            "assignments::assignment02::vec_and_mat_grade::test"
        )
        ;;
    TEST03)
        TESTS=(
            "assignments::assignment03::custom_operators_grade::test"
            "assignments::assignment03::parse_shell_grade::test"
            "assignments::assignment03::small_exercises_grade::test"
        )
        ;;
    TEST04)
        TESTS=(
            "assignments::assignment04::grade::test"
        )
        ;;
    TEST06)
        TESTS=(
            "assignments::assignment06::semiring_grade::test"
            "assignments::assignment06::symbolic_differentiation_grade::test"
        )
        ;;
    TEST07)
        TESTS=(
            "assignments::assignment07::generator_grade::test"
            "assignments::assignment07::my_itertools_grade::test"
            "assignments::assignment07::small_exercises_grade::test"
            "assignments::assignment07::transform_grade::test"
        )
        ;;
    TEST08)
        TESTS=(
            "assignments::assignment08::church_grade::test"
            "assignments::assignment08::small_exercises_grade::test"
        )
        ;;
    TEST09)
        TESTS=(
            "assignments::assignment09::bigint_grade::test"
            "assignments::assignment09::small_exercises_grade::test"
            "assignments::assignment09::matmul_grade::test"
        )
        ;;
    TEST10)
        TESTS=(
            "assignments::assignment10::labyrinth_grade::test"
            "assignments::assignment10::small_exercises_grade::test"
        )
        ;;
    TEST11)
        TESTS=(
            "assignments::assignment11::graph_grade::test_graph"
            "assignments::assignment11::linked_list_grade::test_linked_list"
            "assignments::assignment11::mock_storage_grade::test_mock_storage"
            "assignments::assignment11::tv_room_grade::test_tv_room"
        )
        ;;
    TEST12)
        TESTS=(
            "assignments::assignment12::card_grade"
            "assignments::assignment12::demux_grade::test_demux"
            "assignments::assignment12::funnel_grade::test_funnel"
            "assignments::assignment12::small_exercises_grade::test_pingpong"
        )
        ;;
    TEST13)
        TESTS=(
            "assignments::assignment13::small_exercises_grade::test"
        )
        ;;
    *)
    echo_err "Invalid assignment number: $1"
    echo_err "The assignment number should be between 1 and 12, excluding 5."
    exit 1
    ;;
esac

# Runs tests.
SCORE=$(run_tests)
NUM_TESTS=${#TESTS[@]}
echo Your score: ${SCORE}/${NUM_TESTS}
