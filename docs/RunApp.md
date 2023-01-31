## Run an application inside CantripOS

After following [Getting Started instructions](./GettingStarted.md) you should be able to build and run CantripOS with `system` being the main application. 

To run one of the applications under the [apps directory](../apps/), change the [autostart.repl](../apps/repl/autostart.repl) file:
```sh
builtins
install hello.app
start hello
```

Run: 
```sh
m clean 
m simulate-debug
```