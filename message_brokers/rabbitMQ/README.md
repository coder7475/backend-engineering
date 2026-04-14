# RabbitMQ

A lightweight, feature-rich message broker implementing the AMQP protocol.

## Overview

RabbitMQ is an open-source message broker software that implements Advanced Message Queuing Protocol (AMQP). It provides a central platform for handling messages between producers and consumers.

## Key Features

- **Message Queuing** - Reliable message delivery with acknowledgments
- **Exchange Types** - Direct, topic, fanout, and headers exchanges
- **Routing** - Flexible message routing based on keys and patterns
- **Clustering** - High availability through clustering
- **Management UI** - Built-in management dashboard

## Connection

- **Default Port**: 5672
- **Management UI Port**: 15672
- **Default Credentials**: guest/guest (localhost only)

## Quick Start

```bash
# Start RabbitMQ server
rabbitmq-server

# Or enable management plugin
rabbitmq-plugins enable rabbitmq_management
```

## Resources

- [RabbitMQ Documentation](https://www.rabbitmq.com/docs)
- [RabbitMQ Tutorials](https://www.rabbitmq.com/tutorials)

