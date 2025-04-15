in crate crates/tools 
there is a bin commit.rs

goals:
- optimize code
- add unit tests
- allow to choose between build in (llm_response.rs) solution for generating commit message and any openai api compatible endpoint by setting env vars COMMIT_BASEURL and  COMMIT_MODEL .
- add feature to project to make mistralrs crate optional if we going to use api instead
- implement openai api compatible commit message generation if mistral feature is disabled (default)

instructions:
- create implementation plan and write it into docs/task-01-plan.md
- split plan into tasks in markdown notation like
[] task 1
[] task 2

- after each task finished mark it as done in plan file
- if there is a bug in the code put it into docs/BUGS.md in same way as tasks
- try to fix bug if its in scope of task or its easy to fix. mark fixed tasks as [x] but dont remove them from list
- all dependencies are defined into workspace, and crates use dependencies with .workspace=true 
- implement goals 

documentation:
- you can always check any crates used in this repo from workspace directory in : docs/libs/