# Generating EC keys

## Private key
```
openssl ecparam -genkey -noout -name prime256v1 | openssl pkcs8 -topk8 -nocrypt -out delivery_private_key.pem
```

## Public key

```
openssl ec -in delivery_private_key.pem -pubout -out delivery_public_key.pem
```
