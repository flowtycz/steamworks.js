use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use steamworks_sys::SteamItemDetails_t;

pub type SteamItemInstanceID = BigInt;
pub type SteamItemDef = i32;

#[derive(Debug)]
#[napi(object)]
pub struct SteamItemDetails {
    pub m_item_id: SteamItemInstanceID,
    pub m_i_definition: SteamItemDef,
    pub m_un_quantity: u16,
    pub m_un_flags: u16,
}

impl SteamItemDetails {
    pub(crate) fn from_callback(steamworks_item: SteamItemDetails_t) -> Self {
        Self {
            m_item_id: BigInt::from(steamworks_item.m_itemId),
            m_i_definition: steamworks_item.m_iDefinition,
            m_un_quantity: steamworks_item.m_unQuantity,
            m_un_flags: steamworks_item.m_unFlags,
        }
    }
}
#[napi]
pub mod inventory {
    use std::ptr::null_mut;

    use steamworks_sys::{ SteamAPI_ISteamInventory_DestroyResult, SteamAPI_ISteamInventory_GenerateItems, SteamAPI_ISteamInventory_GetAllItems, SteamAPI_ISteamInventory_GetResultItems, SteamAPI_ISteamInventory_GetResultStatus, SteamAPI_SteamInventory_v003, SteamInventoryResult_t, SteamItemDef_t, SteamItemDetails_t};

    use super::SteamItemDetails;

    #[napi]
    pub fn request_inventory_items() -> SteamInventoryResult_t {
        unsafe {
            let mut handle: SteamInventoryResult_t = 0;
            let inventory = SteamAPI_SteamInventory_v003();

            SteamAPI_ISteamInventory_GetAllItems(inventory, &mut handle);

            return handle;
        }
    }

    #[napi]
    pub fn get_result_status(restult_handle: SteamInventoryResult_t) -> i32 {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();
            
            let result = SteamAPI_ISteamInventory_GetResultStatus(inventory, restult_handle);
            return result as i32;
        }
    }

    #[napi]
    pub fn destroy_inventory_result(restult_handle: SteamInventoryResult_t) {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();
            
            SteamAPI_ISteamInventory_DestroyResult(inventory, restult_handle);
        }
    }

    #[napi]
    pub fn generate_test_item() -> SteamInventoryResult_t {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();

            let mut result_handle:SteamInventoryResult_t = 0;
            const ITEMS: [SteamItemDef_t; 1] = [100];
            let raw = &ITEMS as *const SteamItemDef_t;
            
            SteamAPI_ISteamInventory_GenerateItems(inventory, &mut result_handle, raw, null_mut(), 1);

            return result_handle;
        }
    }

    #[napi]
    pub fn get_result_items_count(restult_handle: SteamInventoryResult_t) -> u32 {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();

            let mut item_count: u32 = 0;
            
            SteamAPI_ISteamInventory_GetResultItems(inventory, restult_handle, core::ptr::null_mut(), &mut item_count);
            
            return item_count;
        }
    }

    #[napi]
    pub fn get_result_items_all(restult_handle: SteamInventoryResult_t) -> Vec<SteamItemDetails> {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();

            let mut item_count: u32 = get_result_items_count(restult_handle);
   
            if item_count == 0 {
                return Vec::new();
            }

            let mut items: Vec<SteamItemDetails_t> =  (0..item_count).map(|_| SteamItemDetails_t {
                m_iDefinition: 0,
                m_itemId: 0,
                m_unFlags: 0,
                m_unQuantity: 0,
            }).collect();

            SteamAPI_ISteamInventory_GetResultItems(inventory, restult_handle, items.as_mut_ptr(), &mut item_count);

            let return_items = items.iter().map(|item_def| SteamItemDetails::from_callback(*item_def)).collect();
            return return_items;
        }
    }
}
