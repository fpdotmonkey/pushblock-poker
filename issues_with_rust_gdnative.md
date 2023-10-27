# Rust `gdnative` issues

- Integration testing is non-existant
- How to use the `_input(event)` callback isn't documented (nor obvious)
  - the handle is actually
    `fn _input(&self, owner: &Owner, event: Ref<InputEvent>) -> ()`
  - The handle makes sense if you read the Godot source
  - The handle doesn't make senes if you read the Godot docs
- How to call the familiar methods on `Ref<InputEvent>` is not obvious
- `Ref<InputEvent>` should implement `std::fmt::Display`
  - It does implement debug, but that just gets you `InputEvent(0x20feb9d0)`