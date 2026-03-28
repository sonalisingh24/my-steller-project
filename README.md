# my-steller-project
# NFT Licensing Smart Contract

## Table of Contents
- [Project Title](#project-title)
- [Project Description](#project-description)
- [Project Vision](#project-vision)
- [Key Features](#key-features)
- [Future Scope](#future-scope)

---

## Project Title

**NFT Licensing** — A decentralized smart contract built on the Soroban SDK (Stellar blockchain) for registering NFTs and managing licensing rights with on-chain royalty tracking.

---

## Project Description

The NFT Licensing smart contract provides a transparent and trustless system for NFT creators and owners to register their digital assets on-chain and grant time-bound, royalty-based usage licenses to third parties (licensees). All licensing activity — including issuance, royalty fees, expiry, and revocation — is recorded immutably on the Stellar blockchain via the Soroban smart contract platform.

The contract exposes **4 core write functions** and **3 view functions**:

| Function | Type | Description |
|---|---|---|
| `register_nft` | Write | Registers a new NFT with title, description, and owner |
| `issue_license` | Write | Issues a time-bound license for an NFT with a royalty fee |
| `revoke_license` | Write | Allows the NFT owner to revoke an active license |
| `view_nft` | Read | Fetches details of a registered NFT by ID |
| `view_license` | Read | Fetches details of a license by its ID |
| `view_license_stats` | Read | Returns global platform-wide licensing statistics |

### Contract Data Structures

**`NFT`** — Represents a registered digital asset:
```
nft_id      → Unique ID assigned on registration
title       → Title of the NFT
descrip     → Description / usage terms
owner       → Owner's address or identifier
reg_time    → Ledger timestamp of registration
is_active   → Whether the NFT is open for licensing
```

**`License`** — Represents a usage license granted to a licensee:
```
license_id   → Unique ID assigned at issuance
nft_id       → The licensed NFT's ID
licensee     → Address/identifier of the license holder
issue_time   → Ledger timestamp of issuance
expiry_time  → issue_time + duration (in seconds)
is_revoked   → Whether the owner revoked this license
royalty_fee  → Fee paid by the licensee
```

**`LicenseStats`** — Global platform statistics:
```
total_nfts       → Total NFTs registered
total_licenses   → Total licenses ever issued
active_licenses  → Currently non-revoked licenses
revoked_licenses → Total revoked licenses
```

---

## Project Vision

The vision of the NFT Licensing project is to **democratize intellectual property management** for digital creators by removing the need for centralized intermediaries such as licensing agencies, copyright registries, and royalty collection societies.

By placing NFT licensing logic directly on-chain:

- **Creators** retain full ownership and control over their assets and can issue, monitor, and revoke licenses without relying on any third party.
- **Licensees** receive verifiable, tamper-proof proof of their usage rights, with clear expiry and royalty terms encoded directly in the contract.
- **Transparency** is enforced by the blockchain — all license activity is publicly auditable.
- **Automation** replaces manual paperwork: license expiry, royalty fee recording, and revocation are all handled trustlessly by smart contract logic.

The ultimate goal is to create a foundation for a **creator economy** where digital rights are as programmable, composable, and transferable as any other on-chain asset.

---

## Key Features

- **NFT Registration** — Any creator can register their digital asset (art, music, video, code, etc.) on-chain with a title, description, and owner identifier, receiving a unique NFT ID.

- **Time-Bound License Issuance** — Owners can grant licenses to any licensee for a configurable duration (in seconds). The expiry time is computed automatically from the ledger timestamp at issuance.

- **On-Chain Royalty Recording** — Every license records the royalty fee paid by the licensee, providing an auditable trail of all royalty transactions.

- **License Revocation** — NFT owners can revoke any active license at any time, immediately invalidating the licensee's usage rights on-chain.

- **Platform-Wide Statistics** — A global `LicenseStats` record tracks total NFTs, total licenses issued, currently active licenses, and total revocations — giving a real-time snapshot of platform activity.

- **Default-Safe Reads** — All view functions return safe default values (with `nft_id: 0` or `license_id: 0`) instead of panicking when a record is not found, making integration straightforward for front-end clients.

- **Soroban-Native Storage with TTL** — All records use Soroban instance storage with TTL extension (`extend_ttl`) to ensure data persistence across ledger epochs.

---

## Future Scope

The current contract lays a minimal, extensible foundation. Planned enhancements include:

1. **Multi-Tier Licensing** — Support for exclusive vs. non-exclusive license types, allowing an NFT owner to issue a single exclusive license or unlimited non-exclusive ones.

2. **On-Chain Royalty Payments** — Integration with Stellar's native asset / token interface (SEP-41) to enforce actual token transfers at license issuance time, not just recording the fee amount.

3. **Sub-Licensing** — Allow a primary licensee to issue sub-licenses to downstream parties, with configurable royalty splits flowing back to the original creator.

4. **License Marketplace** — A secondary marketplace where active licenses can be listed, transferred, or auctioned between parties, with on-chain settlement.

5. **Dispute Resolution**
6.  — An on-chain arbitration mechanism where a neutral third-party arbitrator can rule on licensing disputes and force revocations or fee refunds.

7. **NFT Ownership Transfer** — A function allowing an NFT's ownership to be transferred on-chain, with all associated active licenses automatically updated to reflect the new owner.

8. **Batch Operations** — Support for batch NFT registration and batch license issuance to reduce transaction overhead for creators with large asset portfolios.

9. **Front-End dApp Integration** — A web-based dashboard for creators and licensees to manage their NFTs and licenses without needing to interact with the contract directly via CLI.

---

> Built with ❤️ using [Soroban SDK](c) on the Stellar blockchain.
> ## contract details:CB54D37UEFWD3IVHQLDV34N4CH5KPDKZW4Y3MFJTHZHJM5NSCX7CZ4EZ
> <img width="1919" height="929" alt="image" src="https://github.com/user-attachments/assets/96a40d5e-8a66-40e8-bf90-df1cd4a72fcd" />

