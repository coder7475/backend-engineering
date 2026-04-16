const amqp = require("amqplib");

const msg = { number: process.argv[2] };

connect();

async function connect() {
  try {
    const connection = await amqp.connect("amqp://localhost:5672");

    const channel = await connection.createChannel();
    // create a queue
    const result = await channel.assertQueue("jobs");
    
    channel.sendToQueue("jobs", Buffer.from(JSON.stringify(msg)));

    console.log("Job Sent Successfully: ", msg.number);

    setTimeout(function() {
        connection.close();
        process.exit(0)
    }, 500);
  } catch (e) {
    console.error(e)
  }
}
