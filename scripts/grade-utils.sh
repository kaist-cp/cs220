#!/usr/bin/env bash

# Global variables
# * TEMPLATE_REV: git revision of the latest homework template
# * TESTS: array of "[TARGET] [TEST_NAME] [-- <args>...]"
#   e.g. "--test linked_list", "--lib cache", "--test list_set -- --test-thread 1"
# * RUNNERS: array of "cargo[_asan | _tsan] [--release]"
# * TIMEOUT: default 10s


echo_err() {
    echo "$@" 1>&2
}
export -f echo_err

# check_diff FILE TEST_LINES_FROM_TAIL
# Abort if "--lib" tests are modified.
# Uses global variable TEMPLATE_REV.
check_diff() {
    local FILE=$1
    local TAIL_N=$2
    diff <(tail -n $TAIL_N <(git show $TEMPLATE_REV:$FILE)) <(tail -n $TAIL_N $FILE) \
        || (echo_err "You modified tests for ${FILE}!"; exit 1)
}
export -f check_diff

# Returns non-zero exit code if any of the linters have failed.
run_linters() {
    cargo fmt -- --check
    local FMT_ERR=$?
    cargo clippy -- -D warnings
    local CLIPPY_ERR=$?
    [ "$FMT_ERR" -ne 0 ] && echo_err 'Please format your code with `cargo fmt` first.'
    [ "$CLIPPY_ERR" -ne 0 ] && echo_err 'Please fix the issues from `cargo clippy` first.'
    return $(( FMT_ERR || CLIPPY_ERR ))
}
export -f run_linters

# usage: _run_tests_with CARGO [OPTIONS]
# example: _run_tests_with cargo_tsan --release
# Echos number of failed tests to stdout.
# Echos error message to stderr.
# Uses global variables TESTS, TIMEOUT.
# [OPTIONS] must not contain " -- " (cargo options only).
_run_tests_with() {
    local CARGO=$1; shift
    local MSGS # https://mywiki.wooledge.org/BashPitfalls#local_var.3D.24.28cmd.29
    MSGS=$($CARGO test --no-run "$@" 2>&1)
    if [ $? -ne 0 ]; then
        echo_err "Build failed! Error message:"
        echo_err "${MSGS}"
        echo_err "--------------------------------------------------------------------------------"
        echo ${#TESTS[@]} # failed all tests
        exit 1
    fi

    local PASSED=0
    # local NUM_TESTS=$(echo $TESTS | wc -w)
    for TEST in ${TESTS[@]}; do
        local TEST_CMD="$CARGO test $* --lib -- $TEST"
        # card_game in Assignment12 takes 20 seconds.
        timeout ${TIMEOUT:-22s} bash -c "$TEST_CMD 2> /dev/null" 1>&2
        case $? in
            0)   PASSED=$((PASSED + 1));;
            124) echo_err "Test timed out: $TEST_CMD";;
            *)   echo_err "Test failed:    $TEST_CMD";;
        esac
    done

    echo $PASSED
}

# example: run_tests
# Uses global variable RUNNER and TESTS
run_tests() {
    # "cargo --release" should be split into "cargo" and "--release"
    local IFS=' '
    _run_tests_with $RUNNER
}
export -f run_tests
