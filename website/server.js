const express = require('express')
const { Kafka } = require('kafkajs')
const app = express()
const port = 8080

// testing to receive kafka messages
const kafka = new Kafka({
  clientId: 'sparkling-app',
  brokers: ['kafka:9092',]
})
const consumer = kafka.consumer({ groupId: 'sparkling-app' })

// app.get('/', (req, res) => {
//   res.send('Hello World!')
// })
app.use('/', express.static('public'))

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})

app.get('/realtime', async (req, res) => {

  res.setHeader('Cache-Control', 'no-cache');
  res.setHeader('Content-Type', 'text/event-stream');
  res.setHeader('Access-Control-Allow-Origin', '*');
  res.setHeader('Connection', 'keep-alive');
  res.flushHeaders(); // flush the headers to establish SSE with client

  // connect to kafka broker and subscribe to topic
  await consumer.connect()
  await consumer.subscribe({ topic: 'realtime', fromBeginning: false })

  // stream updates to client
  await consumer.run({
    eachMessage: async ({ topic, partition, message }) => {
      res.write(message.value.toString())
    },
  })

  // If client closes connection, stop sending events
  res.on('close', () => {
      console.log('client dropped me');
      clearInterval(interValID);
      res.end();
  });
});
