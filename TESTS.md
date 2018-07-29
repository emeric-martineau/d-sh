# How write tests ?

D-SH use `own` test framework. It's very easy to use, but not really evolved.

In `src/main/tests` you find two folders :
 * `resources` that contain files (binaries, program),
 * `shell` that contain test framework and tests files.

In `src/main/tests/shell/tests` you have all tests files. Name of folders and files
start by number to be execute in right order.

I know, it's very bad, but because build image is very long, I did this (I'm very sorry)

## Create my first test

Create a file in `src/main/tests/shell/tests/001-build` named `999-my-first-test.sh` :

```
#!/bin/sh

# This file test build base image
DESCRIPTION="Test build base image"
COMMAND="build"
ARGS="-b"
TEST_FUNCTION="my_name_of_test_function_but_must_be_single"

my_name_of_test_function_but_must_be_single_before() {
  echo "Hello, this line go to log file." >> ${LOG_FILE}
}

# First argument is return of d.sh
# Second argument is output of d.sh
my_name_of_test_function_but_must_be_single() {
  echo "$2"
  return $1
}
```

Test required variables:
```
DESCRIPTION   : is variable to be display the current test to be execute
COMMAND       : is D-SH command (build, run, check...)
ARGS          : is argument of D-SH command
TEST_FUNCTION : is the name of test function. This must be single in all tests files.
```

If you create a function, with same name that `TEST_FUNCTION` variable, but ended by `_before`, this function call before `TEST_FUNCTION`.

`TEST_FUNCTION` must be return `0` for success or another value for fail.

In `TEST_FUNCTION`, the first parameter is exit code of D-SH after execute `COMMAND`.
You can use it to check if command work find before check anything.

The second parameter is output of `COMMAND`.

Variables can be use in test:
```
LOG_FILE variable : is current log file of all tests. WARNING, when you write in, append your write !
IMAGE_BASE_NAME   : is to know the name of base name image.
RESOURCES_FOLDER  : is path to the tests resources folder (normaly src/main/test/resources)
SOURCES_FOLDER    : is path to the tests D-SH folder copied when tests run (normaly src/main/test/d-sh) and remove at end of tests
```

In test function, you car use `error` function to display error.
