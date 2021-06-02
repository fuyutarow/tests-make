# tests-make

![image](https://user-images.githubusercontent.com/14998939/120461563-11dbcd00-c3d5-11eb-9e35-4f0bb2ee0d59.png)


## Usage
```
tests-make some_tests.toml
```


## Installation

```
brew install fuyutarow/tap/tests-make
```
Support
- Homebrew (mac)
- Linuxbrew (Linux, WSL)


## Features

### basic test
```toml
[tests.hello-success]
script = '''
echo hello, world!
'''
tobe = '''
hello, world!
'''
```
The `script` field contains the shell script to be executed. The `tobe` field describes the standard output to be expected.


#### env vars
```toml
[env]
INPUT = '''
hello, world!
'''

[tests.hello-tests-make]
script = '''
echo hello, ${INPUT}
'''
tobe = '''
hello, tests-make
'''
```
see [example](https://github.com/fuyutarow/tests-make/blob/alpha/examples/tests.toml)


### include tests-make.toml
```toml
includes = ["examples/others/tests.toml"]
```
see [example](https://github.com/fuyutarow/tests-make/blob/alpha/examples/includes.toml)


For a more practical example of tests-make, see this project. ([partiql-rs](https://github.com/fuyutarow/partiql-rs))
In this project, we are testing the standard output of the CLI command `pq`.
