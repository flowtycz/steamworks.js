const { app, BrowserWindow } = require('electron')
const steamworks = require('steamworks.js');
const client = steamworks.init(3097530);

function createWindow() {
    const mainWindow = new BrowserWindow({
        width: 800,
        height: 600,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false,
        }
    })

    // will be true when opened from steam big picture
    if (process.env.SteamTenfoot) {
        mainWindow.setFullScreen(true)
    } else {
        mainWindow.maximize()
    }
    
    mainWindow.webContents.openDevTools()
    mainWindow.loadFile('index.html')
}

app.whenReady().then(() => {
    createWindow()

    app.on('activate', function () {
        if (BrowserWindow.getAllWindows().length === 0) createWindow()
    })
})

app.on('window-all-closed', function () {
    if (process.platform !== 'darwin') app.quit()
})

require('steamworks.js').electronEnableSteamOverlay()
