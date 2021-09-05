# What is this?

This is a small project I made in a few days to figure out rust

It's not the best, or cleanest code but it works

## What does it do?

The `client` is made for a `raspberry pi` attached to a `HC-SR04`.
The sensor queries the range every defined period of time and sends it to the `server` via a websocket

The `server` accepts the data and, if needed, can simulate a key press sequence

# Should you use this for anything?

No, not really

# What each file does

```
piRange
├── client
│  ├── .cargo
│  │  └── config    # Linking stage for cross compilation so I don't need to build on pi
│  ├── Cargo.lock    # Dependencies managed by cargo, ignore
│  ├── Cargo.toml    # Where dependencies are defined
│  ├── deploy.sh    # Short script to cross-compile and push binary to raspberry pi
│  ├── rustfmt.toml    # Code formatting info
│  └── src
│     ├── args.rs    # Defines and gets arguments
│     ├── main.rs    # Where the main program is
│     ├── sensor.rs    # Manages the physical sensor and gets distance
│     ├── shutdown.rs    # Manages safe shutdowns
│     └── trigger.rs    # Manages running of the program
├── LICENSE    # MIT license
├── README.md    # This is where you are
└── server
   ├── Cargo.lock    # Dependencies managed by cargo, ignore
   ├── Cargo.toml    # Where dependencies are defined
   ├── rustfmt.toml    # Formatting info
   └── src
      ├── args.rs    # Defines and gets arguments
      └── main.rs    # Handles websocket connections and actions
```
