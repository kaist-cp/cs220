# KAIST CS220: Programming Principles

## Logistics

- Instructor: [Jeehoon Kang](https://cp.kaist.ac.kr/jeehoon.kang)
- Time: Mon & Wed 14:30-15:45 (2024 Fall)
- Place
  + Rm. 1501, Bldg. E3-1. **YOUR PHYSICAL ATTENDANCE IS REQUIRED** unless announced otherwise.
  + [Zoom room](https://kaist.zoom.us/my/jeehoon.kang) (if remote participation is absolutely necessary).
    Ask Jeehoon via email for the passcode.
- Websites: <https://github.com/kaist-cp/cs220>, <https://gg.kaist.ac.kr/course/20/>
- Announcements: in [issue tracker](https://github.com/kaist-cp/cs220/issues?q=is%3Aissue+is%3Aopen+label%3Aannouncement)
  + We assume you read each announcement within 24 hours.
  + We strongly recommend you to watch the repository.
- TA: [Janggun Lee](https://cp.kaist.ac.kr/janggun.lee) (head), [Sunho Park](https://cp.kaist.ac.kr/sunho.park), [Jaewoo Kim](https://cp.kaist.ac.kr/jaewoo.kim).
  + Office Hours: Fri 9:00-10:00, Rm. 4441, Bldg. E3-1.
    If you want to attend, please email the TAs by the day before and arrive by 9:15.
    See [below](https://github.com/kaist-cp/cs220#rules) for the office hour policy.
    <!-- Fri 9:00am-12:00pm, [Zoom room](https://zoom.us/j/4842624821)(The passcode is same as the class). It is not required, but if you want to come, do so by 9:30am. See [below](#communication) for office hour policy. -->
- **IMPORTANT**: you should not expose your work to others. In particular, you should not fork
  the [upstream](https://github.com/kaist-cp/cs220) and push there.


## Course description

This course will equip you with the programming concepts needed to effectively communicate your ideas to computers.
For a detailed overview, please refer to the introduction in the Google Slides presentation.

### Outline

- Part 1: CS101 Review using the Rust programming language
- Part 2: Type and Correctness
- Part 3: Reference and Ownership
- Part 4: Function and Iterator
- Part 5: Concurrency and Parallelism

### Textbook

- [Slides](https://docs.google.com/presentation/d/17G3SwkE_tq0H3lTt9N0ysIbHhqDZBfHkoWD5LwwAKSo/edit#slide=id.p)
- [The Rust Book](https://doc.rust-lang.org/book/)


### Prerequisites

- It is **strongly recommended** that students have completed courses in:

    + Mathematics (MAS101): proposition statement and proof
    + Programming (CS101): basic programming skills

  A solid foundation in these areas is crucial for success in this course.


### Schedule

- Week 01: introduction
- Week 02: part 1 (CS101 review)
- Week 03: part 1 (CS101 review)
- Week 04: part 1 (CS101 review)
- Week 05: part 2 (type)
- Week 06: part 2 (correctness)
- Week 07: part 2 (type as automatic verification)
- Week 08: mid-term exam
- Week 09: part 3 (reference)
- Week 10: part 3 (ownership)
- Week 11: part 4 (function as a value)
- Week 12: part 4 (combinator)
- Week 13: part 5 (mutability)
- Week 14: part 5 (concurrency)
- Week 15: part 5 (parallelism)
- Week 16: final exam


### Tools

Ensure you are proficient with the following development tools:

- [Git](https://git-scm.com/): Essential for downloading homework templates and managing your development process.
  If you're new to Git, please complete [this tutorial](https://www.atlassian.com/git/tutorials).

    + Follow these steps to set up your repository:
        * Clone the upstream repository directly without forking it:
          ```bash
          $ git clone --origin upstream https://github.com/kaist-cp/cs220.git
          $ cd cs220
          $ git remote -v
          upstream        https://github.com/kaist-cp/cs220.git (fetch)
          upstream        https://github.com/kaist-cp/cs220.git (push)
          ```
        * To receive updates from the upstream, fetch and merge `upstream/main`:
          ```bash
          $ git fetch upstream
          $ git merge upstream/main
          ```

    + For managing your development on a Git server, create a private repository:
        * Upgrade to a "PRO" GitHub account, available at no cost.
          See the [documentation](https://education.github.com/students).
        * Go to https://github.com and create a new private repository named `cs220`.
        The repository should be initialized as empty.
        * Configure your repository as a remote:
          ```bash
          $ git remote add origin git@github.com:<github-id>/cs220.git
          $ git remote -v
          origin	 git@github.com:<github-id>/cs220.git (fetch)
          origin	 git@github.com:<github-id>/cs220.git (push)
          upstream https://github.com/kaist-cp/cs220.git (fetch)
          upstream https://github.com/kaist-cp/cs220.git (push)
          ```
        * Push your work to your repository:
          ```bash
          $ git push -u origin main
          ```
        * If you're accessing your repository for the first time, git may display an error due to insufficient permissions.
          * On a development server, copy the SSH public key mentioned in the error message and add it to your GitHub account.
          * In your own environment, you may need to generate an SSH key using `ssh-keygen` and retrieve the public key from `~/.ssh/id_rsa.pub`. You should never share the private key located in `~/.ssh/id_rsa`.
          * You can add SSH keys to your GitHub account [here](https://github.com/settings/keys).
- [Rust](https://www.rust-lang.org/): You will use Rust for homework.
  We chose Rust because its ownership type system greatly simplifies the development of large-scale system software.
  + `cargo doc --open` on your local will show you the documentation for each homework.

- [ChatGPT](https://chat.openai.com/) or other Large Language Models (LLMs) (optional): Useful for completing your homework.
    + In an AI-driven era, learning to effectively utilize AI in programming is crucial.
      Homework difficulty is adjusted assuming the use of ChatGPT 3.5 or an equivalent tool.

- [Visual Studio Code](https://code.visualstudio.com/) (optional): Recommended for developing your homework, although you may use any editor of your preference.

- [Single Sign On (SSO)](https://auth.fearless.systems/): Use the following SSO credentials to access [gg](https://gg.kaist.ac.kr) and the [development server](https://cloud.fearless.systems):
    + id: KAIST student id (8-digit number)
    + email: KAIST email address (@kaist.ac.kr)
    + password: Reset it here: <https://auth.fearless.systems/if/flow/default-recovery-flow/>
    + Log in to [gg](https://gg.kaist.ac.kr) using the "kaist-cp-class" option, and to the [development server](https://cloud.fearless.systems) using the "OpenID Connect" option.

- [Development Server](https://cloud.fearless.systems/):
    + **IMPORTANT: Do not attempt to hack or overload the server. Please use it responsibly.**
    + Create and connect to a workspace to use the terminal or VSCode (after installation).
    + We recommend using VSCode with the "Rust Analyzer" and "CodeLLDB" plugins.


## Grading & Honor code

### Cheating

**IMPORTANT: READ CAREFULLY. THIS IS A SERIOUS MATTER.**

- Sign the KAIST CS Honor Code for this semester.
  Failure to do so may lead to expulsion from the course.

- We will employ sophisticated tools to detect code plagiarism.
    + Search for "code plagiarism detector" on Google Images to understand how these tools can identify advanced forms of plagiarism.
      Do not attempt plagiarism in any form.

### Programming assignments (40%)

- All assignments will be announced at the start of the semester.
- Submit your solutions to <https://gg.kaist.ac.kr/course/20>.
- You are **permitted** to use ChatGPT or other LLMs.
- For the concepts that don't come out in prerequisite courses, we tried to comment on the relevant resources in the assignment code. Please read them carefully.
- **How to submit your assignment:**
  - To submit your solution, you should run `submit.sh` in `scripts` directory. In other words, you should run the following command:
    ```bash
    # Run this command at the root directory of this repository.
    $ ./scripts/submit.sh
    ```
  - After running the command above, in the `target` directory, you can find a `assignment<NUMBER>.zip` file (`assignment09.zip` for example). Submit this file to <https://gg.kaist.ac.kr/course/20>.
- Read the documentation at <https://kaist-cp.github.io/cs220/cs220/>.
- You can check your grade of each assignment by running the grading script.
  - You can run the grading script with the following command:
  ```bash
  $ ./scripts/grade.sh <assignment_number>

  # E.g. To grade `assignment09`, run the following command:
  $ ./scripts/grade.sh 9
  ```


### Midterm and final exams (60%)

- Dates & Times: Oct 23th (Wed), Dec 18th (Wed), 13:00-15:45 (or shorter, TBA)

- Location: (the same as usual)

- Physical attendance is required.
  If necessary, online participation via Zoom will be accommodated.

- You are expected to bring your own laptop.
  Laptops can also be borrowed from the School of Computing Administration Team.

- We will use [Safe Exam Browser](https://safeexambrowser.org/) to prevent cheatings.
  - You should have your laptop configured with Safe Exam Browser before the exam.
  - TBA: Details will be announced later.

### Attendance (?%)

- A quiz must be completed on the [Course Management](https://gg.kaist.ac.kr/course/20) website for each session (if any).
  **Quizzes should be completed by the end of the day.**

- Failing to attend a significant number of sessions will result in an automatic grade of F.


## Communication

### Registration

- Ensure your ability to log into the [lab submission website](https://gg.kaist.ac.kr).
    + Use your `kaist-cp-class` account for login.
    + Your ID is your `@kaist.ac.kr` email address.
    + Reset your password here: [https://auth.fearless.systems/if/flow/default-recovery-flow/](https://auth.fearless.systems/if/flow/default-recovery-flow/)
    + Contact the instructor if login issues arise.

### Rules

- Course-related announcements and information will be posted on the [course website](https://github.com/kaist-cp/cs220) and the [GitHub issue tracker](https://github.com/kaist-cp/cs220/issues).
  It is expected that you read all announcements within 24 hours of their posting.
  Watching the repository is highly recommended for automatic email notifications of new announcements.

- Questions about course materials and assignments should be posted in [the course repository's issue tracker](https://github.com/kaist-cp/cs220/issues).
    + Avoid sending emails to the instructor or TAs regarding course materials and assignments.
    + Research your question using Google, Stack Overflow, and ChatGPT before posting.
    + Describe your question in detail, including:
        * Environment (OS, Rust version, and other relevant program information).
        * Used commands and their results, with logs formatted in code.
          See [this guide](https://guides.github.com/features/mastering-markdown/).
        * Any changes made to directories or files.
          For solution files, describe the modified code sections.
        * Your Google search results, including search terms and learned information.
    + Use a clear and descriptive title for your issue.
    + For further instructions, read [this section](https://github.com/kaist-cp/cs220#communication) on the course website.
    + The requirement to ask questions online first is twofold: It ensures clarity in your query and allows everyone to benefit from shared questions and answers.

- Email inquiries should be reserved for confidential or personal matters.
  Questions not adhering to this guideline (e.g., course material queries via email) will not be addressed.

- Office hours will not cover *new* questions.
  Check the issue tracker for similar questions before attending.
  If your question is not listed, post it as a new issue for discussion.
  Office hour discussions will focus on unresolved issues.

- Emails to the instructor or head TA should start with "CS220" in the subject line, followed by a brief description.
  Include your name and student number in the email.
  Emails lacking this information (e.g., those without a student number) will not receive a response.

- If attending remotely via Zoom (https://kaist.zoom.us/my/jeehoon.kang), set your Zoom name to `<your student number> <your name>` (e.g., `20071163 강지훈`).
  Instructions for changing your Zoom name can be found [here](https://support.zoom.us/hc/en-us/articles/201363203-Customizing-your-profile).

- The course is conducted in English.
  However, you may ask questions in Korean, which will be translated into English.

## Ignore

1830eaed90e5986c75320daaf131bd3730b8575e866c4e92935a690e7c2a0716
