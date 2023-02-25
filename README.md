
Setup Firebase : 
- create a project
- Authentification / Add email/password auth

- create Firestore database / test mode / europe
- rules / delete : 
    : if
  request.time < timestamp.date(2023, 3, 17);
    
Get Firebase Token : 
curl -X POST \  'https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword?key=$API_KEY' \
  -H 'content-type: application/json' \
  -d '{ "email":"test@test.com", "password":"testeuh", "returnSecureToken":true }'

Add secret to Github Repo : 
GCP_SERVICE_ACCOUNT_KEY, GCP_PROJECT_ID and FIREBASE_ADMIN_KEY

lock branch main


Test firestore : 
curl -sL https://firebase.tools | bash
firebase init
firebase emulators:exec 'cargo test'


cargo run --bin erp-import -- --file-path import/import-products.json --firebase-id mspr-epsi-coffee --firebase-token firebase-token/firebase-adminsdk-sa.json







