const express = require('express')
const redis = require('redis')
const app = express()
const port = 8080

class SubscriberService {
  constructor() {
    this.observers = []
  }

  subscribe(callback) {
    this.observers.push(callback);
    console.log("connected clients: " + this.observers.length);
    return this;
  }
 
  unsubscribe(callback) {
    this.observers = this.observers.filter((observer) => observer !== callback);
    console.log("connected clients: " + this.observers.length);
    return this;
  }

  async run(redisClient) {
    await redisClient.subscribe("realtime-with-metadata", (message, channel) => {
      this.observers.forEach((callback) => callback(message));
    })
  }
}

const subscriberService = new SubscriberService()

app.use('/', express.static('public'))

app.get('/realtime', async (req, res) => {
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Content-Type', 'text/event-stream');
  // res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Connection', 'keep-alive');
  res.flushHeaders(); // flush the headers to establish SSE with client

  let onMessage = (message) => {
    res.write('data: '+ message.toString() + '\n\n');
  }
  subscriberService.subscribe(onMessage);

  // If client closes connection, stop sending events
  res.on('close', () => {
      subscriberService.unsubscribe(onMessage);
      res.end();
  });
});

function sleep(ms) {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}

// wait a little until startup
async function startup() {
  await sleep(5000);
  let redisClient = await redis.createClient({
    url: "redis://sparkling-redis/"
  }).connect();
  subscriberService.run(redisClient);
  app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
  })
}
startup()
