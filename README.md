# Tick Tock

Requirements:
  - To build, follow the steps for your OS on [the requirements-page on gtk-rs](https://gtk-rs.org/docs-src/requirements)
  - Have a `config.json` in the root of the project with the minimum requirements:
    - TENKFEET__TOKEN
    - TENKFEET__ROOT
    - USER


**config.json**
```json
{
  "tenkfeet": {
    "token": "...",
    "root": "https://api.10000ft.com"
  },
  "user": "Dennis Pettersson"
}
```

To build:
```sh
# compile project
make # or cargo build --release
# install in ${HOME}
make install
# uninstall
make uninstall
```
