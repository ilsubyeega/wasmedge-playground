# rs-test-perms-function
This function is meant for testing permissions such like networking(in/out), ~~file access (not available)~~, terminal access etc.

## Checking
- TCP Server : `Get-Process -Id (Get-NetTCPConnection -LocalPort 1972).OwningProcess`