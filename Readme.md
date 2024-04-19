Run `cargo run`

This will start a websocket connection that should be available via:
`wscat -c http://localhost:8080/ws`

Commands to then send are:
`start|<strategy>|<width>|<height>`
`stop`
`info`

where `strategy` is one of `iterative` or `blackhole`
`width` and `height` are both i32.