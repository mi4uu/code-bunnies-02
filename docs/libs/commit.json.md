# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `commit`

## Modules

## Module `prompt`

```rust
pub(crate) mod prompt { /* ... */ }
```

### Types

#### Struct `PromptParams`

```rust
pub(in ::prompt) struct PromptParams {
    pub(in ::prompt) system_message: String,
    pub(in ::prompt) instruction: String,
    pub(in ::prompt) role: String,
    pub(in ::prompt) example_input: Option<String>,
    pub(in ::prompt) bad_example_output: Option<String>,
    pub(in ::prompt) example_output: Option<String>,
    pub(in ::prompt) input: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `system_message` | `String` |  |
| `instruction` | `String` |  |
| `role` | `String` |  |
| `example_input` | `Option<String>` |  |
| `bad_example_output` | `Option<String>` |  |
| `example_output` | `Option<String>` |  |
| `input` | `String` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Deserialize**
  - ```rust
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, <__D as >::Error>
where
    __D: _serde::Deserializer<''de> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Instrument**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DeserializeOwned**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **WithSubscriber**
- **Serialize**
  - ```rust
    fn serialize<__S>(self: &Self, __serializer: __S) -> _serde::__private::Result<<__S as >::Ok, <__S as >::Error>
where
    __S: _serde::Serializer { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `Format`

```rust
pub enum Format {
    Inst,
    Markdown,
    Tags,
    Tiny,
}
```

##### Variants

###### `Inst`

###### `Markdown`

###### `Tags`

###### `Tiny`

##### Implementations

###### Methods

- ```rust
  pub fn to_string(self: &Self) -> String { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **Sync**
- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Instrument**
- **WithSubscriber**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `create_prompt`

```rust
pub(in ::prompt) fn create_prompt(params: &PromptParams, template_name: &str) -> Result<String, minijinja::Error> { /* ... */ }
```

#### Function `get_prompt`

```rust
pub fn get_prompt(msg: String, format: Format) -> Result<String, minijinja::Error> { /* ... */ }
```

## Module `split_diff`

Module for splitting Git diff messages.

```rust
pub(crate) mod split_diff { /* ... */ }
```

### Functions

#### Function `split_git_diff`

Splits a Git diff message string into a vector of strings, where each element
represents the diff for a single file.

It identifies file boundaries by looking for lines starting with "diff --git ".
Each element in the output vector will contain the complete diff section for one file,
including the initial "diff --git " line.

Any content in the input string *before* the first "diff --git " line is ignored,
as it's typically not part of a specific file's diff hunk.

# Arguments

* `msg`: The input string containing the Git diff output.

# Returns

A `Vec<String>` where each string is a chunk corresponding to a single file's diff.
- Returns an empty vector if the input string is empty or contains no "diff --git " lines.

# Example

```rust
let diff_output = r#"diff --git a/file1.txt b/file1.txt
index 0000000..1111111 100644
--- a/file1.txt
+++ b/file1.txt
@@ -1,1 +1,1 @@
-Hello
+Hello World
diff --git a/src/main.rs b/src/main.rs
index 2222222..3333333 100644
--- a/src/main.rs
+++ b/src/main.rs
@@ -5,3 +5,4 @@
 fn main() {
     println!("Example");
 }
+// New comment
"#;
let file_diffs = split_git_diff(&diff_output);
assert_eq!(file_diffs.len(), 2);
assert!(file_diffs[0].starts_with("diff --git a/file1.txt"));
assert!(file_diffs[0].contains("+Hello World"));
assert!(file_diffs[1].starts_with("diff --git a/src/main.rs"));
assert!(file_diffs[1].contains("+// New comment"));
```

```rust
pub fn split_git_diff(msg: &str) -> Vec<String> { /* ... */ }
```

## Module `llm_response`

```rust
pub(crate) mod llm_response { /* ... */ }
```

### Functions

#### Function `get_commit_msg`

**Attributes:**

- `#[<cfg>(not(feature = "mistral"))]`

```rust
pub async fn get_commit_msg(diffs: Vec<String>) -> anyhow::Result<String> { /* ... */ }
```

#### Function `get_prompt`

```rust
pub(in ::llm_response) fn get_prompt(diff: String, prev: String) -> String { /* ... */ }
```

#### Function `extract_commit_message`

```rust
pub fn extract_commit_message(content: String) -> Option<String> { /* ... */ }
```

### Constants and Statics

#### Constant `MODEL_ID`

```rust
pub(in ::llm_response) const MODEL_ID: &str = "mradermacher/calme-3.3-llamaloi-3b-GGUF";
```

#### Constant `MODEL_FILE`

```rust
pub(in ::llm_response) const MODEL_FILE: &str = "calme-3.3-llamaloi-3b.Q4_K_M.gguf";
```

#### Constant `MODEL_TOKENIZER`

```rust
pub(in ::llm_response) const MODEL_TOKENIZER: &str = "MaziyarPanahi/calme-3.3-llamaloi-3b";
```

## Functions

### Function `main`

```rust
pub(crate) fn main() -> anyhow::Result<()> { /* ... */ }
```

### Function `commit`

```rust
pub(crate) fn commit(msg: String) -> anyhow::Result<()> { /* ... */ }
```

### Function `get_diff`

```rust
pub(crate) fn get_diff() -> String { /* ... */ }
```

## Constants and Statics

### Constant `MAX_LEN_PER_DIFF`

```rust
pub(crate) const MAX_LEN_PER_DIFF: usize = 2000;
```

