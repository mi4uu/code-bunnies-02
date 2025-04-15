# Implementation Plan for Task 01: Optimizing and Extending `commit.rs`

## Overview

This plan outlines the steps to optimize the `commit.rs` binary, make the `mistralrs` crate optional, and implement OpenAI API compatibility for generating commit messages.

## Tasks

[x] **Task 1: Create a proper implementation for the non-mistral case**
- Implement `get_commit_msg` function for the non-mistral case using OpenAI API
- Use existing `OpenAiProvider` implementation as reference
- Allow configuration via environment variables:
  - `COMMIT_BASEURL`: Base URL for the API (default to OpenAI's endpoint)
  - `COMMIT_MODEL`: Model to use (default to a reasonable model like "gpt-3.5-turbo")
  - `COMMIT_API_KEY`: API key for the service

[x] **Task 2: Refactor `commit.rs` to optimize code**
- Remove unnecessary cloning (`parts.clone()`)
- Use more efficient data structures and operations
- Extract common functionality into functions
- Improve error handling

[x] **Task 3: Add unit tests for OpenAI implementation**
- Add tests for the OpenAI API implementation
- Mock API responses for testing
- Test with different environment variable configurations

[x] **Task 4: Add unit tests for various edge cases**
- Test with empty diffs
- Test with large diffs that need truncation
- Test with multiple diff parts

[x] **Task 5: Ensure feature flags are properly set up**
- Confirm that `mistralrs` is properly configured as an optional dependency
- Verify that code correctly handles both cases (with and without the `mistral` feature)

[x] **Task 6: Documentation and code comments**
- Add comprehensive documentation for the OpenAI implementation
- Update existing documentation to reflect changes
- Ensure code comments explain complex parts

[x] **Task 7: Final testing and cleanup**
- Test end-to-end functionality with both implementations
- Clean up any debug code or unused imports
- Ensure all requirements are met

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| COMMIT_BASEURL | Base URL for the OpenAI API compatible service | https://api.openai.com/v1 |
| COMMIT_MODEL | Model to use for generating commit messages | gpt-3.5-turbo |
| COMMIT_API_KEY | API key for the service | (Required) |
