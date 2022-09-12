# KAIST CS220: Programming Principles

## Logistics

- Instructor: [Jeehoon Kang](https://cp.kaist.ac.kr/jeehoon.kang)
- Time: Tue & Thu 13:00pm-14:15pm
- Place
  + Rm. 1501, Bldg. E3-1. **YOUR PHYSICAL ATTENDANCE IS REQUIRED** unless announced otherwise.
  + [Zoom room](https://kaist.zoom.us/my/jeehoon.kang) (if remote participation is absolutely necessary). The passcode is announced at KLMS.
- Websites: https://github.com/kaist-cp/cs220 , https://gg.kaist.ac.kr/course/12
- Announcements: in [issue tracker](https://github.com/kaist-cp/cs220/issues?q=is%3Aissue+is%3Aopen+label%3Aannouncement)
  + We assume you read each announcement within 24 hours.
  + We strongly recommend you to watch the repository.
- TA: [Minseong Jang](https://cp.kaist.ac.kr/minseong.jang), [Jaemin Choi](https://cp.kaist.ac.kr/jaemin.choi), [Seungmin Jeon](https://cp.kaist.ac.kr/seungmin.jeon), [Jaehwang Jung](https://cp.kaist.ac.kr/jaehwang.jung)
  + Office Hour: Fri 9:00am-10:00am, [Zoom room](https://zoom.us/j/71923059728) (passcode: same as the class, announced at KLMS.). It is not required, but if you want to come, do so by 9:15am.  See [below](https://github.com/kaist-cp/cs220#rules) for office hour policy.
    <!-- Fri 9:00am-12:00pm, [Zoom room](https://zoom.us/j/4842624821)(The passcode is same as the class). It is not required, but if you want to come, do so by 9:30am. See [below](#communication) for office hour policy. -->



## Course description

### Textbook

- [Slides](https://docs.google.com/presentation/d/17G3SwkE_tq0H3lTt9N0ysIbHhqDZBfHkoWD5LwwAKSo/edit#slide=id.p)
- [The Rust Book](https://doc.rust-lang.org/book/)


### Prerequisites

- It is **strongly recommended** that students already took courses on:

    + Mathematics (MAS101): proposition statement and proof
    + Programming (CS101): basic programming skills

  Without a proper understanding of these topics, you will likely struggle in this course.


### Tools

Make sure you're capable of using the following development tools:

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

- You can connect to server by `ssh s<student-id>@cp-service.kaist.ac.kr -p13000`, e.g., `ssh s20071163@cp-service.kaist.ac.kr -p13000`.

    + **IMPORTANT: Don't try to hack. Don't try to freeze the server. Please be nice.**

    + Your initial password is `123454321`. IMPORTANT: you should change it ASAP.

    + I require you to register public SSH keys to the server. (In March, we'll expire your password so that you can log in only via SSH keys.)
      See [this tutorial](https://serverpilot.io/docs/how-to-use-ssh-public-key-authentication/) for more information on SSH public key authentication.
      Use `ed25519`.

    + In your client, you may want to set your `~/.ssh/config` as follows for easier SSH access:

      ```
      Host cs220
        Hostname cp-service.kaist.ac.kr
        Port 13000
        User s20071163
      ```

      Then you can connect to the server by `ssh cs220`.

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

For details, see <https://gg.kaist.ac.kr/course/12>.


### Midterm and final exams (60%)

- Date & Time: TBA (midterm) and TBA (final), 13:00pm-15:45pm (or shorter, TBA)

- Place: Rm. 1501, Bldg. E3-1, KAIST

    + If online participation is absolutely necessary, we'll use Zoom.


### Attendance (?%)

- You should solve a quiz at the [Course Management](https://gg.kaist.ac.kr/course/12) website for each session. **You should answer to the quiz by the end of the day.**

- If you miss a significant number of sessions, you'll automatically get an F.


## Communication

### Registration

- Make sure you can log in the [lab submission website](https://gg.kaist.ac.kr).

    + Reset your password here: https://gg.kaist.ac.kr/accounts/password_reset/
      The email address is your `@kaist.ac.kr` address.

    + The id is your student id (e.g., 20071163).

    + Log in with your student id and the new password.
      If you cannot, please contact the head TA in the chat.

### Rules

- Course-related announcements and information will be posted on the
  [website](https://github.com/kaist-cp/cs220) as well as on the [GitHub issue
  tracker](https://github.com/kaist-cp/cs220/issues).  You are expected to read all
  announcements within 24 hours of their being posted.  It is highly recommended to watch the
  repository so that new announcements will automatically be delivered to you email address.

- Ask questions on course materials and assignments in [this repository's issue tracker](https://github.com/kaist-cp/cs220/issues).
    + Don't send emails to the instructor or TAs for course materials and assignments.
    + Before asking a question, search it in Google and Stack Overflow.
    + Describe your question as detailed as possible. It should include following things:
      * Environment (OS, gcc, g++ version, and any other related program information).
      * Command(s) that you used and the result. Any logs should be formatted in code. Refer to [this](https://guides.github.com/features/mastering-markdown/).
      * Any directory or file changes you've made. If it is solution file, just describe which part of the code is modified.
      * Googling result. Search before asking, and share the keyword used for searching and what you've learned from it.
    + Give a proper title to your issue.
    + Read [this](https://cp-git.kaist.ac.kr/cs220/cs220#communication) for more instructions.

    + I'm requiring you to ask questions online first for two reasons. First, clearly writing a
      question is the first step to reach an answer. Second, you can benefit from questions and
      answers of other students.

- Ask your questions via email **only if** they are either confidential or personal. Any questions
   failing to do so (e.g. email questions on course materials) will not be answered.

- We are NOT going to discuss *new* questions during the office hour. Before coming to the office
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
