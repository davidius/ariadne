# Configuring your services.json file

A valid services.json file needs to contain at least a `services` array and a `recipes` array, though both can be empty (though that wouldn't be very useful), as in this example:

```json
{
  "services": [],
  "recipes": []
}
```

Each `service` has the following structure:

```typescript
{
    name: string, // a unique identifier for the service
    service_run_config: {
        dir: string, // full path to directory from which the command will be run
        start_command: string, // command to start the service, e.g. `npm run start`
        env?: {
            // a key-value map of environment variables to set before running the start_command
        },
        pre_commands?: string[], // array of strings, each string representing a command. These commands, if provided, will be run before the start_command
    }
}
```

Each `recipe` is a combination of `service`s that you define. It has this structure:

```typescript
{
    name: string, // a unique identifier for the recipe
    services: ServiceRunConfig[], // array of config objects determining how each service will be run
}
```

> :warning: Note that the order of the `services` array is the order in which the services will run. Currently they can only run in sequence, but I intend to add parallelization in a future version.

Each `service` contained within `recipes` has the following structure:

```typescript
{
    name: string, // refers to the `name` of the service as defined in the main `services` array
    runtype: "foreground" | "background", // running a service in the foreground means watching its logs; running it in the background does not do this, and just waits for it to complete
    continue_on_log_regex?: string, // a string or regular expression to look for in the logs. Once ariadne has identified the log, it will move to the next service in the recipe. If not provided, ariadne will move to the next service only when the current one exits 
}
```