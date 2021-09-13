All of the tests are run from `tests.rs`, which saves a massive amount of time in browser restarts between tests and is convenient in some other ways too.

Tests consist of a block of CUI code and then some DOM API steps to verify that a page was generated like it should be. Like CUI apps themselves, test code is placed in a root element inside of the body. This is done specifically so that tests can work, because wasm_bindgen_test uses elements in the body to store some data (such as console.log output).

Each test starts with a `cui_test_setup!` block which contains some CUI code. This macro generates the static html that the page starts with and the rust code that would modify at at runtime, but has runtime code to insert the html into the body of the page before the rest of the generated code. This allows tests to render the static portion of the page without using an html file.

The macro also creates several variables for convenience which are accessible in the rest of the test after the macro is run, namely `window`, `document`, `head`, `body`, and `root`. Be sure to use `root` as the base element of the page instead of `body`.
