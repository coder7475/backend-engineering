const amqp = require("amqplib");


connect();

async function connect() {
  try {
    const connection = await amqp.connect("amqp://localhost:5672");

    const channel = await connection.createChannel();
    // assert a queue
    const result = await channel.assertQueue("jobs");
   
    channel.consume("jobs", msg => {
      const gotMsg = msg.content.toString();
      console.log(JSON.parse(gotMsg));
    })

    console.log("waiting for message...")
    setTimeout(function() {
        connection.close();
        process.exit(0)
    }, 500);
  } catch (e) {
    console.error(e)
  }
}
