use solana_geyser_plugin_interface::geyser_plugin_interface::GeyserPlugin;

mod plugin;
pub use plugin::SupabasePlugin;

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
///
/// The Solana validator and this plugin must be compiled with the same Rust compiler version and Solana core version.
/// Loading this plugin with mismatching versions is undefined behavior and will likely cause memory corruption.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin: Box<dyn GeyserPlugin> = Box::new(SupabasePlugin::default());
    Box::into_raw(plugin)
}