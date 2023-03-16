
# API qui permet de récupérer la liste des produits pour chaque reseller

<div align="center"><a href="https://www.rust-lang.org" target="_blank"><img src="https://www.vectorlogo.zone/logos/rust-lang/rust-lang-icon.svg" height="200px" /></a></div>


## Installation nécessaire

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)
  
## Lancement de l'application

1. Créer un dossier firebase-token et un fichier firebase-key.json à l'interieur, récupérer le contenu de firebase-key.json sur l'interface web firebase : <br />

    ```bash 
    mkdir firebase-token && cd firebase-token && touch firebase-key.json
    ```
2. Lancer l'application
    ```bash 
    cargo run
    ```

## Récupérer un token firebase pour pouvoir tester l'api en prod (pour passer la gateway)

L'api key se récupère sur le site firebase

```bash 
curl -X POST \  'https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword?key=$API_KEY' \
    -H 'content-type: application/json' \
    -d '{ "email":"test@test.com", "password":"testeuh", "returnSecureToken":true }'
```

## Lancement des tests

Utilise l'émulateur firestore
```bash 
export FIRESTORE_EMULATOR_HOST=localhost:8080
```

Run les tests avec la commande basic
```bash 
firebase emulators:exec --project mspr-epsi-coffee 'cargo test'
```

Permet de run les tests et d'avoir le code coverage
```bash 
firebase emulators:exec --project mspr-epsi-coffee 'cargo tarpaulin --out Xml --output-dir coverage --fail-under 50'
```
