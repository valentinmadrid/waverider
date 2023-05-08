use postgrest::Postgrest;
use serde::Deserialize;
use serde_json::{Value, Map};
use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPluginError;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult,
};
use std::io::Cursor;
use std::{
    error::Error,
    fmt::{self, Debug},
    fs::OpenOptions,
    io::Read,
};
use tokio::runtime::Runtime;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};
use serde::{Serialize};

pub struct SupabasePlugin {
    postgres_client: Option<Postgrest>,
    configuration: Option<Configuration>,
    programs: Vec<[u8; 32]>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Configuration {
    pub supabase_url: String,
    pub supabase_key: String,
    pub programs: Option<Vec<String>>,
}

impl Configuration {
    pub fn load(config_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = OpenOptions::new().read(true).open(config_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str::<Configuration>(&contents)?)
    }
}

impl Default for SupabasePlugin {
    fn default() -> Self {
        SupabasePlugin {
            postgres_client: None,
            configuration: None,
            programs: Vec::new(),
        }
    }
}

impl GeyserPlugin for SupabasePlugin {
    fn name(&self) -> &'static str {
        "supabase-geyser"
    }

    fn on_load(&mut self, config_file: &str) -> PluginResult<()> {
        println!("config file: {}", config_file);
        let config = match Configuration::load(config_file) {
            Ok(c) => c,
            Err(_e) => {
                return Err(GeyserPluginError::ConfigFileReadError {
                    msg: String::from("Error opening, or reading config file"),
                });
            }
        };
        println!("Your supabase url: {:#?} ", &config.supabase_url);
        self.postgres_client = Some(
            Postgrest::new(&config.supabase_url).insert_header("apikey", &config.supabase_key),
        );

        match config.programs.as_ref() {
            Some(accounts) => {
                accounts.iter().for_each(|account| {
                    let mut acc_bytes = [0u8; 32];
                    acc_bytes.copy_from_slice(&bs58::decode(account).into_vec().unwrap()[0..32]);
                    self.programs.push(acc_bytes);
                });
            }
            None => (),
        }

        self.configuration = Some(config);
        Ok(())
    }

    fn on_unload(&mut self) {}

    fn update_account(
        &mut self,
        account: ReplicaAccountInfoVersions,
        _slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        let account_info = match account {
            ReplicaAccountInfoVersions::V0_0_1(_) => {
                return Err(GeyserPluginError::AccountsUpdateError {
                    msg: "V1 not supported, please upgrade your Solana CLI Version".to_string(),
                });
            }
            ReplicaAccountInfoVersions::V0_0_2(account_info) => account_info,
        };

        self.programs.iter().for_each(|program| {

            if program == account_info.owner {
                let account_pubkey = bs58::encode(account_info.pubkey).into_string();
                let account_owner = bs58::encode(account_info.owner).into_string();
                let account_data = account_info.data;
                let account_lamports = account_info.lamports;
                let account_executable = account_info.executable;
                let account_rent_epoch = account_info.rent_epoch;
                if account_data.len() > 9 {
      
                let mut data = &account_data[8..];
               
                let mut new_account = None;
                match NewAccount::deserialize(&mut data) {
                    Ok(account) => {
                        // Successfully deserialized `data` into a `NewAccount` instance.
                        // Set `new_account` to the created object.
                        new_account = Some(account);
                
                        // Do something with `new_account` here.
                    },
                    Err(error) => {
                        // An error occurred during deserialization.
                        // Set `new_account` to `None`.
                     
                
                        // Handle the error here.
                        println!("Error: {}", error);
                        // You can also choose to return an error or panic here, depending on your use case.
                    }
                }
                let mut testdata: &NewAccount = &NewAccount { age: 0, name: String::from(""), country: String::from("")};
                if let Some(unwrapped) = &new_account {
                    // This block will only execute if new_account is Some(value), and unwrapped will be a reference to the value
                    println!("Unwrapped value: {:?}", unwrapped);
                    testdata = unwrapped;
                }
                println!("Unwrapped value 2s: {:?}", testdata);
                
            let string = serde_json::to_string(&testdata).unwrap();
            let json_array = format!("[{}]", string);
let upsert_data = json_array.as_str();
                println!("Unwrapped value 2s: {:?}", upsert_data);
let base_account_string = serde_json::to_string(
    &serde_json::json!({ "account": account_pubkey, "owner": account_owner, "data": account_data, "executable": account_executable }) 
)
.unwrap();

println!("Unwrapped BASE ACCOUNT: {:?}", base_account_string);

let mut value1: Value = serde_json::from_str(base_account_string.as_str()).unwrap();
let value2: Value = serde_json::from_str(string.as_str()).unwrap();
let object1_map: &mut Map<String, Value> = value1.as_object_mut().unwrap();
for (key, value) in value2.as_object().unwrap().iter() {
    object1_map.insert(key.clone(), value.clone());
}
let string = serde_json::to_string(&value1).unwrap();
                let request_string = format!("[{}{}]", base_account_string, string);

                println!("Request string {:?}", string);
                
                let rt = Runtime::new().unwrap();
                // map the new_account data to json
            
                let result = rt.block_on(
                    self.postgres_client
                        .as_mut()
                        .unwrap()
                        .from("accounts")
                        .upsert(string)
                        .execute()
                );

                println!("Result: {:?}", result);
/* 
                let result2 = rt.block_on(
                    self.postgres_client
                        .as_mut()
                        .unwrap()
                        .from("accounts")
                        .upsert(string)
                        .execute()
                );

                println!("Result 2: {:?}", result2);
*/
            
            } else {
            }
        }
        });

        Ok(())
    }

    fn notify_end_of_startup(&mut self) -> PluginResult<()> {
        Ok(())
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        false
    }
}

impl Debug for SupabasePlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SupabasePlugin")
            .field("postgres_client", &self.postgres_client.is_some())
            .finish()
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Serialize)]
pub struct NewAccount {
    age: u32,
    name: String,
    country: String,
}
