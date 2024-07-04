/** @type {import('steamworks.js')} */
const steamworks = require('steamworks.js');
const client = steamworks.init(480);

const playerName = client.localplayer.getName()
document.getElementById('name').innerText = playerName

document.getElementById('activateOverlay').addEventListener('click', function() {
    let avatar = client.localplayer.getMediumAvatar();
    let avatarL = client.localplayer.getLargeAvatar();

    console.log(avatarL);
})
