# ws_case_study

This project is a Web Socket case study using API Gateway (Web Socket), Lambda and DynamoDB.

![Infrastructure](https://docs.aws.amazon.com/images/apigateway/latest/developerguide/images/ws-chat-app.png)

We have one Lambda function that is responsible for handling the connection ($connect), disconnection ($disconnect) and message events (sendmessage). The connection and disconnection events are responsible for adding and removing the connection id from the DynamoDB table. The message event is responsible for sending the message to all connected clients.

To deploy you can use the AWS CLI or the AWS Console. Remember to change the default parameters values.