# adg-raeven
Raevens game for the adg

### Build with cargo
```
cargo build
```

### Attaching debugger in vscode
require symbolic link in binary directory
example: 
```
cd target/debug
ln -s ../../assets assets
cd -
```
vscode launch.json example
```
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "1",
            "program": "${workspaceFolder}/target/debug/scene3d-test",
            "preLaunchTask": "build current"
        },
    ]
```

### Run with cargo
```
cargo run --bin scene3d-test
```
