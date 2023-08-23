#!/usr/bin/env bash

set -e
set -uo pipefail
IFS=$'\n\t'

# Imports library.
BASEDIR=$(dirname "$0")
source $BASEDIR/test_cases.sh
source $BASEDIR/grade-utils.sh

RUNNERS=(
    "cargo"
    "cargo --release"
)

# Lints.
run_linters || exit 0

# Executes test for each runner.
for RUNNER in "${RUNNERS[@]}"; do
    echo "Running with $RUNNER..."
    echo "Below lines will show only failed tests."

    ASSIGNMENT=$(printf "assignment%02d" $1)
    TEST_NAME=$(printf "TEST%02d" $1)
    TESTS=$(get_test_cases $TEST_NAME)

    # Runs tests.
    SCORE=$(run_tests)
done

echo Your score: ${SCORE}

exit $SCORE
