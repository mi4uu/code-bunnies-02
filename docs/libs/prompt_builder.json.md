# Crate Documentation

**Version:** 0.1.0

**Format Version:** 43

# Module `prompt_builder`

## Modules

## Module `agent`

```rust
pub mod agent { /* ... */ }
```

### Types

#### Struct `Agent`

```rust
pub struct Agent {
    pub name: Option<String>,
    pub description: Option<String>,
    pub agent_do: Vec<String>,
    pub agent_dont: Vec<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `Option<String>` |  |
| `description` | `Option<String>` |  |
| `agent_do` | `Vec<String>` |  |
| `agent_dont` | `Vec<String>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_name</* synthetic */ impl Into<String>: Into<String>>(self: Self, name: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_description</* synthetic */ impl Into<String>: Into<String>>(self: Self, description: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_agent_do</* synthetic */ impl Into<String>: Into<String>>(self: Self, do_action: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_agent_dont</* synthetic */ impl Into<String>: Into<String>>(self: Self, dont_action: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_agent_dos</* synthetic */ impl Into<String>: Into<String>>(self: Self, do_actions: Vec<impl Into<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_agent_donts</* synthetic */ impl Into<String>: Into<String>>(self: Self, dont_actions: Vec<impl Into<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn merge(self: &Self, other: &Agent) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Agent { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
## Module `examples`

```rust
pub mod examples { /* ... */ }
```

### Modules

## Module `basic_usage`

```rust
pub mod basic_usage { /* ... */ }
```

## Module `template_usage`

```rust
pub mod template_usage { /* ... */ }
```

## Module `formatters`

```rust
pub mod formatters { /* ... */ }
```

### Types

#### Enum `Format`

```rust
pub enum Format {
    Markdown,
    Xml,
    SemiXml,
}
```

##### Variants

###### `Markdown`

###### `Xml`

###### `SemiXml`

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Format { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `CustomFormatter`

```rust
pub struct CustomFormatter {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_template</* synthetic */ impl Into<String>: Into<String>>(self: Self, template: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_variable</* synthetic */ impl Into<String>: Into<String>, /* synthetic */ impl Into<String>: Into<String>>(self: Self, key: impl Into<String>, value: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn format(self: &Self, prompt: &Prompt) -> String { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CustomFormatter { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `Formattable`

```rust
pub trait Formattable {
    /* Associated items */
}
```

##### Implementations

This trait is implemented for the following types:

- `Prompt`

## Module `project`

```rust
pub mod project { /* ... */ }
```

### Types

#### Struct `Project`

```rust
pub struct Project {
    pub name: Option<String>,
    pub description: Option<String>,
    pub languages: Vec<String>,
    pub use_libs: Vec<String>,
    pub dont_use: Vec<String>,
    pub requirements: Vec<String>,
    pub rules: Vec<String>,
    pub good_examples: Vec<String>,
    pub bad_examples: Vec<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `Option<String>` |  |
| `description` | `Option<String>` |  |
| `languages` | `Vec<String>` |  |
| `use_libs` | `Vec<String>` |  |
| `dont_use` | `Vec<String>` |  |
| `requirements` | `Vec<String>` |  |
| `rules` | `Vec<String>` |  |
| `good_examples` | `Vec<String>` |  |
| `bad_examples` | `Vec<String>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_name</* synthetic */ impl Into<String>: Into<String>>(self: Self, name: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_description</* synthetic */ impl Into<String>: Into<String>>(self: Self, description: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_language</* synthetic */ impl Into<String>: Into<String>>(self: Self, language: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_languages</* synthetic */ impl Into<String>: Into<String>>(self: Self, languages: Vec<impl Into<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_use</* synthetic */ impl Into<String>: Into<String>>(self: Self, lib: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_uses</* synthetic */ impl Into<String>: Into<String>>(self: Self, libs: Vec<impl Into<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_dont_use</* synthetic */ impl Into<String>: Into<String>>(self: Self, lib: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_requirement</* synthetic */ impl Into<String>: Into<String>>(self: Self, requirement: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_rule</* synthetic */ impl Into<String>: Into<String>>(self: Self, rule: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_good_example</* synthetic */ impl Into<String>: Into<String>>(self: Self, example: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_bad_example</* synthetic */ impl Into<String>: Into<String>>(self: Self, example: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn merge(self: &Self, other: &Project) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Project { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
## Module `prompt`

```rust
pub mod prompt { /* ... */ }
```

### Types

#### Enum `StepType`

```rust
pub enum StepType {
    INSTRUCTION,
    RUN,
    ASK,
    THINK,
    ACT,
    TEST,
    CUSTOM(String),
}
```

##### Variants

###### `INSTRUCTION`

###### `RUN`

###### `ASK`

###### `THINK`

###### `ACT`

###### `TEST`

###### `CUSTOM`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Methods

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(s: &str) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StepType) -> bool { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StepType { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `Step`

```rust
pub struct Step {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(step_type: StepType, text: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn builder() -> StepBuilder { /* ... */ }
  ```

- ```rust
  pub fn with_continue_on_error(self: Self, continue_on_error: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_action_on_error</* synthetic */ impl Into<String>: Into<String>>(self: Self, action: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_action_on_success</* synthetic */ impl Into<String>: Into<String>>(self: Self, action: impl Into<String>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Step { /* ... */ }
    ```

#### Struct `StepBuilder`

```rust
pub struct StepBuilder {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_type(self: Self, step_type: StepType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_text</* synthetic */ impl Into<String>: Into<String>>(self: Self, text: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_continue_on_error(self: Self, continue_on_error: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_action_on_error</* synthetic */ impl Into<String>: Into<String>>(self: Self, action: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_action_on_success</* synthetic */ impl Into<String>: Into<String>>(self: Self, action: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: Self) -> Step { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> StepBuilder { /* ... */ }
    ```

- **Freeze**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Prompt`

```rust
pub struct Prompt {
    pub agent: Option<crate::agent::Agent>,
    pub project: Option<crate::project::Project>,
    pub goal: Option<String>,
    pub steps: Vec<Step>,
    pub rules: Vec<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `agent` | `Option<crate::agent::Agent>` |  |
| `project` | `Option<crate::project::Project>` |  |
| `goal` | `Option<String>` |  |
| `steps` | `Vec<Step>` |  |
| `rules` | `Vec<String>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_agent(self: Self, agent: &Agent) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_project(self: Self, project: &Project) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_goal</* synthetic */ impl Into<String>: Into<String>>(self: Self, goal: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_steps(self: Self, steps: Vec<Step>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn add_step</* synthetic */ impl Into<StepType>: Into<StepType>, /* synthetic */ impl Into<String>: Into<String>>(self: Self, step_type: impl Into<StepType>, text: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn add_step_obj(self: Self, step: Step) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_rule</* synthetic */ impl Into<String>: Into<String>>(self: Self, rule: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_rules</* synthetic */ impl Into<String>: Into<String>>(self: Self, rules: Vec<impl Into<String>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn merge(self: &Self, other: &Prompt) -> Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn to_format(self: &Self, format: Format) -> String { /* ... */ }
  ```

- ```rust
  pub fn to_custom_format(self: &Self, formatter: &CustomFormatter) -> String { /* ... */ }
  ```

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Formattable**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dest: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Prompt { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Re-exports

### Re-export `Agent`

```rust
pub use agent::Agent;
```

### Re-export `CustomFormatter`

```rust
pub use formatters::CustomFormatter;
```

### Re-export `Format`

```rust
pub use formatters::Format;
```

### Re-export `Project`

```rust
pub use project::Project;
```

### Re-export `Prompt`

```rust
pub use prompt::Prompt;
```

### Re-export `Step`

```rust
pub use prompt::Step;
```

### Re-export `StepType`

```rust
pub use prompt::StepType;
```

