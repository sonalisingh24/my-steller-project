#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, symbol_short};

// Tracks overall licensing statistics across the platform
#[contracttype]
#[derive(Clone)]
pub struct LicenseStats {
    pub total_nfts: u64,      // Total NFTs registered on the platform
    pub total_licenses: u64,  // Total licenses issued
    pub active_licenses: u64, // Currently active (non-expired) licenses
    pub revoked_licenses: u64,// Licenses revoked by the NFT owner
}

// Symbol key for global license stats storage
const ALL_STATS: Symbol = symbol_short!("ALL_STATS");

// Maps a unique NFT ID to its NFT metadata
#[contracttype]
pub enum NFTBook {
    NFT(u64),
}

// Represents an NFT registered on the platform
#[contracttype]
#[derive(Clone)]
pub struct NFT {
    pub nft_id: u64,         // Unique identifier for the NFT
    pub title: String,        // Title/name of the NFT asset
    pub descrip: String,      // Description of the NFT and usage terms
    pub owner: String,        // Owner's address or identifier
    pub reg_time: u64,        // Timestamp when NFT was registered
    pub is_active: bool,      // Whether the NFT is available for licensing
}

// Counter key for unique NFT IDs
const COUNT_NFT: Symbol = symbol_short!("C_NFT");

// Maps a unique License ID to its license record
#[contracttype]
pub enum LicenseBook {
    License(u64),
}

// Represents a license granted for an NFT
#[contracttype]
#[derive(Clone)]
pub struct License {
    pub license_id: u64,     // Unique identifier for the license
    pub nft_id: u64,          // The NFT being licensed
    pub licensee: String,     // Address/identifier of the license holder
    pub issue_time: u64,      // Timestamp when the license was issued
    pub expiry_time: u64,     // Timestamp when the license expires
    pub is_revoked: bool,     // Whether the owner has revoked this license
    pub royalty_fee: u64,     // Royalty fee paid (in smallest token unit)
}

// Counter key for unique License IDs
const COUNT_LIC: Symbol = symbol_short!("C_LIC");

#[contract]
pub struct NFTLicensingContract;

#[contractimpl]
impl NFTLicensingContract {

    // Registers a new NFT on the platform so it can be licensed.
    // Returns the unique NFT ID assigned to the newly registered NFT.
    pub fn register_nft(env: Env, title: String, descrip: String, owner: String) -> u64 {
        let mut count_nft: u64 = env.storage().instance().get(&COUNT_NFT).unwrap_or(0);
        count_nft += 1;

        let time = env.ledger().timestamp();

        let nft = NFT {
            nft_id: count_nft,
            title,
            descrip,
            owner,
            reg_time: time,
            is_active: true,
        };

        let mut stats = Self::view_license_stats(env.clone());
        stats.total_nfts += 1;

        env.storage().instance().set(&NFTBook::NFT(count_nft), &nft);
        env.storage().instance().set(&COUNT_NFT, &count_nft);
        env.storage().instance().set(&ALL_STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "NFT Registered with ID: {}", count_nft);
        count_nft
    }

    // Issues a license for an NFT to a licensee for a given duration and royalty fee.
    // nft_id      - The ID of the NFT to license
    // licensee    - Address/identifier of the party receiving the license
    // duration    - Duration of the license in seconds
    // royalty_fee - Fee paid by the licensee for this license
    // Returns the unique License ID.
    pub fn issue_license(
        env: Env,
        nft_id: u64,
        licensee: String,
        duration: u64,
        royalty_fee: u64,
    ) -> u64 {
        let nft = Self::view_nft(env.clone(), nft_id);

        // Ensure the NFT exists and is available for licensing
        if !nft.is_active || nft.nft_id == 0 {
            log!(&env, "NFT ID {} is not available for licensing", nft_id);
            panic!("NFT is not available for licensing");
        }

        let mut count_lic: u64 = env.storage().instance().get(&COUNT_LIC).unwrap_or(0);
        count_lic += 1;

        let time = env.ledger().timestamp();

        let license = License {
            license_id: count_lic,
            nft_id,
            licensee,
            issue_time: time,
            expiry_time: time + duration,
            is_revoked: false,
            royalty_fee,
        };

        let mut stats = Self::view_license_stats(env.clone());
        stats.total_licenses += 1;
        stats.active_licenses += 1;

        env.storage().instance().set(&LicenseBook::License(count_lic), &license);
        env.storage().instance().set(&COUNT_LIC, &count_lic);
        env.storage().instance().set(&ALL_STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "License ID: {} issued for NFT ID: {}", count_lic, nft_id);
        count_lic
    }

    // Allows an NFT owner to revoke a license before it expires.
    // Once revoked, the licensee no longer has valid usage rights.
    pub fn revoke_license(env: Env, license_id: u64) {
        let mut license = Self::view_license(env.clone(), license_id);

        if license.is_revoked || license.license_id == 0 {
            log!(&env, "License ID {} cannot be revoked", license_id);
            panic!("License does not exist or is already revoked");
        }

        license.is_revoked = true;

        let mut stats = Self::view_license_stats(env.clone());
        if stats.active_licenses > 0 {
            stats.active_licenses -= 1;
        }
        stats.revoked_licenses += 1;

        env.storage().instance().set(&LicenseBook::License(license_id), &license);
        env.storage().instance().set(&ALL_STATS, &stats);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "License ID: {} has been revoked", license_id);
    }

    // Returns the details of a registered NFT by its unique ID.
    pub fn view_nft(env: Env, nft_id: u64) -> NFT {
        env.storage().instance().get(&NFTBook::NFT(nft_id)).unwrap_or(NFT {
            nft_id: 0,
            title: String::from_str(&env, "Not_Found"),
            descrip: String::from_str(&env, "Not_Found"),
            owner: String::from_str(&env, "Not_Found"),
            reg_time: 0,
            is_active: false,
        })
    }

    // Returns the details of a license by its unique ID.
    pub fn view_license(env: Env, license_id: u64) -> License {
        env.storage().instance().get(&LicenseBook::License(license_id)).unwrap_or(License {
            license_id: 0,
            nft_id: 0,
            licensee: String::from_str(&env, "Not_Found"),
            issue_time: 0,
            expiry_time: 0,
            is_revoked: true,
            royalty_fee: 0,
        })
    }

    // Returns the global licensing statistics of the platform.
    pub fn view_license_stats(env: Env) -> LicenseStats {
        env.storage().instance().get(&ALL_STATS).unwrap_or(LicenseStats {
            total_nfts: 0,
            total_licenses: 0,
            active_licenses: 0,
            revoked_licenses: 0,
        })
    }
}