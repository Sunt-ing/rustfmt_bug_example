### Motivation
This repo is used to reproduce the `rustfix` bug reported in https://github.com/rust-lang/rustfmt/issues/5131 .

### How to reproduce it?
**Step1**: clone this repo:
```
git clone https://github.com/sunt-ing/rustfmt_bug_example.git
```

**Step 2**: ensure the repo is correct.
```bash
cd rustfmt_bug_example
cargo test
```
Should be fine.

**Step 3**: format it by running `rustfmt` with the rules in `rustfmt.toml`.
```bash
cd rustfmt_bug_example
cargo fmt
```

**Step 4**: run the test again:
```bash
cd rustfmt_bug_example
cargo test
```
Test failed because of lack of importing dependencies.


**Tips**: if you want to run again, you can drop all the local changes:
```bash
cd rustfmt_bug_example
git co -- .
```

### Expected behaviour
I don't think that `rustfmt` should turn the correct code into incorrect in any condition.

Both the original name of the dependency and the given alias should be kept. 

`rustfmt` should provide another option to solve the issue that both of the two names are exposed in the same scope. 
