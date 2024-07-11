/** @type {import('steamworks.js')} */
const steamworks = require('steamworks.js');
const client = steamworks.init(3097530);

const playerName = client.localplayer.getName()
document.getElementById('name').innerText = playerName


let status = null;
let handle = null;

document.getElementById('3').addEventListener('click', function() {
    let items = client.inventory.generateTestItem([300]) // give the user a case item
})

let apiCall = 0;
document.getElementById('2').addEventListener('click', async function() {
    apiCall = client.inventory.requestInventoryItems();
    let result = client.inventory.getResultStatus(apiCall);
    while (result != 1) {
        await new Promise(r => setTimeout(r, 100));
        result = client.inventory.getResultStatus(apiCall);
    }

    let items = client.inventory.getResultItemsAll(apiCall);
    console.log(items);

    let caseID = items.find(item => item.mIDefinition == 200);
    if (!caseID) {
        return;
    }
    client.inventory.exchangeItems([300], [1], 1, [caseID.mItemId], [1], 1) // grant the item generator
    while (result != 1) {
        await new Promise(r => setTimeout(r, 100));
        result = client.inventory.getResultStatus(apiCall);
    }

    items = client.inventory.getResultItemsAll(apiCall);
    console.log(items);
})

document.getElementById('1').addEventListener('click', function() {
    let items = client.inventory.startPurchase([200], [1]) // give the user a case item
})
