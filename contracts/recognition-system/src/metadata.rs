use crate::{
    datatype::{NFTError, NFTMetadata, RecognitionNFT},
    interfaces::MetadataOperations,
    RecognitionSystemContract, RecognitionSystemContractArgs, RecognitionSystemContractClient,
};
use soroban_sdk::{Address, Env, String, U256, Vec};

#[allow(dead_code)]
impl MetadataOperations for RecognitionSystemContract {
    fn new(
        env: &Env,
        organization: Address,
        title: String,
        date: String,
        task: String
    ) -> Result<NFTMetadata, NFTError> {
        let metadata = NFTMetadata {
            ev_org: organization,
            ev_title: title,
            ev_date: date,
            task,
        };
        Ok(metadata)
    }

    fn update_metadata(
        env: &Env,
        admin: Address,
        token_id: U256,
        organization: Address,
        title: String,
        date: String,
        task: String,
    ) -> Result<(), NFTError> {
        // Check that admin is authorized
        admin.require_auth();

        // Get the existing NFT
        let mut nft: RecognitionNFT = env
            .storage()
            .persistent()
            .get(&token_id)
            .ok_or(NFTError::IDInvalid)?;

        // Assign updated event fields
        nft.metadata.ev_title = title;
        nft.metadata.ev_date = date;
        nft.metadata.ev_org = organization;
        nft.metadata.task = task;

        env
            .storage()
            .persistent()
            .set(&token_id, nft);
        Ok(())
    }
}
