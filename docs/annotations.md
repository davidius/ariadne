# Annotating your logs

Once you've set up some services in [your services_config.md file](services_config.md), you might find it helpful to add annotations to your logs.

Annotations are stored in a log_annotations.json file that you store alongside your services.json file in the .ariadne directory. Unlike the services.json file, this is not necessary to run the app.

```json
{
  "annotations": LogAnnotation[],
}
```

Each `LogAnnotation` has the following structure:

```typescript
{
    "regex": string, // a string or regular expression to match in the logs
    "annotation_type": "ERROR" | "INFO",
    "hint": string, // a message that will appear when the regex is matched
    "affected_services": string[], // a list of services for which you would like to match. These should correspond with the names in the services.json file
    "links": string[] // a list of URLs for more information
}
```