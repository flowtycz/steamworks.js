use napi_derive::napi;

#[napi]
pub mod utils {
    use napi::bindgen_prelude::BigInt;
    use steamworks_sys::{SteamAPI_ISteamUtils_IsAPICallCompleted, SteamAPI_SteamUtils_v010};

    #[napi]
    pub fn get_app_id() -> u32 {
        let client = crate::client::get_client();
        client.utils().app_id().0
    }

    #[napi]
    pub fn get_server_real_time() -> u32 {
        let client = crate::client::get_client();
        client.utils().get_server_real_time()
    }

    #[napi]
    pub fn is_steam_running_on_steam_deck() -> bool {
        let client = crate::client::get_client();
        client.utils().is_steam_running_on_steam_deck()
    }

    #[napi]
    pub fn is_api_call_completed(call: BigInt) -> bool {
        unsafe {
            let api_call = call.get_u64().1;
            
            println!("Apicall: {api_call}");

            let utils = SteamAPI_SteamUtils_v010();

            let mut failed:bool = false;
            let result = SteamAPI_ISteamUtils_IsAPICallCompleted(utils, api_call, &mut failed);

            println!("Result: {failed}");

            return result;
        }
    }
}
