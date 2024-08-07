## 1. Fill in the Slido Poll about your goals for this course

Done

## 2. Discuss in your teams what a decentralised version of a game like monopoly would be like, if there was no software on a central server.Consider What are the essential pieces of functionality ? How could people cheat ? How could you prevent them from cheating? This is just a general discussion, there is no need to write any code or do any design.

You would need a key pieces:
* A decentralized ledger
* A consensus mechanism
* A gossip protocol
* A notion of state outside of token balances
* Verifiable randomness
* A notion of smart contracts or programmable obligations

State would have to track the board and who owns what property, perhaps through NFTs. A ledger would track fungible monopoly money, and verifiable randomness would have to dictate where a player's position changes on the board. Some type of smart contract mechanism would have to enforce payments when a player lands on a position.

Cheating - players would try to cheat by gaming the consensus mechanism or the state transition function. If randomness generators were predictable, that would also be an avenue for cheating as it could allow an adversary to understand where a player would land and attempt to front-run them by buying properties.

You could prevent this by slashing someone's stake if they attempted to game the state transition function or the consensus mechanism. Finally, if you could game the gossip protocol by either manipulating other nodes' view of reality (e.g. some man-in-the-middle type attack), dropping transactions, or refusing to advance the game state.

## 3. Do you feel that Central Bank Digital Currencies (CBDCs) are a move towards decentralisation? Will they help or hinder adoption of other cryptocurrencies?

I would say no because the typical notion of a CBDC is that the central authority would have censorship capabilities. While the validation design could still be technically decentralized, there would be no censorship resistance.

I have no strong sense of whether or not it would help adoption - my guess is yes some because it would familiarize more people with digital currencies.

## 4. Listen to the Zero knowledge episode about Solana

Done