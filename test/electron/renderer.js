/** @type {import('steamworks.js')} */
const steamworks = require('steamworks.js');
const client = steamworks.init(3097530);

const playerName = client.localplayer.getName()
document.getElementById('name').innerText = playerName

let status = null;
let handle = null;

document.getElementById('1').addEventListener('click', function() {
    handle = client.inventory.requestInventoryItems();
    console.log(handle);
})

document.getElementById('2').addEventListener('click', function() {
    let item_count = client.inventory.getResultItemsCount(handle);

    console.log(item_count);

    let items = client.inventory.getResultItemsNCount(handle, 1);
    console.log(items);
})

document.getElementById('3').addEventListener('click', function() {
    let items = client.inventory.getResultItemsAll(handle);
    console.log(items);
})

