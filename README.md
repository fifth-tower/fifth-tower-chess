## Creating your repo from the template

[`demo`](http://chess.5-tower.online/) is optional but highly recommended. You can install them with

## Developing

```sh
trunk serve --port 3000 --open
```

will open your app in your default browser at `http://localhost:3000`.

## Deploying

To build

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.
