# r7-svcregd

Service Registry Daemon

FreeBSD-first, local-only json-rpc server that allows clients to discover each
other and setup direct IPC channel.

This is not a bus. Bus pattern is bad. r7-svcregd never sits in the middle of
communication


## Technology
unix sockets, pipes, fd passing, json-rpc, socketpair(2), pipe(2)


## Protocol

