# KAIST CS220: Programming Principles

## Logistics

- Instructor: [Jeehoon Kang](https://cp.kaist.ac.kr/jeehoon.kang)
- Time: Mon & Wed 14:30pm-15:45pm
- Place
  + Rm. 304, Bldg. E11. **YOUR PHYSICAL ATTENDANCE IS REQUIRED** unless announced otherwise.
  + [Zoom room](https://kaist.zoom.us/my/jeehoon.kang) (if remote participation is absolutely necessary). The passcode is announced at KLMS.
- Websites: <https://github.com/kaist-cp/cs220>, <https://gg.kaist.ac.kr/course/17/>
- Announcements: in [issue tracker](https://github.com/kaist-cp/cs220/issues?q=is%3Aissue+is%3Aopen+label%3Aannouncement)
  + We assume you read each announcement within 24 hours.
  + We strongly recommend you to watch the repository.
- TA: [Haechan An](https://cp.kaist.ac.kr/haechan.an), [Jungin Rhee](https://cp.kaist.ac.kr/jungin.rhee), [Woojin Lee](https://cp.kaist.ac.kr/woojin.lee)
  + Office Hour: Fri 9:15am-10:15am, Rm. 4432, Bldg. E3-1. If you want to come, do so by 9:30am.  See [below](https://github.com/kaist-cp/cs220#rules) for office hour policy.
    <!-- Fri 9:00am-12:00pm, [Zoom room](https://zoom.us/j/4842624821)(The passcode is same as the class). It is not required, but if you want to come, do so by 9:30am. See [below](#communication) for office hour policy. -->
- **IMPORTANT**: you should not expose your work to others. In particular, you should not fork
  the [upstream](https://github.com/kaist-cp/cs220) and push there.


## Course description

### Textbook

- [Slides](https://docs.google.com/presentation/d/17G3SwkE_tq0H3lTt9N0ysIbHhqDZBfHkoWD5LwwAKSo/edit#slide=id.p)
- [The Rust Book](https://doc.rust-lang.org/book/)


### Prerequisites

- It is **strongly recommended** that students already took courses on:

    + Mathematics (MAS101): proposition statement and proof
    + Programming (CS101): basic programming skills

- Without a proper understanding of these topics, you will likely struggle in this course.
- In the assignment, for the concepts that don't come out in prerequisite courses, we tried to comment on the relevant resources in the assignment code. Please read them carefully.


### Tools

Make sure that you're capable of using the following development tools:

- [Git](https://git-scm.com/): for downloading the homework skeleton and version-controlling your
  development. If you're not familiar with Git, walk through [this
  tutorial](https://www.atlassian.com/git/tutorials).

    + **IMPORTANT**: you should not expose your work to others. In particular, you should not fork
      the [upstream](https://github.com/kaist-cp/cs220) and push there. Please the following
      steps:

        * Directly clone the upstream without forking it.

          ```bash
          $ git clone --origin upstream git@github.com:kaist-cp/cs220.git
          $ cd cs220
          $ git remote -v
          upstream	git@github.com:kaist-cp/cs220.git (fetch)
          upstream	git@github.com:kaist-cp/cs220.git (push)
          ```

        * To get updates from the upstream, fetch and merge `upstream/main`.

          ```bash
          $ git fetch upstream
          $ git merge upstream/main
          ```

    + If you want to manage your development in a Git server, please create your own private
      repository.

        * You may upgrade your GitHub account to "PRO", which is free of charge.  
          Refer to the [documentation](https://education.github.com/students).

        * Set up your repository as a remote.

          ```bash
          $ git remote add origin git@github.com:<github-id>/cs220.git
          $ git remote -v
          origin	 git@github.com:<github-id>/cs220.git (fetch)
          origin	 git@github.com:<github-id>/cs220.git (push)
          upstream git@github.com:kaist-cp/cs220.git (fetch)
          upstream git@github.com:kaist-cp/cs220.git (push)
          ```

        * Push to your repository.

          ```bash
          $ git push -u origin main
          ```

- [Rust](https://www.rust-lang.org/): as the language of homework implementation. We chose Rust
  because its ownership type system greatly simplifies the development of large-scale system
  software.
    - We recommend you to read [this page](https://cp.kaist.ac.kr/helpdesk#technical-expertise) that describes how to study Rust.
    - `cargo doc --open` on your local will show you the documentation for each homework.

- [Visual Studio Code](https://code.visualstudio.com/) (optional): for developing your homework. If you prefer other editors, you're good to go.

- [ChatGPT](https://chat.openai.com/) or other LLMs (optional): for your homework.
  - In the era of AI, we believe that it is crucial to learn how to wisely use AI in programming.
  - So we adjusted the difficulty of homework assuming that you'll use ChatGPT 3.5 (or equivalent) to solve it.

- [Development server](https://cloud.fearless.systems/)

    + **IMPORTANT: Don't try to hack. Don't try to freeze the server. Please be nice.**

    + Now you can [use it as a VSCode remote server as in the video](https://www.youtube.com/watch?v=TTVuUIhdn_g&list=PL5aMzERQ_OZ8RWqn-XiZLXm1IJuaQbXp0&index=3).

    + [NOTE: We recommend the `rust-analyzer` plugin instead of `rls`](https://github.com/rust-analyzer/rust-analyzer).

    + [NOTE: If permission denied error occurs when trying to install `CodeLLDB Extension` into the 
      remote server](https://github.com/kaist-cp/cs420/issues/5), please follow the steps: 
      1. Download [this file](https://github.com/vadimcn/vscode-lldb/releases/download/v1.5.0/codelldb-x86_64-linux.vsix) at the remote server.
      1. Follow [the instructions](https://code.visualstudio.com/docs/editor/extension-gallery#_install-from-a-vsix) to install it.

    + [NOTE: If you cannot connect to the remote server via VSCode with `fail to create hard link` error message](https://github.com/kaist-cp/cs420/issues/91), please follow the steps:
      1. Close VSCode window and try to connect to the remote server via terminal(or cmd). If you encounter `Connection timed out` error message, try again after a few minutes.
      1. Delete all the files in `~/.vscode-server/bin/`.


## Grading & honor code

### Cheating

**IMPORTANT: PAY CLOSE ATTENTION. VERY SERIOUS.**

- Please sign the KAIST CS Honor Code for this semester.
  Otherwise, you may be expelled from the course.

- We will use sophisticated tools for detecting code plagiarism​.

    + [Google "code plagiarism detector" for images](https://www.google.com/search?q=code+plagiarism+detector&tbm=isch) and see how these tools can detect "sophisticated" plagiarisms.
      You really cannot escape my catch. Just don't try plagiarism in any form.

### Programming assignments (40%)

- We'll announce **all** assignments before the semester begins.
- Submit your solution to <https://gg.kaist.ac.kr/course/17>.
- **How to submit your assignment:**
  - To submit your solution, you should run `submit.sh` in `scripts` directory. In other words, you should run the following command:
  ```bash
  # Run this command at the root directory of this repository.
  $ ./scripts/submit.sh <assignment_number>

  # E.g. To submit `assignment09`, run the following command:
  $ ./scripts/submit.sh 9
  ```
  - After running the command above, in the `target` directory, you can find a `assignment<NUMBER>.zip` file (`assignment09.zip` for example). Submit this file to <https://gg.kaist.ac.kr/course/17>.
- Read the documentation at <https://cp.kaist.ac.kr/cs220/cs220/>.
- You can check your grade of each assignment by running the grading script.
  - You can run the grading script with the following command:
  ```bash
  $ ./scripts/grade.sh <assignment_number>
  
  # E.g. To grade `assignment09`, run the following command:
  $ ./scripts/grade.sh 9
  ```
- You're **allowed** to use ChatGPT or other LLMs. Instead, you'll solve more problems than previous semesters.


### Midterm and final exams (60%)

- Date & Time: Oct 18 (midterm) and Dec 13 (final), 13:00pm-15:45pm (or shorter, TBA)

- Place: Rm. 304, Bldg. E11, KAIST

- Your physical apperance is required. If online participation is **absolutely necessary**, we'll use Zoom.

- You should bring your own laptop. (You can also borrow one from School of Computing Admin Team.)

- We will use [Safe Exam Browser](https://safeexambrowser.org/) to prevent cheatings.
  - You should have your laptop configured with Safe Exam Browser before the exam. 
  - TBA: Details will be announced later. 

### Attendance (?%)

- You should solve a quiz at the [Course Management](https://gg.kaist.ac.kr/course/17) website for each session. **You should answer the quiz by the end of the day.**

- If you miss a significant number of sessions, you'll automatically get an F.


## Communication

### Registration

- Make sure you can log in the [lab submission website](https://gg.kaist.ac.kr).

    + Log in with your `kaist-cp-class` account.

    + Your id is your `@kaist.ac.kr` email address.

    + Reset your password here: https://auth.fearless.systems/if/flow/default-recovery-flow/

    + If you cannot log in, please contact the instructor.

### Rules

- Course-related announcements and information will be posted on the
  [website](https://github.com/kaist-cp/cs220) as well as on the [GitHub issue
  tracker](https://github.com/kaist-cp/cs220/issues).  You are expected to read all
  announcements within 24 hours of their being posted.  It is highly recommended to watch the
  repository so that new announcements will automatically be delivered to your email address.

- Ask questions on course materials and assignments in [this repository's issue tracker](https://github.com/kaist-cp/cs220/issues).
    + Don't send emails to the instructor or TAs for course materials and assignments.
    + Before asking a question, ask it to [ChatGPT](https://chat.openai.com/). Or search for it in Google and Stack Overflow.
    + Describe your question in as much detail as possible. It should include the following things:
      * Environment (OS, gcc, g++ version, and any other related program information).
      * Command(s) that you used and the result. Any logs should be formatted in code. Refer to [this](https://guides.github.com/features/mastering-markdown/).
      * Any directory or file changes you've made. If it is the solution file, just describe which part of the code is modified.
      * Googling result. Search before asking, and share the keyword used for searching and what you've learned from it.
    + Give a proper title to your issue.
    + Read [this](https://github.com/kaist-cp/cs220#communication) for more instructions.
    + Questions will be answered within 2 days mostly.

    + I'm requiring you to ask questions online first for two reasons. First, clearly writing a
      question is the first step to reaching an answer. Second, you can benefit from the questions and answers of other students.

- Ask your questions via email **only if** they are either confidential or personal. Any questions
   failing to do so (e.g. email questions on course materials) will not be answered.

- We are NOT going to discuss *new* questions during office hours. Before coming to the office
  hour, please check if there is a similar question on the issue tracker. If there isn't, file a new
  issue and start discussion there. The agenda of the office hour will be the issues that are not
  resolved yet.

- Emails to the instructor or the head TA should begin with "CS220:" in the subject line, followed
  by a brief description of the purpose of your email. The content should at least contain your name
  and student number. Any emails failing to do so (e.g. emails without student number) will not be
  answered.

- If you join the session remotely from Zoom (https://kaist.zoom.us/my/jeehoon.kang),
  your Zoom name should be `<your student number> <your name>` (e.g., `20071163 강지훈`).
  Change your name by referring to [this](https://support.zoom.us/hc/en-us/articles/201363203-Customizing-your-profile).

- This course is conducted in English. But you may ask questions in Korean. Then I will translate it to English.

## Ignore

1830eaed90e5986c75320daaf131bd3730b8575e866c4e92935a690e7c2a0716
