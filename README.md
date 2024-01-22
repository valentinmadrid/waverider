# Waverider 🌊
*Waverider is a Solana Geyser Plugin streaming account Data to a PostgREST Server*

## How does it work ?
At a high level, Waverider allows you to specify a set of programs, from which you want the accounts (PDA's) to be streamed to your Postgres database.

Waverider is divided into two plugins, one called "default" and the other "atlantic".
The difference between the two is that atlantic is also built to stream deserialized data to the database, while the default one only contains the raw data buffer.

### Use-cases

- Speed: Making a SQL Query to a Postgres database can be up to 10 times faster than using gPa (getProgramAccounts) calls.
- Realtime: We can benefit from Supabase's realtime infrastructure to stream Realtime account updates to dApp frontends.
- Filtering: With the atlantic plugin, we can directly filter on chain data with a SQL Query, instead of having to use gPa Filters.

### Demo

Walktrough of Waverider streaming deserialized account data to a Supabase instance.

https://user-images.githubusercontent.com/94240868/236879119-5b4f104d-9741-46bc-803a-2865e54a63cd.mp4

## Getting started

### Clone Waverider

```
git clone https://github.com/nautilus-project/waverider.git
cd waverider
```

### Setting up the default Plugin

1. __Build the Plugin__

```
yarn build-default
```

2. __Set up your database__

This plugin works with every PostgREST Server, although i recommend you set up a Supabase instance. They have a generous free tier, but can also be self hosted for completely free.

Take the SQL Script at [config/default.sql](https://github.com/nautilus-project/waverider/blob/main/config/default.sql) and run it in your database, either via the CLI or a GUI(Supabase web GUI).

If you are using Supabase, also make sure to turn off Row Level Security for the accounts table, you can do this via the UI.

3. __Fill the configuration file__

If you are a mac user, open `config/config.default.mac.json`. If you are on Windows or Linux, open `config/config.default.json`.
Fill out these fields:

```json
{
  // Your supabase REST API URL (starting with https//). Don't forget the /rest/v1 at the end
  "supabase_url": "<url>/rest/v1",
  // Your Supabase Annon Key
  "supabase_key": "<key>",
  // The programs you want to index accounts from
  "programs": ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"]
}
```

4. __Launch your validator with the plugin__

We'll use a local validator for this example, but this can be attached to a main/test/devnet validator.

⚠️ Important: You HAVE to run the Solana Toolsuite (CLI) with the same version as this scripts Plugin Interface(view in Cargo.toml), and need to run the Plugin with the same Rust version you have built your Solana Toolsuite.

For mac users, run: `solana-test-validator --geyser-plugin-config config/config.default.mac.json`.

For Linux/Windows users, run: `solana-test-validator --geyser-plugin-config config/config.default.json`.

### Running the atlantic Plugin

If you want deserialized data to be streamed to your accounts, it's going to take a little glass chewing.

First, you'll have to define your structs in the `plugin.rs` file. Then, get your accounts discriminators and add them as if statements into the code. The if statements should be like: `if the first 8 bytes of the Geyser plugin Interface account data are equal to your structs discriminator, run YourStruct::deserialize. Then specify the table name where you want this data to be written to.` Keep in mind that you will have to create the SQL schema yourself based on your Account Structs.

I'm going to specify more documentation to this topic soon. Feel free to contribute any ideas on how to make this process easier.
