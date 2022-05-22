Welcome to **Ariadne**, a command line app to help you automate your command line. Save commonly used commands so that you can run them without having to remember all the minute details (am I authenticated with my cloud service? are my environment variables correct? am I using the right Java version?).

# Features

- Create aliases for any task that can be run from the command line, and run with `ariadne run <my-task>`
- Chain commands together and create aliases for those, then run them with `ariadne cook <my-recipe>`
- Add annotations to your logs to catch common errors or anything which might be useful

# Installation

The simplest way to install the app is download the latest binary [here](https://github.com/davidius/ariadne/releases).

If you prefer to build from source code, follow these steps:

- be sure you have Rust and Cargo installed
- clone this repository
- cd into the repository folder and build the app with `cargo build --release`
- this will output the binary into the /target/release folder

Either way, be sure to add the binary to your PATH for the full benefits!

# Setup

The only additional setup you'll need is to create a directory named `.ariadne` at the root of your home directory. Within this folder, you'll need to add a file named `tasks.yaml`. That file will contain all the tasks that you'd like to automate. To get started right away, copy over all the content in the example `tasks.yaml` file contained in [docs/example_config](docs/example_config/tasks.yaml).

# Commands

To run a task:

```bash
ariadne run <task_name>
```

Note that `task_name` refers to the `name` field in the tasks.yaml file.

To run a recipe (or pipeline) of tasks:

```bash
ariadne cook <recipe_name>
```

Similarly, `recipe_name` refers to the relevant `name` field for the recipe.

# Next steps

- Add annotations to your logs to provide useful information. To learn more, see this example `log_annotations.yaml` file contained in [docs/example_config](docs/example_config/log_annotations.yaml)
- Live a life of fully-automated command line luxury