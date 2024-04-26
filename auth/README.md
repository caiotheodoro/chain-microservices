
## Test gRPC interfaces:


```
buf generate
```

```bash
grpcui -plaintext '[::1]:50051' 
```
Test message service:

```bash
grpcurl -plaintext -import-path ./proto -proto message.proto -H 'x-authorization: <>' -d '{"message": "Test" }' '[::1]:50051' message.Messaging/Message
```



```bash
grpcurl -plaintext -import-path ./proto -proto client.proto  '[::1]:50051' client.Client/ListUsers
```