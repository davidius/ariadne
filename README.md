Welcome to **Ariadne**, a command-line app to help you run your services locally without having to think about environment variables, authentication, java versions, you name it!

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

# Installation and setup

@TODO