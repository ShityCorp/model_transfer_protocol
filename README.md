# model_transfer_protocol

A cross-platform Rust peer-to-peer file-transfer protocol built over UDP, with a Python-importable interface.

This is a learning creation and not made to compete with any out of the box network library in python. 

This is made specifically for my own need and for fun. 

AI will not be use to write code or any capability. AI may be use for code review and security check as I may create easy to catch mistake.

This is made using Rust not because I know Rust. Its because I do not know it. The objective is to use lower level language then I am use to.

Part of this file was re-writen by AI mainly for lisibility as I am hungry and tiered at this time and want it to be done. 


# Requirements

## 1 : UDP transport and peer sessions
[ ] Bind to a configurable UDP port and receive datagrams.  
[ ] Use one local UDP port for all protocol message types.  
[ ] Track peers by network address and a stable protocol-level device identity; do not treat IP address and port as identity.  
[ ] Maintain session state, including authentication status, last activity, and active transfers.  
[ ] Detect inactive peers and expire temporary session state.  
[ ] Define how peers discover or are told about one another; defer internet-scale discovery and NAT traversal until a later milestone.

## 2 : Device identity, pairing, and trust
[ ] Give each node (device) its own public/private key identity.  
[ ] Keep private keys on their device; never transmit them to another peer.  
[ ] Define how a peer’s identity is verified and approved during initial pairing.  
[ ] Store trusted device identities so approved peers can reconnect without repeating manual pairing.  
[ ] Support removing or revoking a trusted device.  
[ ] Define how identity-key replacement or rotation is verified and completed.  
[ ] Support admission policies such as prompt for approval, trusted devices only, or explicitly configured automatic approval.  
[ ] Limit unauthenticated peers to the pairing or authentication process.

## 3 : Secure sessions and authorization
[ ] Use an established security protocol suitable for UDP, such as DTLS, for peer authentication and encrypted, integrity-protected sessions.  
[ ] Negotiate fresh session keys when establishing a session; do not use a permanent shared session key.  
[ ] Authenticate a peer before accepting file-list or transfer requests.  
[ ] Keep device trust separate from user accounts and file permissions.  
[ ] Let the application decide whether an authenticated user account may enroll or authorize another device, including any automatic enrollment policy.  
[ ] Authorize each requested action against the node’s sharing rules.

## 4 : Message format and validation
[ ] Define a versioned message format with a message type and the identifiers needed to associate it with a peer, session, or transfer.  
[ ] Support message types such as heartbeat, file listing, download request, transfer data, acknowledgment, cancellation, and error.  
[ ] Validate message versions, lengths, fields, and state before processing.  
[ ] Reject malformed, unsupported, unauthorized, duplicate, or stale messages safely.  
[ ] Set limits on message size, concurrent sessions, transfers, and other resource use.

## 5 : File transfer
[ ] Split files into numbered chunks and reassemble them correctly.  
[ ] Detect missing, duplicated, or out-of-order chunks.  
[ ] Define acknowledgments, retries, transfer completion, and cancellation.  
[ ] Limit retry rates and handle packet loss without overwhelming the network.  
[ ] Report transfer progress and failures clearly.  
[ ] Defer resuming interrupted transfers until a later milestone, if needed.

## 6 : Sharing and file safety
[ ] Share only files or folders explicitly selected by the user or application.  
[ ] Prevent access outside approved shared locations.  
[ ] Require policy approval or user approval before accepting an upload.  
[ ] Save received files to a configured or user-selected destination.  
[ ] Avoid silently overwriting existing files.  
[ ] Treat all incoming network data, paths, and filenames as untrusted input.

## 7 : Application-facing interface
[ ] Provide clear operations to start and stop the service, manage trusted peers, and configure sharing.  
[ ] Provide operations to list peers and shared files, request or cancel transfers, and query transfer status.  
[ ] Report results, progress, and errors consistently.  
[ ] Keep network and file operations from blocking the application’s main thread.

## 8 : Python interface
[ ] Expose the Rust library to Python using PyO3.  
[ ] Provide a documented Python interface for service control, peer management, sharing, and transfers.  
[ ] Make progress, completion, cancellation, and errors available to Python.  
[ ] Build compatible Python extensions for each supported operating system and architecture.

## 9 : Logging and diagnostics
[ ] Emit structured diagnostic events for service, peer, security, and transfer activity.  
[ ] Let the host application configure logging and verbosity.  
[ ] Do not configure global logging or write logs to a fixed location.  
[ ] Do not log private keys, session secrets, file contents, or unnecessary sensitive information.

## 10 : Development milestones
[ ] First milestone: two approved peers transfer a small file over the same local network using authenticated, encrypted UDP sessions.  
[ ] Next: multiple peers, larger files, loss/reordering handling, and interruption recovery.  
[ ] Later: account-based device enrollment, peer discovery, and internet connectivity/NAT traversal.  
[ ] Keep protocol and transfer logic platform-independent; use the operating system’s normal UDP sockets on Windows and Linux.