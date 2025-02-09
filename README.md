# adg-raeven
Raevens game for the adg

### Build with cargo
```
cargo build
```

### Building with blenvy
This application, specifically the gameplay. uses blenvy to make the stages. 
To make blenvy export tha stages and assets properly you need to open art/assets.blend in blender 4.2(other versions will not work!), and save once. The assets will then be exported to assets, which are the files that bevy will use to actually run the stage.
You can get the exact version of blender needed with
```
sudo snap install blender_4p2 --channel=4.2lts/stable --classic
```
and open the file with 
```
blender_4p2 src/gameplay/art/assets.blend
```

### Attaching debugger in vscode
require symbolic link in binary directory
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
