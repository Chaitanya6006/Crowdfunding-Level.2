# 🚀 Crowdfunding DApp (Soroban + Stellar Testnet)

A simple Level-2 Crowdfunding decentralized application built using
Soroban smart contracts and React frontend.

---

## ✨ Features

- Freighter wallet connect
- Donate to campaign
- Multiple donors supported
- Refund available
- Total donation display
- Deployed on Stellar Testnet
- Clean React frontend

---

## 🛠 Tech Stack

- Rust (Soroban Smart Contract)
- Stellar Testnet
- React + Vite
- Freighter Wallet

---

## 📸 Screenshots

### Dashbord
![Dashbord](<img width="1919" height="951" alt="Screenshot 2026-03-02 195141" src="https://github.com/user-attachments/assets/9722b3d8-6baa-420d-8708-5ae6d33c0f09" />
)
### Wallet Connected

![Wallet Connected](<img width="1887" height="981" alt="image" src="https://github.com/user-attachments/assets/73bec882-6b3d-4ab4-aa2a-ec22a66d3974" />
)

### MultiWallet options
![MultiWalle ] (<img width="1891" height="865" alt="Screenshot 2026-03-02 193837" src="https://github.com/user-attachments/assets/6e2fee77-bfb8-43eb-8d66-ec488370046b" />
)

### Donate Screen

![Donate](<img width="1867" height="943" alt="image" src="https://github.com/user-attachments/assets/1828177a-c149-4edc-94e8-889ccf0d82e3" />
)

### withdraw screen 

![withdraw] (<img width="1886" height="753" alt="Screenshot 2026-03-02 193924" src="https://github.com/user-attachments/assets/545de76b-ccaa-40c9-a468-8af18595b0d2" />
)

### Refund screen

![Refund] (<img width="1899" height="940" alt="Screenshot 2026-03-02 194407" src="https://github.com/user-attachments/assets/27e1a6c8-dd06-4fa3-a993-aafc21237455" />
)

### Total Donation

![Total Donation](<img width="1866" height="882" alt="image" src="https://github.com/user-attachments/assets/cbeba115-67ad-4e83-923c-8b2ef0e15364" />
)

---

## 📄 Smart Contract Functions

- initialize(goal, creator)
- donate(donor, amount)
- refund(donor)
- get_total()

---

## 🧪 How to run frontend locally

Install dependencies

```bash
npm install
```

Run frontend

```bash
npm run dev
```

# 🚀 Crowdfunding DApp – Level 2 (Soroban Smart Contract)

This repository contains the Level-2 project implementation of a Crowdfunding decentralized application using Soroban smart contracts on Stellar Testnet.

---

## 📌 Smart Contract – Deployment Status

This smart contract is deployed on Stellar Testnet using the Soroban CLI.

---

### 🌐 Network

Stellar Testnet

---

### 📦 WASM Hash

6d2167f59bfef544597887d370e6c9eae514ee34c10042a6411a82491b100ac7

---

### 🧾 Deployed Contract IDs

The same WASM file was deployed multiple times during testing.

#### ✅ Deployment #1

Contract ID  
CDUHZOQP5AVTDL6AAAN5ZX5SM52PGPJLQZVDZCOQZI7J4J5VM6HONUPC

Transaction  
https://stellar.expert/explorer/testnet/tx/875158f87a451dcca4782aa1d047ac9fc2eeabc84c79fa83346363455ee45f5a

---

#### ✅ Deployment #2

Contract ID  
CDWIOJYEFWU2S3ZX5EXLDSCJ4FTUZAFA3KO7RS453MWK5P5NZNOQZUK6

Transaction  
https://stellar.expert/explorer/testnet/tx/959a41e4b7a527f57d5600417f2f24fdea3314b8360f0662e3c56c071a00d8da

---

### 👤 Deployer Account

GDRNC2W7HFYMEWGRW3FC24N7OAZ2QQUBOEGWCZDRMQ4GMW7FVFRXA34Q

---

### 🛠️ Deployment Command

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/crowdfunding_contract.wasm \
  --source test \
  --network testnet

## 👤 Author

Chaitanya Chaudhari  
Project – Crowdfunding DApp using Soroban
