#!/bin/bash
# Use readline wrapper to assist with tab completion/history and stuff
rlwrap -f ./test-completions.txt cargo run jsonrpc

# {"jsonrpc": "2.0", "method": "new_entity", "params": [], "id": 1}
