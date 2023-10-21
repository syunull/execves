# execves

execves is a wrapper around execve that loads additional environmental variables

You can source additional envariables from

- files
- statically

## example config

```yaml
command: /usr/bin/printenv
arguments: []
environment:
  APP_NAME: printenv
environment_files:
  - /etc/app/environment
```

## example environment file

```
APP_OPTIONS="log_level=debug,compression=off"
APP_ENVIRONMENT=development
```

## why would you use this?

if you want to inject environmental variables into applications without changing their code

# Is this a good idea?

probably not
