Welcome to **Ariadne**, a command line app to help you automate your command line. Save commonly used commands so that you can run them without having to remember all the minute details (am I authenticated with my cloud service? are my environment variables correct? am I using the right Java version?).

# Features

- Create aliases for any command, and run with `ariadne run <my-command>`
- Chain commands together and create aliases for those, then run them with `ariadne cook <my-recipe>`
- Add annotations to your logs to catch common errors or anything which might be useful
# Simplest usage

Suppose you have a node application that is ordinarily run with `npm run start`. If you want to automate this, you'll need to have something like the following configuration in your services.json file (more on this later):

```json
    // ... more config
    {
      "name": "my-app",
      "service_run_config": {
        "dir": "/path/to/my/app",
        "start_command": "npm run start"
      }
    },
    // ... more config
```

With that set up, running your application is as simple as entering:

```bash
ariadne run my-app
```

# Installation

The simplest way to install the app is download the latest binary [here](https://github.com/davidius/ariadne/releases).

If you prefer to build from source code, follow these steps:

- be sure you have Rust and Cargo installed
- clone this repository
- cd into the repository folder and build the app with `cargo build --release`
- this will output the binary into the /target/release folder

Either way, be sure to add the binary to your PATH for the full benefits!

# Setup

The only additional setup you'll need is to create a directory named `.ariadne` at the root of your home directory. Within this folder, you'll need to add a file named `services.json`. That file will contain all the services that you'd like to automate. To get started right away, copy over all the content in the example `services.json` file contained in [docs/example_config](docs/example_config/services.json).

# Commands

To run a service:

```bash
ariadne run <service_name>
```

Note that `service_name` refers to the `name` field in the services.json file.

To run a set of services in series:

```bash
ariadne cook <recipe_name>
```

Similarly, `recipe_name` refers to the relevant `name` field for the recipe.

# Next steps

- Set up your services.json file to do what you need it to do. For details on how to do this, refer to [this page](docs/services_config.md).
- Add annotations to your logs by [creating a log_annotations.json file](docs/annotations.md)
- Live a life of fully-automated command line luxury