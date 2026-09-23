Environment where AI agents collaborate and compete to complete goals

Project structure:
- /spec are the specification tests that the implementation must follow
- /state is where state structures live
- /impl_ is the actual implementation

Any mock input string and state that passes assumptions(), should pass the specification tests

Unless i tell you otherwise, follow these rules:
- either only modify spec or impl (you can modify state either way)
- exception: when modifying impl, you can modify spec/check.rs, but under the constraints listed below
- only modify mock_states and mock_input_strs in spec/check.rs; do not add any additional functions
- only modify flush_state and next_state in impl_/next_state.rs; do not add any additioanl functions
- maximum of 500 LOC per file, except for spec/check.rs which can be as large as needed

If you think something is inconsistent/wrong with the spec, say so.
Unless i say otherwise, if i say "commit", always commit all changes.