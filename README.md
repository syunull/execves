# execves

execves is a wrapper around execve that loads additional environmental variables from `s`.

You can source additional envariables from

- files
- executables
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

if you want source a file or executable to inject environmental variables into applications without changing their code

# Is this a good idea?

probably not
