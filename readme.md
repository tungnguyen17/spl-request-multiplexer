# Request Multiplexer for Solana

## About

When working with some solana program, they only allow one address to be the authority. However, in fact there are cases that the authority right needed to be shared with many people. For example, a testing team to share the authority of a token mint with all of the members. There are some ways to mitigate this like: sharing the private key, or use a multisig program.

This program is intended to use as delegator for multiple people sharing a signer with their own private key. Using for production is not recommended in most cases because just 1 unethic member of the group can take-over the authority without any obstable or challenge.

## Features

This program is designed as a lite version of a Multisig program. Creating request, voting and voting threshold are stripped out for simplicity. So any member of a group will have the same priviledge.

- Anyone can create a group with unrestricted amount of members.

- Group owner can add/remove members of the group, and can transfer ownership of the group to another address.

- Members can execute the request directly.
