main focus:
 - crate crates/builder
    - providers
    - tools
 - crate crates/aiprovider


goals:
- optimize code
- add unit tests
- move providers implementation into crates/aiprovider
- implement mistral provider
- implement anthropic provider
- create enums for well known models for mistral and anthropic
- create similar enums for other providers, but there need to be valid option to pass custom model like now.
- change implementation of tests to reflect changees in code
- write documentation as docstrings
- implement new unit tests to cover new functionality

instructions:
- create implementation plan and write it into docs/task-02-plan.md
- split plan into tasks in markdown notation like
[] task 1
[] task 2

- after each task finished mark it as done in plan file
- if there is a bug in the code put it into docs/BUGS.md in same way as tasks
- try to fix bug if its in scope of task or its easy to fix. mark fixed tasks as [x] but dont remove them from list
- all dependencies are defined into workspace, and crates use dependencies with .workspace=true 
- implement goals 
- agent resonse is prepared for to be part of agent implementation, for now it is just structurized output. thats how we want to communicate with llm in the future.


documentation:
- you can always check any crates used in this repo from workspace directory in : docs/libs/
- current task definition is in docs/task-02.md