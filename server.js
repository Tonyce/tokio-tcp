const net = require('net');

const connectMap = new Map();

net
  .createServer(socket => {
    const connectionId = genConnectionId();
    console.log(connectionId, 'connet');
    const handler = new SocketHandler(connectionId, socket);
    socket.on('connect', () => console.log('connect'));
    socket.on('ready', () => console.log('ready'));
    socket.on('data', chunk => {
      console.log(chunk.toString());
      const couldWrite = socket.write('hello world!\n');
      if (!couldWrite) {
        socket.pause();
      }
    });
    // socket.setTimeout(3000);
    // socket.on('timeout', () => {
    //   console.log('socket timeout');
    //   socket.end();
    // });
    socket.on('drain', function() {
      console.log(
        'write buffer is empty now .. u can resume the writable stream'
      );
      socket.resume();
    });
    socket
      // Emitted when an error occurs.
      // The 'close' event will be called directly following this event.
      .on('error', err => {
        if (err.code !== 'ECONNRESET') {
          console.log(connectionId, '------socket-------');
          console.error(err);
          console.log(connectionId, '------socket-------');
        } else {
          //   console.log(err.code);
        }
      })
      // Emitted once the socket is fully closed
      .on('close', () => endConnection('close'))
      // Emitted when the other end of the socket sends a FIN packet,
      // thus ending the readable side of the socket.
      .on('end', () => endConnection('end'));
    function endConnection(type) {
      console.log(connectionId, type);
      const { authed, connectionId: handlerConnectionId } = handler;
      if (authed && handlerConnectionId) {
        connectMap.delete(connectionId);
        handler = null;
      }
    }
  })
  .on('error', err => {
    console.log('------server-------');
    console.error(err);
    console.log('------server-------');
  })
  .listen(1337, () => {
    console.log('server start at 1337');
  });

function genConnectionId() {
  return Math.random()
    .toString(16)
    .substr(2);
}

class SocketHandler {
  constructor(connectionId, socket) {
    this.authed = false;
    this.connectionId = connectionId;
    this.socket = socket;
    this.ip = socket.remoteAddress;
    // this.socket.on('data', this.handleData);
    // this.socket.on('drain', () => {
    //   console.log('drain....');
    // });
    // this.socket.on('end', this.socketEnd);
    // this.authLimitTimer = setTimeout(() => {
    //   if (this.authed) {
    //     connectMap.set(connectionId, this);
    //   } else {
    // this.socket.end()
    // }
    // }, 5 * 1000);
  }

  handleData = chunk => {
    console.log(this.socket.bytesRead, chunk.length);
    this.socket.write('hi!\n');
    console.log(this.socket.bytesRead);
  };

  //   socketEnd = () => console.log('end');
}
