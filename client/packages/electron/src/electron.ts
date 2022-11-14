import { app, BrowserWindow, ipcMain } from 'electron';
import dnssd from 'dnssd';
import { IPC_MESSAGES } from './shared';
import thisIp from 'ip';
import {
  FrontEndHost,
  frontEndHostUrl,
  isProtocol,
} from '@openmsupply-client/common/src/hooks/useNativeClient';
const SERVICE_TYPE = 'omsupply';
const PROTOCOL_KEY = 'protocol';
const CLIENT_VERSION_KEY = 'client_version';
const HARDWARE_ID_KEY = 'hardware_id';

const discovery = new dnssd.Browser(dnssd.tcp(SERVICE_TYPE));

let connectedServer: FrontEndHost | null = null;
let discoveredServers: FrontEndHost[] = [];

// This allows TypeScript to pick up the magic constant that's auto-generated by Forge's Webpack
// plugin that tells the Electron app where to look for the Webpack-bundled app code (depending on
// whether you're running in development or production).
declare const MAIN_WINDOW_WEBPACK_ENTRY: string;
declare const MAIN_WINDOW_PRELOAD_WEBPACK_ENTRY: string;

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require('electron-squirrel-startup')) {
  // eslint-disable-line global-require
  app.quit();
}

const connectToServer = (window: BrowserWindow, server: FrontEndHost) => {
  discovery.stop();
  const { ELECTRON_HOST } = process.env;
  const url =
    (typeof ELECTRON_HOST !== 'undefined' && ELECTRON_HOST) ||
    frontEndHostUrl(server);

  window.loadURL(url);

  connectedServer = server;
};

const start = (): void => {
  // Create the browser window.
  const window = new BrowserWindow({
    height: 768,
    width: 1024,
    webPreferences: {
      preload: MAIN_WINDOW_PRELOAD_WEBPACK_ENTRY,
    },
  });
  // and load the index.html of the app.
  window.loadURL(`${MAIN_WINDOW_WEBPACK_ENTRY}?autoconnect=true`);
  window.webContents.openDevTools();

  ipcMain.on(IPC_MESSAGES.START_SERVER_DISCOVERY, () => {
    discoveredServers = [];
    discovery.start();
  });

  ipcMain.on(IPC_MESSAGES.GO_BACK_TO_DISCOVERY, () => {
    // Just navigate, discovery UI requests discovery start
    window.loadURL(`${MAIN_WINDOW_WEBPACK_ENTRY}?autoconnect=false`);
  });

  ipcMain.on(IPC_MESSAGES.CONNECT_TO_SERVER, (_event, server: FrontEndHost) =>
    connectToServer(window, server)
  );

  ipcMain.handle(IPC_MESSAGES.CONNECTED_SERVER, async () => connectedServer);

  ipcMain.handle(IPC_MESSAGES.DISCOVERED_SERVERS, async () => {
    const servers = discoveredServers;
    discoveredServers = [];
    return servers;
  });

  discovery.on('serviceUp', function ({ type, port, addresses, txt }) {
    if (type?.name !== SERVICE_TYPE) return;
    if (typeof txt != 'object') return;
    const protocol = txt[PROTOCOL_KEY];
    const clientVersion = txt[CLIENT_VERSION_KEY];
    const hardwareId = txt[HARDWARE_ID_KEY];
    if (!isProtocol(protocol)) return;
    if (!(typeof clientVersion === 'string')) return;
    if (!(typeof hardwareId === 'string')) return;

    // TODO fiter our ipv6 addresses and just get one ipv4 (it's usually the first one)
    const ip = addresses[0];

    if (!ip) return;

    discoveredServers.push({
      port,
      protocol,
      ip,
      clientVersion: clientVersion || '',
      isLocal: ip === thisIp.address() || ip === '127.0.0.1',
      hardwareId,
    });
  });
};

app.on('ready', start);

app.on('window-all-closed', () => {
  app.quit();
});

app.on(
  'certificate-error',
  (event, _webContents, _url, _error, _certificate, callback) => {
    event.preventDefault();
    return callback(true);

    // TODO store an object with this shape: { [hardware_id + port]: cert }, retreive this object on startup
    // update/save this object when connecting to 'new' server that is not in the object
    // if server is in the object make sure cert matches

    //   if (!connectedServer) return callback(false);

    //   if (url.startsWith(frontEndHostUrl(connectedServer))) {
    //     event.preventDefault();
    //     callback(true);
    //   } else {
    //     callback(false);
    //   }
    // }
  }
);
