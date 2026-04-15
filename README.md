# Student Registry DApp

## Description
Student Registry is a simple decentralized application (DApp) built using Soroban smart contracts on the Stellar network.This application allows users to store and manage student data on-chain.

## Features
- Add new student (NIM (Student ID) & Name)
- View all students
- Delete student by ID
- Data stored on blockchain (decentralized)

## Smart Contract
- Language: Rust
- Network: Stellar Testnet

## Contract ID
CB4VB6FPHKVBIA4SOD3L7AQEHNG6TKLB2DTYLWKNGD75OWPOXUN2OYQ2

## How to Use

### Add Student
soroban contract invoke
--id YOUR_CONTRACT_ID
--fn add_student
--arg nim="12345"
--arg nama="Budi"

### Get Students
soroban contract invoke
--id YOUR_CONTRACT_ID
--fn get_students

### Delete Student
--id YOUR_CONTRACT_ID
--fn delete_student
--arg id=123
