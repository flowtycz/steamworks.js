use napi::bindgen_prelude::BigInt;
use napi_derive::napi;
use steamworks_sys::{uint64, SteamItemDef_t, SteamItemDetails_t};


#[napi(object)]
pub struct SteamItemWithPrice {
    pub i_definition: i32,
    pub price: BigInt,
}
impl SteamItemWithPrice {
    pub(crate) fn from_callback(i_definition: SteamItemDef_t, item_price: uint64) -> Self {
        Self {
            i_definition,
            price: BigInt::from(item_price),
        }
    }
}

#[derive(Debug)]
#[napi(object)]
pub struct SteamItemDetails {
    pub m_item_id: BigInt,
    pub m_i_definition: i32,
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

    
    use napi::bindgen_prelude::BigInt;
    use steamworks_sys::{ uint64, SteamAPI_ISteamInventory_DestroyResult, SteamAPI_ISteamInventory_ExchangeItems, SteamAPI_ISteamInventory_GenerateItems, SteamAPI_ISteamInventory_GetAllItems, SteamAPI_ISteamInventory_GetItemsWithPrices, SteamAPI_ISteamInventory_GetNumItemsWithPrices, SteamAPI_ISteamInventory_GetResultItems, SteamAPI_ISteamInventory_GetResultStatus, SteamAPI_ISteamInventory_RequestPrices, SteamAPI_ISteamInventory_StartPurchase, SteamAPI_SteamInventory_v003, SteamInventoryResult_t, SteamItemDef_t, SteamItemDetails_t};

    use super::{SteamItemDetails, SteamItemWithPrice};

    #[napi]
    pub fn request_inventory_items() -> i32 {
        unsafe {
            let mut handle: SteamInventoryResult_t = 0;
            let inventory = SteamAPI_SteamInventory_v003();

            SteamAPI_ISteamInventory_GetAllItems(inventory, &mut handle);

            return handle;
        }
    }

    #[napi]
    pub fn get_result_status(restult_handle: i32) -> i32 {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();
            
            let result = SteamAPI_ISteamInventory_GetResultStatus(inventory, restult_handle);
            return result as i32;
        }
    }

    #[napi]
    pub fn destroy_inventory_result(restult_handle: i32) {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();
            
            SteamAPI_ISteamInventory_DestroyResult(inventory, restult_handle);
        }
    }

    #[napi]
    pub fn generate_test_item(items: Vec<i32>) -> i32 {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();

            let mut result_handle:SteamInventoryResult_t = 0;
            
            SteamAPI_ISteamInventory_GenerateItems(inventory, &mut result_handle, items.as_ptr(), null_mut(), items.len().try_into().unwrap());

            return result_handle;
        }
    }

    #[napi]
    pub fn get_result_items_count(restult_handle: i32) -> u32 {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();

            let mut item_count: u32 = 0;
            
            SteamAPI_ISteamInventory_GetResultItems(inventory, restult_handle, null_mut(), &mut item_count);
            
            return item_count;
        }
    }

    #[napi]
    pub fn get_result_items_all(restult_handle: i32) -> Vec<SteamItemDetails> {
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

    #[napi]
    pub fn get_items_with_prices() -> Vec<SteamItemWithPrice>  {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();

            SteamAPI_ISteamInventory_RequestPrices(inventory);
            let item_count = SteamAPI_ISteamInventory_GetNumItemsWithPrices(inventory);

            let mut items: Vec<SteamItemDef_t> =  (0..item_count).map(|_| 0).collect();
            let mut prices: Vec<uint64> =  (0..item_count).map(|_| 0).collect();
            let mut base_prices: Vec<uint64> =  (0..item_count).map(|_| 0).collect();

            
            SteamAPI_ISteamInventory_GetItemsWithPrices(inventory, items.as_mut_ptr(), prices.as_mut_ptr(), base_prices.as_mut_ptr(), item_count);

            let result_items = items.iter().zip(prices).map(|(item, price)| SteamItemWithPrice::from_callback(*item, price)).collect();
            return result_items;
        }
    }

    #[napi]
    pub fn start_purchase(items: Vec<i32>, quantities: Vec<u32>) -> u64 {
        unsafe {
            let inventory = steamworks_sys::SteamAPI_SteamInventory_v003();
            let count:u32 = quantities.len().try_into().unwrap();

            let result = SteamAPI_ISteamInventory_StartPurchase(inventory, items.as_ptr() as *const _, quantities.as_ptr() as *const _, count);
            return result;
        }
    }

    #[napi]
    pub fn exchange_items(
        p_array_generate: Vec<i32>, 
        pun_array_generate_quantity: Vec<u32>, 
        un_array_generate_length: u32,
        p_array_destroy: Vec<BigInt>,
        pun_array_destroy_quantity: Vec<u32>, 
        un_array_destroy_length: u32) -> i32 {
        unsafe {
            let mut handle: SteamInventoryResult_t = 0;
            let inventory = SteamAPI_SteamInventory_v003();

            let array_destroy: Vec<u64> = p_array_destroy.iter().map(|item_id| item_id.get_u64().1).collect();

            SteamAPI_ISteamInventory_ExchangeItems(
                inventory, 
                &mut handle, 
                p_array_generate.as_ptr(), 
                pun_array_generate_quantity.as_ptr(), 
                un_array_generate_length, 
                array_destroy.as_ptr(), 
                pun_array_destroy_quantity.as_ptr(), 
                un_array_destroy_length
            );

            return handle;
        }
    }
}
