# Assignment 5: Program correctness and logic

* The primary goal of this assignment is to grasp basic concepts about proving a program's correctness with deductive reasoning.

* You should fill in `TODO`s in three files: `max.mlw`, `binary_search.mlw`, `pascal.mlw`.
  * You will get PARTIAL SCOREs for each of those three files.
  * E.g. If `max.mlw` and `binary_search.mlw` get passed, 2 out of 3 points will be given.

* You may use [Why3 in your browser](https://www.why3.org/try/).
  * Clicking `Verify` button at the top will open a panel on the right side.
  * For each task in the panel (e.g. `loop invariant preservation`), you can right-click it and run the prover.
    * Important: The prover might not be able to verify the correct solution if the number of steps is too small. Make sure to test with 1000~5000 steps.
  * Fill in `TODO`s until the prover can verify all tasks, notified with green check-marks.

* To submit your solution, run `./scripts/submit.sh` and submit `assignment05.zip` in the `target` directory to gg.

* [More on Why3](https://www.why3.org/doc/).
* [Why3 standard library](https://www.why3.org/stdlib/).
