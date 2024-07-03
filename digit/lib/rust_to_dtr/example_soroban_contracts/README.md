# Example Soroban Contracts

We aim for 100% compilation coverage of _atleast_ the Stellar official repo of Soroban contract examples.

Ones left DOABLE:
* Deep Contract Auth
  * Multi-contract via mods

Ones left HARD:
* Account
  * Non contract impl
  * Lots broken
* Liquidity Pool
  * Non-contract impl
  * Traits
  * At its core this defines contract in a really weird way
  * Requires importing stuff
* Token
  * Lots going on
* Mint Lock
  * let-else block

Contracts Complete:
1. Alloc
2. Atomic Multiswap
3. Atomic Swap
4. Auth
5. Cross Contract
6. Custom Types
7. Deployer
8. Errors
9. Eth ABI
10. Events
11. Fuzzing
12. Hello World
13. Increment
14. Logging
15. Simple Account
16. Single Offer
17. Timelock
18. TTL
19. Upgradable Contract
20. Workspace


Deliverable 1 Progress [ 18/25 | 72% ]:
* [x] Logging
* [x] Simple Account
* [x] Single Offer
* [x] Timelock
* [x] TTL
* [x] Upgradeable Contract (2x contracts here: Upgradable New & Upgradable Old)
* [x] Workspace (3x contracts here: Contract A & Contract A Interface & Contract B)

We also will continue creating our own to push the limits of what this compiler (technically transpiler) can do.
