const express = require('express')
const { Kafka } = require('kafkajs')
const app = express()
const port = 8080

class ConsumerService {
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

  async run(consumer) {
    await consumer.run({
      eachMessage: async ({ topic, partition, message }) => {
        this.observers.forEach((callback) => callback(message));
      },
    })
  }
}

// connecting to kafka 
const kafka = new Kafka({
  clientId: 'sparkling-app',
  brokers: ['kafka:9092',] // for inside cluster
  // brokers: ['localhost:9094',] // for outside cluster
})
const consumer = kafka.consumer({ groupId: 'sparkling-app' })
const consumerService = new ConsumerService()

app.use('/', express.static('public'))

app.get('/realtime', async (req, res) => {
  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Content-Type', 'text/event-stream');
  // res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Connection', 'keep-alive');
  res.flushHeaders(); // flush the headers to establish SSE with client

  let onMessage = (message) => {
    res.write('data: '+ message.value.toString() + '\n\n');
  }
  consumerService.subscribe(onMessage);

  // If client closes connection, stop sending events
  res.on('close', () => {
      consumerService.unsubscribe(onMessage);
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
  await consumer.connect();
  await consumer.subscribe({topic: "realtime_with_metadata", fromBeginning: false});
  consumerService.run(consumer)
  app.listen(port, () => {
    console.log(`Example app listening on port ${port}`)
  })
}
startup()
