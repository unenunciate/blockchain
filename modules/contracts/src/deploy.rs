use super::*;

// This function creates a new ACTUS contract.
impl<T: Trait> Module<T> {
    pub fn deploy(attributes: Attributes) -> Result {
        // Getting the contract ID.
        let id = attributes.contract_id;

        // Checking if ID is available.
        if <Contracts<T>>::exists(id) {
            return Err("Contract ID already exists");
        }

        // Get current time.
        let t0 = Time::from_unix(<timestamp::Module<T>>::get().as_());

        // Calculating the initial contract state.
        let state;
        match attributes.contract_type {
            Some(ContractType::PAM) => {
                state = Self::deploy_pam(t0, attributes)?;
            }
            _ => {
                state = Err("Contract type not supported")?;
            }
        }

        // Storing the contract state.
        <Contracts<T>>::insert(id, state);

        // Return Ok if successful.
        Ok(())
    }
}
