# Solana-hackathon2023

> ## Abstract
Bank Boss is designed at its core to be simplistic and user friendly with the capability to allow borrowers and lenders to leverage their assets
to create a decentralized platform to release equity from their assets within the crypto eco-system. Currently individuals have a lenghty process to exit the market and in many instances do not wish to
sell their positions for loss or for percieved loss. Bank Boss will allow them to lock their tokens at an agreed amount and make regular payments to recieve their 
assets back at the loan price. This can create a scenario whereby should the locked asset increase in value to a greater amount within the duration of the loan agreement the borrower can realize the original value and take advantage of the increase in asset price once the loan + interest has been paid in full.

> ## Team Members
- [Vadzim Mahilny](https://github.com/MIDAV0)
- [Karl Timmins](https://github.com/Karlitoyo)

> ## High-level overview of the project
<p align="center">
  <img width="900" height="600" src="solanahackathon-Page-1.drawio.png">
</p>
<!-- ![Alt text](solanahackathon-Page-1.drawio.png) -->

> ## Project implementation
Tech stack:
- [Anchor](https://project-serum.github.io/anchor/getting-started/introduction.html) - Solana smart contract framework
- [React](https://reactjs.org/) - Front end framework
- [Solana](https://solana.com/) - Blockchain

 > ## UX

The UX has been designed to be simple and with the aim of targeting the most all types of users from people with little to no previous experience in financial markets and lending, to seasoned defi users. The UX is created to easily encouraging users to provide liquidity and connect to the platform. 

## Users

The website has a target audience of both new entrants to the DeFi eco-system and existing users. The age 
profile is based on bank account users within our targeted demographic. Any individual with digital assets can make use of the service.

## Strategy

Initial strategy will be to target crypto users who currently require cash for their assests and migrate users from centralized exchanges. This will allow individuals to
retain their assets at the price agreed when taking the loan and to repay the loan to redeem their asset.

## Scope

The Scope of the service is to create financial literacy and encourage saving from a wider range of individuals.
This product can benefit individuals who rely on high interest loans or payday loans, this service can also benefit
current mortgage holders who are struggling to meet regular repayments or who may be in negative equity.


> ## Wireframes
### Landing Page:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/8b487bfd-904d-48a1-9d59-a2cffd238760">
</p>

### Lending Page:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/90321821-cd06-4ca1-9249-66d8301d8320">
</p>

### Lending Page selected:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/c52e4af0-8dc7-4852-9643-472f7f45164e">
</p>

### Borrow Page:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/e6700b43-2bad-448e-9f74-906ed325b7bc">
</p>

### Borrow Page selected:
<p align="center">
  <img width="600" height="600" src="https://github.com/MIDAV0/solana-hackathon2023/assets/30006896/e294dec8-fb13-4703-bf11-ef21a9d9a243">
</p>

 > ## Current-Features

## Development

We currently have a vault contract created which has different templates for accounts such as vault, loan and working logic for account interaction.

This allows users to create vaults, deposit, withdraw and borrow from the contract. We have utilized the Anchor Solana smart contract framework for development.

We have a Next.js frontend and Tailwind Daisy UI for styling.

## Further Development

We encountered challenges with the creation of PDA's (programme driven accounts). The specific issue we faced related to our knowledge of the Solana development enviornment and being able to accesss the required knowledge for our contract interactions with durable nonces. Our intention with the service is to make use of recurring payments and durable nonces enables this feature, however within the timeframe we ecountered challenges in implementing this feature.

We are continuing to devleop this product and we are confident with engagement and further dicsussion with Solana dev's we can achieve this. Many example projects within the Solana eco-system are not focused on our specific use case and as such this created speed bumps in our development.

> ## User-Stories

## First case example user experience

Middle income user who has difficulty meeting loan obligations and struggles with regular savings -

Individual who holds crypto assets requires stablecoin loan to make payment for an upcoming holiday. User feels their asset will rise in value in the short to
medium term. The current method for exiting funds out of the crypto eco-system requires a user to trade the asset on a centralized exhange for a choice of Ethereum/Bitcoin or another asset and this incurrs high fees through a crypto bank for cash and then to withdraw funds. Making use of Bank-Boss users can trade the token they hold for stablecoins at an agreed amount, term, and rate of interest. Withdraw the cash to bank and repay loan over term. During the term of the loan the value of the asset held as collateral increases 2x. The user repays the loan in full to the lender capital + interest and the asset is released at the original price that the borrower recieved the funds for.

## Second case example user experience

Low income user who is in arrears on their mortgage repayments and cannot maintain current levels of debt (however holds crypto asset) -

Borrower has digital asset to the value of €1100 which the user places against a loan as collateral for €1000 + 10% interest. User agrees term of 11 months @ €100 per month. The borrower makes 5 payments of €100. During this period the value of the asset drops to €600, the borrower
decides not to continue with the repayment of the loan. The asset + €500 paid in the 5 monthly installments revert to the lender.

> ## Instruction pipeline
![Alt text](solanahackathon-Instructions.drawio.png)

> ## SWOT

- Strengths

Personal finances applications market is an increasing market. Recent analysis by Statista -
(https://www.statista.com/outlook/298/109/personal-finance/united-states)
indicates market growth to over €1 Billion in value by 2023 in the United States alone. A growth of 25%
from its current position.

The system has numerous strengths which include simple user-friendly UI making use of blockchain technology
, dectralized finance and allowing users to leverage their assets.

- Weaknesses (Challenges)

Challenges identified predominantly relate to smart contract development, further challenges relate to the volatility of the crypto market and asset fluctuations. Our appraoch to mititgate this is to accept assets that do not fluctuate wildly in our first iteration of the service.

- Opportunities

Similiar models are in practice currently however make use of differing strategies to lend. Lending within the crypto eco-system is not fully adopted however this is an area within the space which will become more prevalent as the industry matures.

- Threats

Threats relate to similar products. However this is an emerging space within the crypto industry. Threats also relate to the value of
assets reducing leaving investor open to losses, similar to impairement loss currently faced through providing liquidity. The interest cost being fixed within the original asset collatoral can mitigate this risk.

> ## How to run

`anchor build` - build smart contracts

`anchor test` - test contract

`anchor deploy` -  deploy

`yarn install`

Front end is in `frontend` folder (Next.js-app).
