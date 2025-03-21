//! This module contains the implementation of the StardustWallet trait and the WalletImpl struct.
//! The StardustWallet trait defines the methods for creating, migrating, backing up, restoring, and deleting a wallet.
//! The WalletImpl struct represents an instantiated wallet and holds the necessary state and configuration.
//!

use super::share::Share;
use super::wallet_user::{WalletImplStardust, WalletUser};
use super::wallet_user_eth::{WalletImplEth, WalletImplEthErc20};
use crate::core::{Config, UserRepoT};
use crate::types::currencies::Currency;
use crate::types::newtypes::{AccessToken, EncryptionPin, EncryptionSalt, PlainPassword};
use crate::wallet::error::{ErrorKind, Result, WalletError};
use api_types::api::networks::{ApiNetwork, ApiProtocol};
use async_trait::async_trait;
use iota_sdk::crypto::keys::bip39::Mnemonic;
use log::{info, warn};
use secrecy::{ExposeSecret, SecretBox};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// Represents borrowing a [`WalletUser`] instance with a lifetime connected to the wallet manager.
/// This prevents wallets to be stored and used later by another user.
pub struct WalletBorrow<'a> {
    inner: Box<dyn WalletUser + Send + Sync>,
    /// with this we "attach" a lifetime to this object even though it is not "needed"
    _lifetime: std::marker::PhantomData<&'a ()>,
}

#[cfg(test)]
impl WalletBorrow<'_> {
    /// test function to create [`WalletBorrow`] instances in mock objects
    pub fn from(inner: impl WalletUser + Send + Sync + 'static) -> Self {
        Self {
            inner: Box::new(inner),
            _lifetime: PhantomData,
        }
    }
}

impl Deref for WalletBorrow<'_> {
    type Target = Box<dyn WalletUser + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DerefMut for WalletBorrow<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Creates a wallet and returns an instance to work upon
#[cfg_attr(test, mockall::automock)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait WalletManager: std::fmt::Debug {
    /// Get the recovery share
    fn get_recovery_share(&self) -> Option<Share>;

    /// Set the recovery share
    fn set_recovery_share(&mut self, share: Option<Share>);

    /// Generate a new mnemonic and create shares. Returns the new mnemonic.
    async fn create_wallet_from_new_mnemonic(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
    ) -> Result<String>;

    /// Create shares from a mnemonic
    async fn create_wallet_from_existing_mnemonic(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        mnemonic: &str,
    ) -> Result<()>;

    /// Create shares from a kdbx backup byte stream
    async fn create_wallet_from_backup(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        backup: &[u8],
        backup_password: &PlainPassword,
    ) -> Result<()>;

    /// Create kdbx backup bytes from shares
    async fn create_wallet_backup(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        backup_password: &PlainPassword,
    ) -> Result<Vec<u8>>;

    /// deletes the user's wallet
    async fn delete_wallet(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
    ) -> Result<()>;

    /// Checks if the mnemonic resembled by the shares is the same as the provided mnemonic.
    async fn check_mnemonic(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        mnemonic: &str,
    ) -> Result<bool>;

    /// Changes the password of the existing wallet by re-encrypting the backup share
    async fn change_wallet_password(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        new_password: &PlainPassword,
    ) -> Result<()>;

    /// Tries to instantiate a [`WalletUser`] object from shares and/or returns a mutable reference bound to
    /// the lifetime of this object. The same instance may be reused across several calls to
    /// `try_get`, hence the lifetime is bound to the lifetime of `self`.
    async fn try_get<'a>(
        &'a mut self,
        config: &mut Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        network: ApiNetwork,
        pin: &EncryptionPin,
    ) -> Result<WalletBorrow<'a>>;
}

/// Implementation of [`WalletManager`] that uses the SSS schema to store and retrieve the mnemonic
/// and create implementations of [`WalletUser`].
#[derive(Debug)]
pub struct WalletManagerImpl {
    /// the name of the user we are creating wallets for
    username: String,

    /// The recovery share that the user should download
    pub recovery_share: Option<Share>,
}

#[derive(Debug, PartialEq)]
struct Status {
    /// if the local share was used.
    local: bool,
    /// how the recovery share was used, or [`None`] if it was not used.
    recovery: Option<RecoveryUsed>,
    /// if the remote backup share was used.
    backup: bool,
}

/// Which recovery share that was used.
#[derive(Debug, PartialEq)]
enum RecoveryUsed {
    /// Recovery share stored locally was used
    Local,
    /// Recovery share stored remotely in the backend was used
    Remote,
}

impl WalletManagerImpl {
    /// Create a new [`WalletManagerImpl`] from a username.
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            recovery_share: None,
        }
    }

    // fn for getting the mnemonic
    async fn try_resemble_shares(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
    ) -> Result<(Mnemonic, Status)> {
        info!("Initializing wallet for user from shares");

        let username = &self.username;
        let local_recovery_share = self.recovery_share.clone();

        let user = repo.get(username)?;

        // make sure the provided pin is valid (even though we currenctly do not need it unless
        // decrypting some of the shares, but in the future we might want to encrypt the local
        // share too, for example)
        // let _ = user
        //     .encrypted_password
        //     .as_ref()
        //     .ok_or(crate::Error::WalletNotInitialized(crate::WalletNotInitializedKind::MissingPassword))?
        //     .decrypt(pin, &user.salt)?;

        // check the availability of each share (in priority order of ease-of-use and availability)

        let mut available_shares: Vec<Share> = Vec::new();
        let mut recovery_share_available_with_user_action = false;
        let mut password_required = false;

        // in case of success we need to keep track of the share states
        let mut local_used = false;
        let mut recovery_used = None;
        let mut backup_used = false;

        if let Some(share) = user.local_share.map(|s| s.parse::<Share>()) {
            available_shares.push(share?);
            log::info!("Local storage share available");
            local_used = true;
        }

        if let Some(share) = local_recovery_share {
            available_shares.push(share);
            log::info!("Local recovery share available");
            recovery_used = Some(RecoveryUsed::Local);
        } else {
            log::info!("Local recovery share not available, checking if it can be downloaded");
            recovery_share_available_with_user_action = true;

            // try getting the oauth share (not encrypted)
            if let Some(access_token) = &access_token {
                match crate::backend::shares::download_recovery_share(config, access_token, username).await {
                    Ok(Some(share)) => {
                        available_shares.push(share);
                        recovery_share_available_with_user_action = false; // this share is now available
                        recovery_used = Some(RecoveryUsed::Remote);
                        log::info!("Recovery share downloaded and available");
                    }
                    Ok(None) => log::info!("Recovery share not available"),
                    Err(e) => log::warn!("Error fetching recovery share: {e}"),
                }
            } else {
                log::info!("Access token not available, skipping recovery share download.");
            }
        }

        // if we have less than two, we should try to get the backup share
        // this is the last resort since it requires the password
        if available_shares.len() < 2 {
            if let Some(access_token) = &access_token {
                // try to get it from the backend
                match crate::backend::shares::download_backup_share(config, access_token, username).await {
                    Ok(Some(share)) => {
                        available_shares.push(share);
                        password_required = true;
                        backup_used = true;
                        log::info!("Backup share (encrypted) downloaded and available");
                    }
                    Ok(None) => log::info!("Backup share (encrypted) not available"),
                    Err(e) => log::warn!("Error fetching backup share: {e}"),
                }
            } else {
                log::info!("Access token not available, skipping backup share download.");
            }
        }

        // done, no need to leave the variables mutable anymore
        let available_shares = available_shares;
        let recovery_share_available_with_upload = recovery_share_available_with_user_action;
        let password_required = password_required;

        log::info!(
            "Done collecting shares. Got {} shares, recovery_share_available_with_user_action = {}, password_required = {}",
            available_shares.len(),
            recovery_share_available_with_upload,
            password_required
        );

        if available_shares.len() >= 2 {
            // enough shares are available!

            // if the password is required, we need to try to get it or return an error
            let password = if password_required {
                let password = user
                    .encrypted_password
                    .ok_or(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))?
                    .decrypt(pin, &user.salt)?;

                Some(password)
            } else {
                None
            };

            let shares_ref = available_shares.iter().collect::<Vec<&Share>>();

            // now we can finally try to recreate the mnemonic from the shares
            let mnemonic = crate::share::reconstruct_mnemonic(
                &shares_ref,
                password.as_ref().map(PlainPassword::into_secret).as_ref(),
            )?;

            if !local_used {
                log::info!("Local share not set, recreating shares and storing local share again");

                // create the shares again, and just use a random password since we are not interested
                // in the backup share anyways (which is the only reason this needs a password)
                let shares = crate::share::create_shares_from_mnemonic(
                    secrecy::ExposeSecret::expose_secret(&mnemonic).clone(),
                    &SecretBox::new(String::from("dummy password").as_bytes().into()),
                )?;

                // ignore the error since we were still able to create a valid wallet
                if let Err(e) = repo.set_local_share(username, Some(&shares.local)) {
                    log::warn!("Error storing local share again: {e:#}");
                } else {
                    log::info!("Done storing local share again");
                }
            }

            Ok((
                mnemonic.expose_secret().clone(),
                Status {
                    local: local_used,
                    recovery: recovery_used,
                    backup: backup_used,
                },
            ))
        } else if available_shares.len() == 1 && recovery_share_available_with_upload {
            Err(WalletError::WalletNotInitialized(ErrorKind::SetRecoveryShare))
        } else {
            // there is no way to recover the shares
            Err(WalletError::WalletNotInitialized(ErrorKind::UseMnemonic))
        }
    }

    /// Creates shares from the provided mnemonic and stores the local share locally, uploads the other
    /// shares to the backend and returns the recovery share for the user to download and save.
    async fn create_and_upload_shares(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        mnemonic: impl Into<Mnemonic>,
    ) -> Result<()> {
        log::info!("Creating and uploading shares");

        // get the password from the repo
        let user = repo.get(&self.username)?;
        let Some(encrypted_password) = user.encrypted_password else {
            return Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword));
        };

        let password = encrypted_password.decrypt(pin, &user.salt)?;
        let shares = crate::share::create_shares_from_mnemonic(mnemonic, &password.into_secret())?;

        log::info!("Shares created, storing local share");
        repo.set_local_share(&user.username, Some(&shares.local))?;
        self.recovery_share = Some(shares.recovery.clone());

        if let Some(access_token) = access_token {
            log::info!("Uploading shares");
            crate::backend::shares::upload_backup_share(config, access_token, &shares.backup, &user.username).await?;
            crate::backend::shares::upload_recovery_share(config, access_token, &shares.recovery, &user.username)
                .await?;
            log::info!("Done uploading shares");
        } else {
            log::info!("No access token, skipping uploading backup and recovery shares");
        }
        Ok(())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletManager for WalletManagerImpl {
    fn get_recovery_share(&self) -> Option<Share> {
        self.recovery_share.clone()
    }
    fn set_recovery_share(&mut self, share: Option<Share>) {
        self.recovery_share = share;
    }
    /// Generate a new mnemonic and create shares. Returns the new mnemonic.
    async fn create_wallet_from_new_mnemonic(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
    ) -> Result<String> {
        let mnemonic = iota_sdk::client::Client::generate_mnemonic()?;
        self.create_and_upload_shares(config, access_token, repo, pin, mnemonic.as_ref())
            .await?;

        Ok(mnemonic.to_string())
    }

    /// Create shares from a mnemonic
    async fn create_wallet_from_existing_mnemonic(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        mnemonic: &str,
    ) -> Result<()> {
        self.create_and_upload_shares(config, access_token, repo, pin, mnemonic)
            .await
    }

    /// Create shares from a kdbx backup byte stream
    async fn create_wallet_from_backup(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        backup: &[u8],
        backup_password: &PlainPassword,
    ) -> Result<()> {
        use secrecy::ExposeSecret;
        let mnemonic = crate::kdbx::load_mnemonic(backup, &backup_password.into_secret_string())?;
        self.create_and_upload_shares(config, access_token, repo, pin, mnemonic.expose_secret().clone())
            .await
    }

    /// Create kdbx backup bytes from shares
    async fn create_wallet_backup(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        backup_password: &PlainPassword,
    ) -> Result<Vec<u8>> {
        let (mnemonic, _status) = self.try_resemble_shares(config, access_token, repo, pin).await?;

        Ok(crate::kdbx::store_mnemonic(
            &SecretBox::new(Box::new(mnemonic)),
            &backup_password.into_secret_string(),
        )?)
    }

    async fn delete_wallet(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
    ) -> Result<()> {
        // remove the wallet folder
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = config.path_prefix.join("wallets").join(&self.username);
            if let Err(e) = std::fs::remove_dir_all(path) {
                warn!("Error removing wallet files: {e:?}");
            }
        }

        // clear the local and recovery share
        repo.set_local_share(&self.username, None)?;
        self.recovery_share = None;

        // call backend if access_token exists
        if let Some(access_token) = access_token {
            crate::backend::shares::delete_shares(config, access_token, &self.username).await?;
        }

        Ok(())
    }

    async fn check_mnemonic(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        mnemonic: &str,
    ) -> Result<bool> {
        // first use the existing pin and stored (encrypted) password to resemble the shares into
        // the mnemonic
        let (existing_mnemonic, _status) = self.try_resemble_shares(config, access_token, repo, pin).await?;

        // perform a str-str comparison
        Ok(*mnemonic == **existing_mnemonic)
    }

    async fn change_wallet_password(
        &mut self,
        config: &Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        pin: &EncryptionPin,
        new_password: &PlainPassword,
    ) -> Result<()> {
        // first use the existing pin and stored (encrypted) password to try resemble the shares into the mnemonic
        let result = self.try_resemble_shares(config, access_token, repo, pin).await;

        // if there was a real error (not only a missing wallet), propagate it,
        // otherwise we want to goahead and update the repo
        match result {
            Ok(_) => {}
            Err(WalletError::WalletNotInitialized(ErrorKind::UseMnemonic)) => {
                warn!("No wallet found (UseMnemonic error), continuing to change the password locally only")
            }
            Err(e) => return Err(e),
        }

        // now update the password in the repo (perhaps a bit hacky... xD)
        let mut user = repo.get(&self.username)?;
        let salt = EncryptionSalt::generate();
        let encrypted_password = new_password.encrypt(pin, &salt)?;
        user.salt = salt;
        user.encrypted_password = Some(encrypted_password);
        repo.update(&user)?;

        // and if we need to reconstruct the shares, do it!
        if let Ok((mnemonic, _status)) = result {
            self.create_and_upload_shares(config, access_token, repo, pin, mnemonic.clone())
                .await?;
        }

        Ok(())
    }

    async fn try_get<'a>(
        &'a mut self,
        config: &mut Config,
        access_token: &Option<AccessToken>,
        repo: &mut UserRepoT,
        network: ApiNetwork,
        pin: &EncryptionPin,
    ) -> Result<WalletBorrow<'a>> {
        let (mnemonic, _status) = self.try_resemble_shares(config, access_token, repo, pin).await?;

        // we have the mnemonic and can now instantiate the WalletImpl

        let path = config
            .path_prefix
            .join("wallets")
            .join(&self.username)
            .join(network.clone().key);

        let bo = match network.protocol {
            ApiProtocol::Evm { chain_id } => {
                let wallet = WalletImplEth::new(mnemonic, network.node_urls, chain_id)?;
                Box::new(wallet) as Box<dyn WalletUser + Sync + Send>
            }
            ApiProtocol::EvmERC20 {
                chain_id,
                contract_address,
            } => {
                let wallet = WalletImplEthErc20::new(mnemonic, network.node_urls, chain_id, contract_address)?;
                Box::new(wallet) as Box<dyn WalletUser + Sync + Send>
            }
            ApiProtocol::Stardust {} => {
                let currency = Currency::try_from(network.display_symbol)?;
                let wallet = WalletImplStardust::new(mnemonic, &path, currency, network.node_urls).await?;
                Box::new(wallet) as Box<dyn WalletUser + Sync + Send>
            }
        };

        Ok(WalletBorrow {
            inner: bo,
            _lifetime: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{Config, UserRepoT},
        kdbx::KdbxStorageError,
        testing_utils::{example_api_network, BACKUP_PASSWORD},
        types::{
            newtypes::{AccessToken, EncryptionPin, EncryptionSalt, PlainPassword},
            users::KycType,
        },
        user::{memory_storage::MemoryUserStorage, repository::UserRepoImpl, MockUserRepo},
    };
    use kdbx_rs::errors::UnlockError;
    use rstest::rstest;
    use std::sync::LazyLock;

    const MNEMONIC:&str = "endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river";
    const MNEMONIC_INCORRECT:&str = "answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river";
    const USERNAME: &str = "SuperAdmin";
    static PASSWORD: LazyLock<PlainPassword> =
        LazyLock::new(|| PlainPassword::try_from_string("StrongP@55w0rd").unwrap());

    // share strings to use for testing the resemble-function
    const SHARE_LOCAL: &str = "ME-RS-N-Mi0xLUNBRVFBaGdESXFBRStPVUZYZTJnMTdLRFY1L2pWRllQTHdtZ0dCWExJbitjTERReFRyRHArWGNVMG5yY3UyVmFONFEvZkVoeXNadm5qNFhmRDVIZXZ3eHB2bENTYnZIZTFtOTlXdjJwby8zVWl0d2VhMnVWOTZaejB5WmhEdHlkRDFYcEg1R0RIYXFvZDBpTHdpcDZ3d1k5T0VWdEJhZmtkUVRGaTNNM3gvY2dsK0FDWVQ5WG50TlJycnRtWFRTUGZ4MG54R1lVc0NWUnNKY3h5Q0JxSHBlRGVRekpSTlFxVldMNGpJU3JCZkFRcEpYMnJoT1o4OXM1V3VLaW5PWFd0YUZncTRnd2t1VzR0ZkJJZzVUMjFlaXpGNEpWNzlMcXFXSDZoY3N0Z1huYzZYWTJvZjRvaytlYnJWOFBmR1lOU1NxRWQ4VFpqUzlBL0h0clJGNThEbUdaL2Z2Nmp5MjJjS01hUWllK1ZqdFZ4OUJyblJjWThYYTgxWmNTWlF4YlFLbFQ3MC9tRk5aQlN4ZXNLTWVTU24vV2hycEs0OU80ZW4zRkZJVTJqd2lLcGwybHpHMk0vdThJTzRZSlNCL1B6aVp4cGczcVk5Z25PRHNQR2lDZGNyejErcTVhYUdoMDdXUGlISFg5K1VpbVJjRThZS1BBNXUwNTBkQ2l2eVM2a2VhZkpFalQ0UXkxcElPaFRUd3ZrMWxrR0ZmeWp3bVBqL3JMRGY4YUc3ZXZlVWQveGwxbzlKMnh5ckhvQW9heTNVNVpHYjFCZGJ2OGFGNHJLb2wwTkorUlZBSTZJSHJCUnE4OGxJeGtzSlFxTm9GQ3o5b051N011OTVkMUJpZ3ErNjZiYzBuTWcyWXZYQXdaMkh3RjAzS0xRWEFWYjZVekZ1Lzc0MjYraElNUlR1M01mZDZoa01vMllMVzlxSS9odlBsaWg4RG5qaUFTUG9Fbkx2cVFidVpXaVBnQ3h2c1F4eXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==";
    const SHARE_BACKUP: &str = "ME-RS-AesGcm-K3vx+e6IF6BOUJ2DemvsdflQq2CbolcFqdazfapauZTHdY/Hovh5zC8s5Qmfb2tRmRaluRX1gxMZfGDP52rakFnZpOzOCNGyHiI/dsiFDFbty0fEheEw+p1LrOI4zNwy7NE7ZsK0C756ggVfrhCin2Yw0KA6pALFqfWnQokx5Q43pUFd6ZGD8fwathC4NGx/hVTi9lxA2L6ScNQY9V3bEie40MKdpLQ6ELsPq+38UVJtqIgE0wJs8fDKSIGJVEPvP6wbVa+oPB/uFl5h56YeuYB2UGHdMJ54DCEoUBSd5QGoeKwjIylrZ+wXzchPXhtAfaCmqlf0fmKi9f5FQGrFwH9drf5HFE5Z/JWQC1FMKJTeBZ2CgcFvCtHuVm8VNnhhes1fUc7gL8VNOqE25LHFFQp3fBfeHXRkCmX+PAU+1N8KU6SFX0XqDr5anKAMH6thViBdno2m6K9tzqyucUnfgHYgp/cc+XXo9Ffw7v6lVTW3ls9diZwdwcs9JYqoKhWAs9dVGPz0017glpeAz01moJDPSMkhZwQh9GGWvhyeTWE9T28NS1G3cOBkW0GbgmIDjKeDDXAOjDyN7Db0FFL3TRAXthFtRXjJyZD1Xu2quYyjz1ZG70ILp0rDzzDaikUPUt1TCsAz+8NfLwHKz+H4oPUGprdUqgBVSGOySH+lKZaUbN17qIXjEKg58jh686s6i4GTD7Ndf6Xqsdc00PRDlm+jHwK7bNvkqkcChQHockIaIi4ETHCz/jqrca7uY8RIABv9Ni46+Ix1CrNY4qCUhep9oYZBGSLy2fQWWNk2nZgbrkipwUbgoV1IJV/kWCQ6ycjGG005kv3AFb6sZyrnFbvT7sa/JCKlo8gcVtzXlrJJqiO/7Qb1nTfj9dLd+/4ihpmwpFwPmKHi6zrZjJ8FbaDGkXSg+a82RQqz/AsH10hBd/tSZeZ5chdwgxTouoGix99HZipTKXLiAqW7Mo0N93+atNb9EWeHPBfsVbwJ2shBT5030QrY2qQfhTb4GUl52vPQBvpjxCzjPlvCWzFMlO8wrCP1sJm5egEb0F6Fpa9H3blBdMcb2NuKJ2VfSQzuJrbLzirnX3X0Pbk93S2dE5vs/2xsL6fqV18EPkVXO1mQtqsM8sMF8o6G/PLILN268Ga7CwcCL3qnoaCvahN3sHbciy38UH6s5hRTDvV75nWDj4oIaByrYx+JdgSZ4sucAn/bEQJCDSTVQ3sYQbEJGxc+xImNWudEoxdCmKYZPDFhUEIfO8pQRHTX8ZHZST+m97kJMuPvWg49UlrGu2YE6KbkNBEz7cSoWOuWpbrNjv1I8XKf8Sd82dvRWn3ZDc/4GXXE5oscG8UHTlz3XIpWNNrpE+wmn+AvmU0+n5r4Nv0LOFrlqH8Z2DcfjGqAVJkQMWFriruEcsPOvRgvGUeUtjulxEwcqX/UVmE5871rx0C2aJhazTnLkzt9TDFTaAf7J7zkIkhvKx8AU2A=";
    const SHARE_RECOVERY: &str = "ME-RS-N-Mi0yLUNBSVFBaGdESXFBRWk3b296TFVtbzNscG1jZEIxMXFESnVHbUpRUGYxUk9nOG5WQVNkR1NSTE5YQk41VytwV1dUcWVNSnVOakN3ZVd4eFk0Ylh6Y0NWTlhzYWFLNmM5UEUxWHVFT2lNVW9BeHdQNkRtM3BCT29EVklHWlFYbXZxQk9FOVYyU3FCZTlsblVRcUZBak9SQUllQjFVcWEvdlJMbXN3VWNqZlNJN0pVQWgvL3ZKZVBvbUxGUVFYcjZVSUE5dnpsaDgxVVNjaXZHUXlnOVQydWRNS202RTdveXQvcVpGam1DdUlYSFlkR1FCdkxrK01oaEErVmh2NlM2a0FkSU5veWRGUlZTdXpnU25zT25wcUJxb21oRWZaNkdmb1dsaHM5UUFadXRmeUgzdkxRT0hQeXc1TEZLbUE3dnpOTTJmMkc2dGZaZGR1Q2dnT2gydmZCUnh1ZmJSdStHN2VGSGtLdnVoOW16ekQ3YUF2Z3BRbldKakdrai9paGcyR1EyQVlBWWkrM200SXR5V3J3Q1ZRNDZlUjJCRGpmZllQK3BOTFNnL2xKNlNmbUswd204R2cvL2ZpbVBHODF4alcrckdIQkczUTV4U1JnWnlUcmt0TEFBWlY0VndJOENzdTlmSUNlT0tTYm9UVTVrOFJoWnZRS0pUNnhGS1d1K0l3OTVoWUlUZUdmVGxLa1NtdW93WmVXcFI1TjIyQUEyWENaVWVqVVBLdHlQWUYxOVNGTjRjUDlvTURuUng3bkxkY3B6cGE3QmJWQm1jRGNhTENZVW1PeXJKcDQwK1hoekJHVmlxVnBJTS9qNTJJQTg1TSt1TDVtM0xNUk12UFc4cEliNkpVYVlKV0FXSldWV3JKdlpzUGJrdmh3T0NlOXo5VWJUTXE1WUJzOC9OcFJnN0F4L2lmSTJ5ZHdxbDRacXd4N29MM0ZrK0daK1FDeHRyTWJSK2oxTzhROXFXOEJ5eEcveXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==";
    const SHARE_PASSWORD: &str = "mnemonic share password";

    fn get_user_repo() -> (EncryptionPin, UserRepoT) {
        let mut repo = Box::new(UserRepoImpl::new(MemoryUserStorage::new())) as UserRepoT;
        let salt = EncryptionSalt::generate();
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let encrypted_password = Some(PASSWORD.encrypt(&pin, &salt).unwrap());
        repo.create(&crate::types::users::UserEntity {
            user_id: None,
            username: USERNAME.to_string(),
            encrypted_password,
            salt,
            is_kyc_verified: false,
            kyc_type: KycType::Undefined,
            viviswap_state: None,
            local_share: None,
            wallet_transactions: Vec::new(),
        })
        .unwrap();

        (pin, repo)
    }

    #[rstest]
    #[case(MNEMONIC, true)] // Valid mnemonic
    #[case("", false)] // Empty mnemonic
    #[case(MNEMONIC_INCORRECT, false)] // Incorrect mnemonic
    #[tokio::test]
    async fn test_create_wallet_from_mnemonic(#[case] mnemonic: &str, #[case] should_succeed: bool) {
        // Arrange
        let (config, _cleanup) = Config::new_test_with_cleanup();
        let mut manager = WalletManagerImpl::new(USERNAME);
        let (pin, mut repo) = get_user_repo();

        // Act
        let result = manager
            .create_wallet_from_existing_mnemonic(&config, &None, &mut repo, &pin, mnemonic)
            .await;

        // Assert
        if should_succeed {
            result.expect("should create wallet from valid mnemonic");
            assert!(manager.recovery_share.is_some(), "Wallet was successfully created");
        } else {
            result.expect_err("should fail with invalid or empty mnemonic");
        }
    }

    #[rstest]
    #[case(&BACKUP_PASSWORD, Ok(()))]
    #[case(&PASSWORD, Err(WalletError::KdbxStorage(KdbxStorageError::UnlockError(UnlockError::HmacInvalid))))]
    #[tokio::test]
    async fn test_backup_and_restore(#[case] password: &LazyLock<PlainPassword>, #[case] should_succeed: Result<()>) {
        // Arrange
        let (config, _cleanup) = Config::new_test_with_cleanup();
        let mut manager = WalletManagerImpl::new(USERNAME);
        let (pin, mut repo) = get_user_repo();

        // Create wallet
        manager
            .create_wallet_from_new_mnemonic(&config, &None, &mut repo, &pin)
            .await
            .expect("failed to create new wallet");

        // Create backup
        let backup = manager
            .create_wallet_backup(&config, &None, &mut repo, &pin, &BACKUP_PASSWORD)
            .await
            .expect("failed to create backup");

        // Backup restoration
        let restore_result = manager
            .create_wallet_from_backup(&config, &None, &mut repo, &pin, &backup, password)
            .await;

        // Assert
        match should_succeed {
            Ok(_) => restore_result.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(restore_result.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[tokio::test]
    async fn test_change_password() {
        //Arrange
        //Arrange
        let (mut config, _cleanup) = Config::new_test_with_cleanup();
        let mut manager = WalletManagerImpl::new(USERNAME);
        let (pin, mut repo) = get_user_repo();

        // create a wallet
        manager
            .create_wallet_from_new_mnemonic(&config, &None, &mut repo, &pin)
            .await
            .expect("should succeed to create new wallet");

        let new_password = PlainPassword::try_from_string("new_password").unwrap();
        manager
            .change_wallet_password(&config, &None, &mut repo, &pin, &new_password)
            .await
            .expect("should succeed to change wallet password");

        let wallet = manager
            .try_get(
                &mut config,
                &None,
                &mut repo,
                example_api_network(String::from("IOTA")),
                &pin,
            )
            .await
            .expect("should succeed to get wallet after password change");

        wallet.get_address().await.expect("wallet should return an address");
    }

    #[tokio::test]
    async fn delete_wallet_removes_files() {
        //Arrange
        let (mut config, _cleanup) = Config::new_test_with_cleanup();
        let mut manager = WalletManagerImpl::new(USERNAME);
        let (pin, mut repo) = get_user_repo();

        // create a wallet
        manager
            .create_wallet_from_new_mnemonic(&config, &None, &mut repo, &pin)
            .await
            .expect("should succeed to create new wallet");

        // get the wallet instance to make sure any files are created
        let _wallet = manager
            .try_get(
                &mut config,
                &None,
                &mut repo,
                example_api_network(String::from("IOTA")),
                &pin,
            )
            .await
            .expect("should succeed to get wallet");

        let file_count_before = walkdir::WalkDir::new(&config.path_prefix).into_iter().count();

        //Act
        manager
            .delete_wallet(&config, &None, &mut repo)
            .await
            .expect("should delete wallet");

        // Assert
        let file_count_after = walkdir::WalkDir::new(&config.path_prefix).into_iter().count();
        assert!(file_count_after < file_count_before, "should remove files");
    }

    // Note: all test cases assume that there is no password stored in the user database (since a wallet was never created before)
    #[rstest::rstest]
    // ############### Test cases without the local storage available ###############
    // nothing available at all
    #[case(
        None,
        None,
        None,
        None,
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::UseMnemonic))
    )]
    // only recovery share available
    #[case(
        None,
        None,
        Some(SHARE_RECOVERY),
        None,
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::UseMnemonic))
    )]
    // only backup share available
    #[case(
        None,
        None,
        None,
        Some(SHARE_RECOVERY),
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::SetRecoveryShare))
    )]
    // local recovery and backup share available, but no password
    #[case(
        None,
        Some(SHARE_RECOVERY),
        None,
        Some(SHARE_BACKUP),
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))
    )]
    // local recovery and backup share available, and password
    #[case(
        None,
        Some(SHARE_RECOVERY),
        None,
        Some(SHARE_BACKUP),
        Some(SHARE_PASSWORD),
        Ok(Status {local: false, recovery: Some(RecoveryUsed::Local), backup: true })
    )]
    // recovery and backup available but no password provided
    #[case(
        None,
        None,
        Some(SHARE_RECOVERY),
        Some(SHARE_BACKUP),
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))
    )]
    // recovery and backup available and password provided
    #[case(
        None,
        None,
        Some(SHARE_RECOVERY),
        Some(SHARE_BACKUP),
        Some(SHARE_PASSWORD),
        Ok(Status{local: false, recovery: Some(RecoveryUsed::Remote), backup: true })
    )]
    // ############### Test cases with the local storage available ###############
    #[case(
        Some(SHARE_LOCAL),
        None,
        None,
        None,
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::SetRecoveryShare))
    )]
    #[case(Some(SHARE_LOCAL), Some(SHARE_RECOVERY), None, None, None, Ok(Status{local: true, recovery: Some(RecoveryUsed::Local), backup: false }))]
    #[case(Some(SHARE_LOCAL), None, Some(SHARE_RECOVERY), None, None, Ok(Status{local: true, recovery: Some(RecoveryUsed::Remote), backup: false}))]
    #[case(
        Some(SHARE_LOCAL),
        None,
        None,
        Some(SHARE_RECOVERY),
        None,
        Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))
    )]
    #[case(
        Some(SHARE_LOCAL),
        None,
        None,
        Some(SHARE_RECOVERY),
        Some(SHARE_PASSWORD),
        Ok(Status{local: true, recovery: None, backup: true})
    )]
    #[tokio::test]
    async fn test_resemble_shares(
        #[case] local_share: Option<&str>,
        #[case] local_recovery_share: Option<&str>,
        #[case] remote_recovery_share: Option<&str>,
        #[case] remote_backup_share: Option<&str>,
        #[case] password: Option<&str>,
        #[case] expected_result: Result<Status>,
    ) {
        use crate::{
            share::Share,
            wallet_manager::{WalletManager, WalletManagerImpl},
        };

        // setup the sdk
        let mut srv = mockito::Server::new_async().await;
        let url = format!("{}/api", srv.url());

        let m1_body = remote_recovery_share.map(|s| format!("{{ \"share\":\"{s}\" }}"));
        let m1 = srv
            .mock("GET", "/api/user/shares/recovery")
            .expect(if local_recovery_share.is_none() {
                1 + if expected_result.is_ok() { 1 } else { 0 } // if Ok we try to create wallet
            } else {
                0
            }) // do not download the recovery share if it exists locally
            .with_status(if remote_recovery_share.is_some() { 200 } else { 404 })
            .with_header("content-type", "application/json")
            .with_body(m1_body.unwrap_or_default())
            .create();

        let m2_body = remote_backup_share.map(|s| format!("{{ \"share\":\"{s}\" }}"));
        let m2 = srv
            .mock("GET", "/api/user/shares/backup")
            .expect(
                // only expect a call if we don't have enough shares from the others
                if local_share.is_some() as usize
                    + local_recovery_share.is_some() as usize
                    + remote_recovery_share.is_some() as usize
                    >= 2
                {
                    0
                } else {
                    1 + if expected_result.is_ok() { 1 } else { 0 } // if Ok we try to create wallet
                },
            )
            .with_status(if remote_backup_share.is_some() { 200 } else { 404 })
            .with_header("content-type", "application/json")
            .with_body(m2_body.unwrap_or_default())
            .create();

        // Initialize your Sdk instance with necessary parameters

        let (mut config, _cleanup) = Config::new_test_with_cleanup_url(&url);

        let access_token = Some(AccessToken::try_from_string("a fake token").unwrap());
        let mut repo = MockUserRepo::new();

        let mut manager = WalletManagerImpl::new("share_user");

        // setup shares if provided

        let salt = EncryptionSalt::generate();
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let encrypted_password =
            password.map(|s| PlainPassword::try_from_string(s).unwrap().encrypt(&pin, &salt).unwrap());
        let user = crate::types::users::UserEntity {
            user_id: None,
            username: "share_user".to_string(),
            encrypted_password,
            salt,
            is_kyc_verified: true,
            kyc_type: KycType::Undefined,
            viviswap_state: None,
            local_share: local_share.map(|s| s.to_string()),
            wallet_transactions: Vec::new(),
        };

        repo.expect_get().returning(move |_| Ok(user.clone()));

        manager.recovery_share = local_recovery_share.map(|s| s.parse::<Share>().unwrap());

        // If the expected result is OK and there was no local share from the beginning, we expect
        // the local share to be set to a valid share.
        if expected_result.is_ok() && local_share.is_none() {
            repo.expect_set_local_share().returning(|_, _| Ok(()));
        }
        let mut repo = Box::new(repo) as UserRepoT;

        // Function you want to test
        let result = manager
            .try_resemble_shares(&config, &access_token, &mut repo, &pin)
            .await
            .map(|(_mnemonic, status)| status);

        // Assert
        match (&result, expected_result) {
            (Ok(s), Ok(s2)) => assert_eq!(s, &s2),
            (Err(WalletError::WalletNotInitialized(k)), Err(WalletError::WalletNotInitialized(k2))) => {
                assert_eq!(*k, k2)
            }
            (other, other2) => panic!("Expected {other2:?} but got {other:?}"),
        }

        // if the result is Ok, make sure we have access to a valid wallet
        if result.is_ok() {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let wallet = manager
                .try_get(
                    &mut config,
                    &access_token,
                    &mut repo,
                    example_api_network(String::from("IOTA")),
                    &pin,
                )
                .await
                .unwrap();

            let address = wallet.get_address().await.unwrap();
            let balance = wallet.get_balance().await.unwrap();
            println!("Recevier address: {address}, balance = {balance:?}");

            // This check is nice, but does not play nice with nextest running the tests in parallel
            // from the CI, commented out for now.
            // assert!(balance.base_coin() > 0);
        }

        m1.assert();
        m2.assert();
    }
}
