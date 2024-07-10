/** @type {import('steamworks.js')} */
const steamworks = require('steamworks.js');
const client = steamworks.init(3097530);

const playerName = client.localplayer.getName()
document.getElementById('name').innerText = playerName

let status = null;
let handle = null;

document.getElementById('1').addEventListener('click', function() {
    // handle = client.inventory.requestInventoryItems();
    // console.log(handle);
    let items = client.inventory.getItemsWithPrices()

    let steamID = client.localplayer.getSteamId();
    console.log(steamID);

    console.log(items);
})

let apiCall = 0;
document.getElementById('2').addEventListener('click', function() {
    apiCall = client.inventory.startPurchase([100], [3]);

    console.log(apiCall);
})

document.getElementById('3').addEventListener('click', function() {
    let result = client.utils.isApiCallCompleted(apiCall);

    console.log(result);
})

